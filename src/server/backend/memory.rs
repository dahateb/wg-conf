use super::StorageBackend;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use std::sync::Mutex;

pub struct MemoryBackend {
    ipv4_storage: Arc<Mutex<HashMap<String, Ipv4Addr>>>,
}

impl MemoryBackend {
    pub fn new() -> MemoryBackend {
        MemoryBackend {
            ipv4_storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl StorageBackend for MemoryBackend {
    fn store_ipv4(&self, key: String, ip: Ipv4Addr) {
        let mut storage = self.ipv4_storage.lock().unwrap();
        storage.insert(key, ip);
        println!("Memory Size: {}", storage.len());
    }

    fn retrieve_ipv4(&self, key: &String) -> Option<Ipv4Addr> {
        self.ipv4_storage.lock().unwrap().get(key).copied()
    }

    fn get_ipv4_size(&self) -> u32 {
        self.ipv4_storage.lock().unwrap().len().try_into().unwrap()
    }
}
