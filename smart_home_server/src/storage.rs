use smart_home::{SmartHomeStorage, StorageError};

pub struct Storage {}

impl SmartHomeStorage for Storage {
    fn add_device(
        &mut self,
        _house: &str,
        _room_name: &str,
        _device_name: &str,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn add_room(&mut self, _house: &str, _room_name: &str) -> Result<(), StorageError> {
        Ok(())
    }

    fn get_device_status(
        &self,
        _house: &str,
        _room_name: &str,
        _device_name: &str,
    ) -> Result<String, StorageError> {
        Ok("Works fine".to_string())
    }

    fn remove_device(
        &mut self,
        _house: &str,
        _room_name: &str,
        _device_name: &str,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn remove_room(&mut self, _house: &str, _room_name: &str) -> Result<(), StorageError> {
        Ok(())
    }
}
