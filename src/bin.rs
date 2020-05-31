extern crate esp_receiver;
extern crate diesel;

use self::esp_receiver::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {

    use esp_receiver::schema::clockdata::dsl::*;
    let connection = establish_connection();
    let result = clockdata
        .limit(5)
        .load::<Clockdata>(&connection)
        .expect("Error loading clockdata");




}