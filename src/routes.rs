use crate::HOLIDAYS;
use rocket::serde::json::Json;

#[get("/<month>/<day>")]
pub fn get_holidays_by_date(month: usize, day: usize) -> Result<Json<Vec<String>>, ()> {
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
