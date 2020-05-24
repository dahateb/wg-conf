use ini::Ini;
use std::net::{Ipv4Addr, Ipv6Addr};
use ipnetwork::{Ipv4Network, Ipv6Network};
use memory::MemoryBackend;
use std::sync::Arc;

pub mod memory;


pub struct WireguardConfig {
    ini: Ini,
    adresses: Vec<String>,
    public_key: String,
    backend: Arc<dyn StorageBackend>
}

impl WireguardConfig {

    pub fn new(ini_file: &str) -> WireguardConfig{
        let i: Ini = Ini::load_from_file(ini_file).unwrap();
        let section = i.section(Some("Interface")).unwrap();
        let backend = MemoryBackend::new();
        let conf = WireguardConfig {
            ini: i.clone(),
            adresses: section.get("Address").unwrap().split(',').map(|addr| addr.into() ).collect::<Vec<String>>().into(),
            public_key: "123456".into(),
            backend: Arc::new(backend)
        };
        return conf;
    }

    pub fn get_ipv4(&self, key: String) -> Result<Ipv4Addr, String> {
        let mut err_result = "Ip not found".into();
        for addr in &self.adresses {
            println!("Address={}", addr.trim());
            let net_option: Result<Ipv4Network,_ > = addr.parse();
            let ipv4_len = self.backend.get_ipv4_size();
            println!("backend size: {}", ipv4_len);
            match net_option {
                Ok(net) => {               
                    if let Some(ip) = self.backend.retrieve_ipv4(&key) {
                        return Ok(ip);
                    }
                    let next = net.nth(ipv4_len + 1).unwrap(); 
                    self.backend.store_ipv4(key, next);
                    println!("Network Size={}", net.size());
                    println!("IP={}", net.ip());
                    println!("2nd={}", next);
                    return  Ok(next)
                },
                Err(err) => err_result = format!("ip parse Error: {}", err)
            }
        }
        return Err(err_result);
    }

    pub fn get_ipv6(&self) -> Result<Ipv6Addr, String> {
        let mut err_result = "Ip not found".into();
        for addr in &self.adresses {
            println!("Address={}", addr.trim());
            let net_option: Result<Ipv6Network,_ > = addr.trim().parse();
            match net_option {
                Ok(net) => {                    
                    println!("Network Size={}", net.size());
                    println!("IP={}", net.ip());
                    println!("2nd={}", net.iter().next().unwrap());
                    return Ok(net.iter().next().unwrap())
                },
                Err(err) => err_result = format!("ip parse Error: {}", err)
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