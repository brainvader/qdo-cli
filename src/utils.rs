use anyhow::{Context, Ok, Result};
use chrono::{offset::Utc, DateTime, Datelike};
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use std::path::PathBuf;
use std::{env, fs};
use uuid::Uuid;

pub fn get_home_dir() -> Result<String> {
    let home_dir =
        env::var("HOME").with_context(|| "Failed to retrieve HOME environment variable")?;
    Ok(home_dir)
}

pub fn get_qdo_path() -> Result<PathBuf> {
    let home_dir = get_home_dir()?;
    let qdo_dir = PathBuf::from(home_dir).join("qdo");
    Ok(qdo_dir)
}

pub fn gen_qdo_dir(home_dir: &str) -> Result<()> {
    let qdo_dir = get_qdo_path().with_context(|| "Failed to generate a full path to qdo")?;

    // Check if the qdo directory exists
    if !qdo_dir.exists() {
        // Create the directory if it does not exist
        fs::create_dir(&qdo_dir)?;
        println!("ディレクトリを作成しました: {:?}", qdo_dir);
    } else {
        // Inform if the directory already exists
        println!("ディレクトリは既に存在します: {:?}", qdo_dir);
    }

    Ok(())
}

pub fn get_quiz_path() -> Result<PathBuf> {
    let uuid = quiz_uuid();
    let (year, month, day, _) = gen_timestamp();
    let qdo_path =
        get_qdo_path().with_context(|| "Failed to get the full path to the qdo directory")?;
    let mut quiz_path = qdo_path
        .join(year.to_string())
        .join(month.to_string())
        .join(day.to_string())
        .join(uuid.to_string());
    quiz_path.set_extension("html");
    Ok(quiz_path)
}

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
