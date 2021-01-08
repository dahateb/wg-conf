use ini::Ini;
use ipnetwork::Ipv4Network;

use std::net::Ipv4Addr;
use url::Url;

pub fn build_config_file(
    address: &str,
    private_key: &str,
    peer_endpoint: &str,
    peer_public_key: &str,
    wg_port: &str,
    netmask: &str,
    config_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_file_name = config_file.unwrap_or("/etc/wireguard/wg0.conf");
    let ipv4_addr: Ipv4Addr = address.parse()?;
    let netmask_int = netmask.parse()?;
    let ipv4_network = Ipv4Network::new(ipv4_addr, netmask_int)?;
    let peer_url: Url = peer_endpoint.parse()?;
    let peer_host = peer_url.host().unwrap();
    let mut conf = Ini::new();
    conf.with_section(Some("Interface"))
        .set("Address", address)
        .set("PrivateKey", private_key);
    conf.with_section(Some("Peer"))
        .set("Endpoint", format!("{}:{}", peer_host, wg_port))
        .set("PublicKey", peer_public_key)
        .set("AllowedIps", format!("{}", ipv4_network));
    conf.write_to_file(config_file_name).unwrap();
    Ok(())
}
