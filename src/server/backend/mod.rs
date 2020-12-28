use std::net::Ipv4Addr;

pub mod memory;

pub trait StorageBackend: Sync + Send {
    fn store_ipv4(&self, key: String, ip: Ipv4Addr);

    fn retrieve_ipv4(&self, key: &String) -> Option<Ipv4Addr>;

    fn get_ipv4_size(&self) -> u32;
}
