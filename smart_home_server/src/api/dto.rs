use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoHouse {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoRoom {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DtoDevice {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoRoomRequest {
    pub room: DtoRoom,
    pub device: DtoDevice,
}
