use super::storage::Storage;
use std::sync::{Mutex, MutexGuard};

pub type House = smart_home::House<Storage>;

pub struct SmartHomeController {
    house: Mutex<House>,
}

impl SmartHomeController {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            house: Mutex::new(House::new(name.as_ref().to_string(), Storage {})),
        }
    }

    pub fn acquire(&self) -> MutexGuard<House> {
        self.house.lock().unwrap()
    }
}
