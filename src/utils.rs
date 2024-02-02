use std::io::Read;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{anyhow, Context, Ok, Result};
use chrono::{offset::Utc, DateTime, Datelike};
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use rust_embed::RustEmbed;
use uuid::Uuid;

pub struct TimeStamp {
    year: i32,
    month: u32,
    day: u32,
    time: String,
}

impl TimeStamp {
    pub fn iso_8601_format(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{}",
            self.year, self.month, self.day, self.time
        )
    }
}

pub fn get_home_dir() -> Result<String> {
    let home_dir =
        env::var("HOME").with_context(|| "Failed to retrieve HOME environment variable")?;
    Ok(home_dir)
}

pub fn get_qdo_path() -> Result<PathBuf> {
    let home_dir = get_home_dir()?;
    let qdo_dir = PathBuf::from(home_dir).join(".qdo");
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

pub fn get_quiz_directory_path(timestamp: &TimeStamp) -> Result<PathBuf> {
    let TimeStamp {
        year, month, day, ..
    } = timestamp;
    let qdo_path =
        get_qdo_path().with_context(|| "Failed to get the full path to the qdo directory")?;

    Ok(qdo_path
        .join(year.to_string())
        .join(month.to_string())
        .join(day.to_string()))
}

const ASSET_NAME: &str = "quiz.html";
#[derive(RustEmbed)]
#[folder = "templates/"]
struct DefaultTemplateAsset;

pub fn get_default_quiz_template() -> Result<String> {
    let asset = DefaultTemplateAsset::get(ASSET_NAME)
        .ok_or_else(|| anyhow!("Asset not found: {}", ASSET_NAME))?;
    let data = asset.data.as_ref();
    let template_str =
        std::str::from_utf8(&data).context("Failed to convert asset data to string")?;
    Ok(template_str.to_owned())
}

/// Reads and returns the content of a quiz template file.
///
/// This function opens a file specified by the given path, reads its content,
/// and returns the content as a `String`. It handles errors during file opening
/// and reading, providing contextual information in case of failure.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the quiz template file.
///
/// # Returns
///
/// This function returns a `Result<String>` which is `Ok` containing the file content
/// as a string if the file is successfully read, or an `Err` with an error message
/// if the file cannot be opened or read.
///
/// # Errors
///
/// This function will return an error if the file specified by `path` cannot be opened,
/// or if the file content cannot be read into a string.
///
/// # Examples
///
/// ```
/// let template_content = get_quiz_template("path/to/template.html")
///     .expect("Failed to read the template file");
/// println!("{}", template_content);
/// ```
pub fn get_quiz_template(path: &str) -> Result<String> {
    let mut file = fs::File::open(path).with_context(|| anyhow!("Failed to open {}", path))?;
    let mut template = String::new();
    file.read_to_string(&mut template)
        .with_context(|| anyhow!("Failed to read {}", path))?;
    Ok(template)
}

pub fn quiz_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn gen_timestamp() -> Result<TimeStamp> {
    // generate time stamp from the current time
    let utc: DateTime<Utc> = Utc::now();
    let jst: DateTime<Tz> = utc.with_timezone(&Tokyo);
    let year = jst.year();
    let month = jst.month();
    let day = jst.day();
    let time = jst.time().format("%H:%M:%S").to_string();
    Ok(TimeStamp {
        year,
        month,
        day,
        time,
    })
}
