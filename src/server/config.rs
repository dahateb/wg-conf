use crate::crypto::get_public_key;
use crate::server::backend::StorageBackend;
use crate::server::backend::memory::MemoryBackend;
use ini::{Ini, ParseOption, Properties, SectionEntry};
use ipnetwork::{Ipv4Network, Ipv6Network};
use std::net::{Ipv4Addr, Ipv6Addr};
use tokio::sync::Mutex;

pub struct WireguardConfig {
    ini: Mutex<Ini>,
    filename: String,
    adresses: Vec<String>,
    public_key: String,
    backend: Box<dyn StorageBackend>,
}

pub struct RegisterResult {
    pub ipv4_addr: Ipv4Addr,
    pub ipv6_addr: Option<Ipv6Addr>,
    pub public_key: String,
}

// @TODO add write to config here
impl WireguardConfig {
    pub fn new(ini_file: &str) -> WireguardConfig {
        let i: Ini = Ini::load_from_file_opt(
            ini_file,
            ParseOption {
                enabled_quote: true,
                enabled_escape: true,
            },
        )
        .map_err(|e| format!("Error loading ini file {} : {}", ini_file, e))
        .unwrap();
        let section = i.section(Some("Interface")).unwrap();
        let private_key = section.get("PrivateKey");
        let peers = i.section_all(Some("Peer"));
        let backend = MemoryBackend::new();

        for peer in peers {
            println!("{:?}", peer);
            let public_key = peer.get("PublicKey").unwrap();
            let allowed_ip: Result<Ipv4Network, _> = peer.get("AllowedIPs").unwrap().parse();
            backend.store_ipv4(public_key.into(), allowed_ip.unwrap().ip());
        }

        let conf = WireguardConfig {
            ini: Mutex::new(i.clone()),
            filename: String::from(ini_file),
            adresses: section
                .get("Address")
                .unwrap()
                .split(',')
                .map(|addr| addr.into())
                .collect::<Vec<String>>()
                .into(),
            public_key: get_public_key(private_key.unwrap()),
            backend: Box::new(backend),
        };
        println!("{:?}", conf.adresses);
        return conf;
    }

    pub async fn register(&self, key: String) -> Result<RegisterResult, String> {
        let result = RegisterResult {
            ipv4_addr: self.get_ipv4(key.clone())?,
            ipv6_addr: self.get_ipv6().ok(),
            public_key: self.public_key.clone(),
        };

        {
            let mut ini = self.ini.lock().await;

            //check if publickey is already registered
            let mut is_included = false;
            for peer in ini.section_all(Some("Peer")) {
                debug!("{:?}", peer);
                if peer.get("PublicKey").unwrap() == key {
                    is_included = true;
                }
            }
            let mut props = Properties::new();
            props.insert("PublicKey", key.clone());
            props.insert::<&str, String>("AllowedIPs", result.ipv4_addr.to_string() + "/32");
            match ini.entry(Some("Peer".into())) {
                SectionEntry::Vacant(vac) => {
                    vac.insert(props);
                }
                SectionEntry::Occupied(mut occ) => {
                    if !is_included {
                        occ.append_or_update(props.clone(), |properties| {
                            if properties.contains_key("PublicKey")
                                && properties.get("PublicKey").unwrap() == key
                            {
                                *properties = props.clone();
                                return true;
                            }
                            return false;
                        });
                    }
                }
            }
            ini.write_to_file(self.filename.clone()).unwrap()
        }

        Ok(result)
    }

    fn get_ipv4(&self, key: String) -> Result<Ipv4Addr, String> {
        let mut err_result = "Ip not found".into();
        if let Some(ip) = self.backend.retrieve_ipv4(&key) {
            return Ok(ip);
        }
        for addr in &self.adresses {
            println!("Local Address={}", addr.trim());
            let net_option: Result<Ipv4Network, _> = addr.parse();
            let ipv4_len = self.backend.get_ipv4_size();
            debug!("backend size: {}", ipv4_len);
            match net_option {
                Ok(net) => {
                    let next = net.nth(ipv4_len + 2).ok_or_else(|| {
                        format!("IPv4 address pool exhausted for network {}", addr.trim())
                    })?;
                    self.backend.store_ipv4(key, next);
                    debug!("Network Size={}", net.size());
                    debug!("IP={}", net.ip());
                    debug!("2nd={}", next);
                    return Ok(next);
                }
                Err(err) => err_result = format!("ip parse Error: {}", err),
            }
        }
        return Err(err_result);
    }

