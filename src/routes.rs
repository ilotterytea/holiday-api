use crate::HOLIDAYS;
use chrono::Datelike;
use rocket::serde::json::Json;

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
