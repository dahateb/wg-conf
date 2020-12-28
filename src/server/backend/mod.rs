use crate::crypto::get_public_key;
use ini::{Ini, ParseOption, Properties, SectionEntry};
use ipnetwork::{Ipv4Network, Ipv6Network};
use memory::MemoryBackend;
use std::net::{Ipv4Addr, Ipv6Addr};
//use std::sync::Mutex;
use tokio::sync::Mutex;

pub mod memory;

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
            let mut props = Properties::new();
            props.insert("PublicKey", key);
            props.insert::<&str, String>("AllowedIPs", result.ipv4_addr.to_string() + "/32");
            match ini.entry(Some("Peer".into())) {
                SectionEntry::Vacant(vac) => {
                    vac.insert(props);
                }
                SectionEntry::Occupied(mut occ) => {
                    occ.append(props);
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
            println!("backend size: {}", ipv4_len);
            match net_option {
                Ok(net) => {
                    let next = net.nth(ipv4_len + 2).unwrap();
                    self.backend.store_ipv4(key, next);
                    println!("Network Size={}", net.size());
                    println!("IP={}", net.ip());
                    println!("2nd={}", next);
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
            println!("Address={}", addr.trim());
            let net_option: Result<Ipv6Network, _> = addr.trim().parse();
            match net_option {
                Ok(net) => {
                    println!("Network Size={}", net.size());
                    println!("IP={}", net.ip());
                    println!("2nd={}", net.iter().next().unwrap());
                    return Ok(net.iter().next().unwrap());
                }
                Err(err) => err_result = format!("ipv6 parse Error: {}", err),
            }
        }
        return Err(err_result);
    }
}

pub trait StorageBackend: Sync + Send {
    fn store_ipv4(&self, key: String, ip: Ipv4Addr);

    fn retrieve_ipv4(&self, key: &String) -> Option<Ipv4Addr>;

    fn get_ipv4_size(&self) -> u32;
}