    fn get_ipv6(&self) -> Result<Ipv6Addr, String> {
        let mut err_result = "Ip not found".into();
        for addr in &self.adresses {
            debug!("Address={}", addr.trim());
            let net_option: Result<Ipv6Network, _> = addr.trim().parse();
            match net_option {
                Ok(net) => {
                    debug!("Network Size={}", net.size());
                    debug!("IP={}", net.ip());
                    debug!("2nd={}", net.iter().next().unwrap());
                    return Ok(net.iter().next().unwrap());
                }
                Err(err) => err_result = format!("ipv6 parse Error: {}", err),
            }
        }
        return Err(err_result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ipv4_assigns_new_ip() {
        let config = WireguardConfig::new("examples/conf/wg0.conf");
        let key = "test_public_key_1".to_string();

        let result = config.get_ipv4(key.clone());

        assert!(result.is_ok());
        let ip = result.unwrap();
        // First new IP should be 10.80.0.3 (base is .1, existing peer is .2, new is .3)
        assert_eq!(ip, Ipv4Addr::new(10, 80, 0, 3));
    }

    #[test]
    fn test_get_ipv4_returns_cached_ip_for_same_key() {
        let config = WireguardConfig::new("examples/conf/wg0.conf");
        let key = "test_public_key_2".to_string();

        let first_result = config.get_ipv4(key.clone());
        let second_result = config.get_ipv4(key.clone());

        assert!(first_result.is_ok());
        assert!(second_result.is_ok());
        // Same key should return the same IP
        assert_eq!(first_result.unwrap(), second_result.unwrap());
    }

    #[test]
    fn test_get_ipv4_assigns_sequential_ips_for_different_keys() {
        let config = WireguardConfig::new("examples/conf/wg0.conf");
        let key1 = "test_public_key_3".to_string();
        let key2 = "test_public_key_4".to_string();

        let result1 = config.get_ipv4(key1);
        let result2 = config.get_ipv4(key2);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        let ip1 = result1.unwrap();
        let ip2 = result2.unwrap();
        // IPs should be different and sequential
        assert_ne!(ip1, ip2);
    }

    #[test]
    fn test_get_ipv4_retrieves_existing_peer_ip() {
        let config = WireguardConfig::new("examples/conf/wg0.conf");
        // This is the public key from the existing peer in wg0.conf
        let existing_key = "nzZFP6R+abNfKjbknIf2QkqgyAf1PcmwpOCCelE7FDs=".to_string();

        let result = config.get_ipv4(existing_key);

        assert!(result.is_ok());
        // Should return the existing peer's IP
        assert_eq!(result.unwrap(), Ipv4Addr::new(10, 80, 0, 2));
    }

    #[test]
    fn test_get_ipv4_returns_error_when_pool_exhausted() {
        let config = WireguardConfig::new("examples/conf/wg0.conf");

        // For a /24 network (10.80.0.0/24), valid host IPs are .1 to .254
        // The network starts at .1 (server), existing peer at .2
        // nth(0) = .0 (network), nth(1) = .1 (server), nth(2) = .2, ..., nth(255) = .255 (broadcast)
        // nth(256) = None, which causes an error to be returned
        //
        // Pre-fill the backend to simulate 253 allocated IPs (indices 2..255)
        // so the next allocation attempt would be nth(255) = .255, then nth(256) = None
        for i in 2..255 {
            let key = format!("overflow_test_key_{}", i);
            config
                .backend
                .store_ipv4(key, Ipv4Addr::new(10, 80, 0, i as u8));
        }

        // At this point backend has 254 entries (1 existing + 253 added)
        // Next call will try nth(254 + 2) = nth(256) which is None -> error
        let key = "overflow_trigger_key".to_string();
        let result = config.get_ipv4(key);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("IPv4 address pool exhausted"));
    }
}
