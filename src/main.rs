#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod holiday;

use std::fs::{self, File};

use holiday::Holiday;

#[get("/")]
fn index() -> &'static str {
    "yo!"
}

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
    rocket::build().mount("/", routes![index])
}
