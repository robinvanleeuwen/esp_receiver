use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Clockdata {
    pub id: i32,
    pub datetime: NaiveDateTime,
    pub value: String,
    pub comment: String,
}