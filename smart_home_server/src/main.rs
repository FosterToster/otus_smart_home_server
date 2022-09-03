pub use actix_web::{web::Data, App, HttpServer};

use smart_home_server::api::{
    add_device, add_room, all_devices, house_name, house_rooms, remove_device, remove_room, report,
};

pub use smart_home_server::controller::SmartHomeController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let controller = Data::new(SmartHomeController::new("My home"));

    HttpServer::new(move || {
        App::new()
            .app_data(controller.clone())
            .service(house_name)
            .service(house_rooms)
            .service(add_room)
            .service(remove_room)
            .service(all_devices)
            .service(add_device)
            .service(remove_device)
            .service(report)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
