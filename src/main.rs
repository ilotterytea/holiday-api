#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod holiday;
mod routes;

use std::fs::{self, File};

use holiday::Holiday;

lazy_static! {
    static ref HOLIDAYS: Vec<Vec<Holiday>> = {
        let mut vec: Vec<Vec<Holiday>> = Vec::new();

        let paths = fs::read_dir("./data").unwrap();

        for path in paths {
            let file = File::open(path.unwrap().path()).unwrap();

            let _vec: Vec<Holiday> =
                serde_json::from_reader(file).expect("JSON was not well-formatted!");

            vec.push(_vec);
        }

        vec
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api/v1/",
            routes![
                routes::get_holidays_by_date,
                routes::get_todays_holiday_utc,
                routes::get_todays_holiday_timezone,
                routes::search_holidays
            ],
        )
        .mount("/", routes![routes::index])
        .attach(rocket_dyn_templates::Template::fairing())
}
