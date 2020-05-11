use ini::Ini;
use std::net::{Ipv4Addr, Ipv6Addr};
use ipnetwork::{Ipv4Network, Ipv6Network};




pub struct WireguardConfig {
    ini: Ini,
    adresses: Vec<String>,
    public_key: String
    //backend: dyn Backend
}

impl WireguardConfig {

    pub fn new(ini_file: &str) -> WireguardConfig{
        let i: Ini = Ini::load_from_file(ini_file).unwrap();
        let section = i.section(Some("Interface")).unwrap();
        let conf = WireguardConfig {
            ini: i.clone(),
            adresses: section.get("Address").unwrap().split(',').map(|addr| addr.into() ).collect::<Vec<String>>().into(),
            public_key: "123456".into()
        };
        return conf;
    }

    pub fn get_ipv4(&self) -> Result<Ipv4Addr, String> {
        let mut err_result = "Ip not found".into();
        for addr in &self.adresses {
            println!("Address={}", addr.trim());
            let net_option: Result<Ipv4Network,_ > = addr.parse();
            match net_option {
                Ok(net) => {                    
                    println!("Network Size={}", net.size());
                    println!("IP={}", net.ip());
                    println!("2nd={}", net.nth(2).unwrap());
                    return  Ok(net.nth(2).unwrap())
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



trait StorageBackend {

    fn store_ipv4(&self, key: String, ip: Ipv4Addr);

    fn retrieve_ipv4(&self, key: String) -> Ipv4Addr;

    fn get_ipv4_size() -> u32;

}