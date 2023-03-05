use crate::{holiday::Holiday, HOLIDAYS};
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Timelike};
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/<month>/<day>")]
pub fn get_holidays_by_date(month: u32, day: u32) -> Result<Json<Vec<String>>, ()> {
    Ok(Json(
        HOLIDAYS
            .get(0)
            .unwrap()
            .iter()
            .filter(|p| p.date.0 == month && p.date.1 == day)
            .map(|p| p.name.clone())
            .collect::<Vec<String>>(),
    ))
}

#[get("/today")]
pub fn get_todays_holiday_utc() -> Result<Json<Vec<String>>, ()> {
    let now = chrono::Utc::now();
    let month = now.date_naive().month();
    let day = now.date_naive().day();

    Ok(Json(
        HOLIDAYS
            .get(0)
            .unwrap()
            .iter()
            .filter(|p| p.date.0 == month && p.date.1 == day)
            .map(|p| p.name.clone())
            .collect::<Vec<String>>(),
    ))
}

#[get("/today?<utc>")]
pub fn get_todays_holiday_timezone(utc: i64) -> Result<Json<Vec<String>>, ()> {
    if utc > 12 || utc < -12 {
        return Ok(Json(Vec::new()));
    }

    let now = chrono::Utc::now() + Duration::hours(utc);
    let month = now.date_naive().month();
    let day = now.date_naive().day();

    Ok(Json(
        HOLIDAYS
            .get(0)
            .unwrap()
            .iter()
            .filter(|p| p.date.0 == month && p.date.1 == day)
            .map(|p| p.name.clone())
            .collect::<Vec<String>>(),
    ))
}

#[get("/search?<q>")]
pub fn search_holidays(q: &str) -> Result<Json<Vec<Holiday>>, ()> {
    let holidays = HOLIDAYS.get(0).unwrap();
    let mut _hol: Vec<Holiday> = Vec::new();
    let reg = regex::Regex::new(format!(r"(?i){}", q).as_str()).unwrap();

    for holiday in holidays {
        if reg.is_match(holiday.name.as_str()) {
            _hol.push(holiday.clone());
        }
    }

    Ok(Json(_hol))
}
