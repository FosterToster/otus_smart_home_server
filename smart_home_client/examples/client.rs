use std::vec;

use smart_home_client::SmartHomeHttpClient;
use smart_home_server::api::dto::*;

fn main() {
    //make a new http client
    let client = SmartHomeHttpClient::new("127.0.0.1:8080".to_string());

    //check if it works properly
    println!("House name: '{}'", client.house().unwrap().name);

    //mock rooms
    let rooms = vec![
        "Bedroom".to_string(),
        "Kitchen".to_string(),
        "Hall".to_string(),
    ];

    let devices = vec![
        "LightBulb".to_string(),
        "Thermometer".to_string(),
        "Socket".to_string(),
    ];

    //add some rooms
    rooms.iter().cloned().for_each(|room| {
        println!(
            "New room: '{}'",
            client.add_room(DtoRoom { name: room }).unwrap().name
        );
    });

    //add some devices to rooms
    rooms.iter().cloned().for_each(|room| {
        devices.iter().cloned().for_each(|device| {
            println!(
                "New device '{}' in '{}'  ",
                client
                    .add_device(DtoRoom { name: room.clone() }, DtoDevice { name: device })
                    .unwrap()
                    .name,
                room
            );
        })
    });

    //make a report over it all
    println!("{}", client.report().unwrap());
}
