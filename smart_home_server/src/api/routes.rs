use crate::controller::SmartHomeController;
use actix_web::{get, post, web::Data, web::Json, Responder};
use serde::Serialize;

use super::dto::*;
use super::HttpSmartHomeError;

pub type Result<T> = std::result::Result<Json<T>, HttpSmartHomeError>;

fn do_ok<T: Serialize>(data: T) -> Result<T> {
    Ok(Json(data))
}

#[get("/house")]
pub async fn house_name(controller_state: Data<SmartHomeController>) -> impl Responder {
    let guard = controller_state.acquire();

    do_ok(DtoHouse {
        name: guard.name().to_string(),
    })
}

#[get("/rooms")]
pub async fn house_rooms(controller_state: Data<SmartHomeController>) -> impl Responder {
    let guard = controller_state.acquire();

    do_ok(
        guard
            .get_assigned_rooms()
            .iter()
            .cloned()
            .map(|room_name| DtoRoom { name: room_name })
            .collect::<Vec<DtoRoom>>(),
    )
}

#[post("/rooms/add")]
pub async fn add_room(
    controller_state: Data<SmartHomeController>,
    room: Json<DtoRoom>,
) -> impl Responder {
    let mut guard = controller_state.acquire();

    guard.add_room(&room.name).map(move |_| do_ok(room))?
}

#[post("/rooms/delete")]
pub async fn remove_room(
    controller_state: Data<SmartHomeController>,
    room: Json<DtoRoom>,
) -> impl Responder {
    let mut guard = controller_state.acquire();

    guard.remove_room(&room.name).map(move |_| do_ok(room))?
}

#[post("/devices")]
pub async fn all_devices(
    controller_state: Data<SmartHomeController>,
    room: Json<DtoRoom>,
) -> impl Responder {
    let guard = controller_state.acquire();

    let devices = guard.get_room_assigned_devices(&room.name)?;

    do_ok(
        devices
            .iter()
            .cloned()
            .map(|device| DtoDevice { name: device })
            .collect::<Vec<DtoDevice>>(),
    )
}

#[post("/devices/add")]
pub async fn add_device(
    controller_state: Data<SmartHomeController>,
    request: Json<DtoRoomRequest>,
) -> impl Responder {
    let mut guard = controller_state.acquire();

    guard
        .add_device(&request.room.name, &request.device.name)
        .map(move |_| do_ok(request.device.clone()))?
}

#[post("/devices/delete")]
pub async fn remove_device(
    controller_state: Data<SmartHomeController>,
    request: Json<DtoRoomRequest>,
) -> impl Responder {
    let mut guard = controller_state.acquire();

    guard
        .remove_device(&request.room.name, &request.device.name)
        .map(move |_| do_ok(request.device.clone()))?
}

#[get("/report")]
pub async fn report(
    controller_state: Data<SmartHomeController>,
) -> std::result::Result<String, HttpSmartHomeError> {
    let guard = controller_state.acquire();

    guard.get_report().map_err(HttpSmartHomeError::from)
}
