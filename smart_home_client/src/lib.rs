use reqwest::{
    blocking::{get, Client},
    Result,
};
use serde::{de::DeserializeOwned, Serialize};
pub use smart_home_server::api::dto::*;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct SmartHomeHttpClient {
    addr: String,
}

impl SmartHomeHttpClient {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    fn make_json_request<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        content: Option<T>,
    ) -> Result<R> {
        match content {
            Some(body) => Client::builder()
                .build()?
                .post(format!("http://{}{}", self.addr, path))
                .json(&body)
                .send()?
                .error_for_status()?
                .json::<R>(),
            None => get(format!("http://{}{}", self.addr, path))?
                .error_for_status()?
                .json::<R>(),
        }
    }

    pub fn house(&self) -> Result<DtoHouse> {
        self.make_json_request::<DtoHouse, DtoHouse>("/house", None)
    }

    pub fn all_rooms(&self) -> Result<Vec<DtoRoom>> {
        self.make_json_request::<DtoRoom, Vec<DtoRoom>>("/rooms", None)
    }

    pub fn add_room(&self, room: DtoRoom) -> Result<DtoRoom> {
        self.make_json_request("/rooms/add", Some(room))
    }

    pub fn remove_room(&self, room: DtoRoom) -> Result<DtoRoom> {
        self.make_json_request("/rooms/delete", Some(room))
    }

    pub fn all_devices(&self, room: DtoRoom) -> Result<Vec<DtoDevice>> {
        self.make_json_request("/devices", Some(room))
    }

    pub fn add_device(&self, room: DtoRoom, device: DtoDevice) -> Result<DtoDevice> {
        self.make_json_request("/devices/add", Some(DtoRoomRequest { room, device }))
    }

    pub fn remove_device(&self, room: DtoRoom, device: DtoDevice) -> Result<DtoDevice> {
        self.make_json_request("/devices/remove", Some(DtoRoomRequest { room, device }))
    }

    pub fn report(&self) -> Result<String> {
        get(format!("http://{}/report", self.addr))?
            .error_for_status()?
            .text()
    }
}
