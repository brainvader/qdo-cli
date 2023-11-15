use chrono::{offset::Utc, DateTime, Datelike};
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use uuid::Uuid;

pub fn quiz_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn gen_timestamp() -> (i32, u32, u32, String) {
    // generate time stamp from the current time
    let utc: DateTime<Utc> = Utc::now();
    let jst: DateTime<Tz> = utc.with_timezone(&Tokyo);
    let year = jst.year();
    let month = jst.month();
    let day = jst.day();
    let time = jst.time().format("%H%M%S").to_string();
    (year, month, day, time)
}
