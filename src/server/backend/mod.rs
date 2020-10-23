use crypto::get_public_key;
use ini::Ini;
use ipnetwork::{Ipv4Network, Ipv6Network};
use memory::MemoryBackend;
use std::net::{Ipv4Addr, Ipv6Addr};

pub mod crypto;
pub mod memory;

pub struct WireguardConfig {
    ini: Ini,
    adresses: Vec<String>,
    public_key: String,
    backend: Box<dyn StorageBackend>,
}

// @TODO add write to config here
impl WireguardConfig {
    pub fn new(ini_file: &str) -> WireguardConfig {
        let i: Ini = Ini::load_from_file(ini_file).unwrap();
        let section = i.section(Some("Interface")).unwrap();
        let private_key = section.get("PrivateKey");
        let peers = i.section_all(Some("Peer"));
        let backend = MemoryBackend::new();
        
        for peer in peers {
            println!("{:?}", peer);
            let public_key = peer.get("PublicKey").unwrap();
            let allowed_ip: Result<Ipv4Addr, _>  = peer.get("AllowedIPs").unwrap().parse();
            backend.store_ipv4(public_key.into(), allowed_ip.unwrap());
        }
                
        
        let conf = WireguardConfig {
            ini: i.clone(),
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
        return conf;
    }

    pub fn get_ipv4(&self, key: String) -> Result<Ipv4Addr, String> {
        let mut err_result = "Ip not found".into();
        if let Some(ip) = self.backend.retrieve_ipv4(&key) {
            return Ok(ip);
        }
        for addr in &self.adresses {
            println!("Address={}", addr.trim());
            let net_option: Result<Ipv4Network, _> = addr.parse();
            let ipv4_len = self.backend.get_ipv4_size();
            println!("backend size: {}", ipv4_len);
            match net_option {
                Ok(net) => {
                    let next = net.nth(ipv4_len + 1).unwrap();
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

    pub fn get_ipv6(&self) -> Result<Ipv6Addr, String> {
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
                Err(err) => err_result = format!("ip parse Error: {}", err),
            }
        }
        return Err(err_result);
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone()
    }
}

pub trait StorageBackend: Sync + Send {
    fn store_ipv4(&self, key: String, ip: Ipv4Addr);

    fn retrieve_ipv4(&self, key: &String) -> Option<Ipv4Addr>;

    fn get_ipv4_size(&self) -> u32;
}
