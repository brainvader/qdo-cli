use std::default::Default;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{anyhow, Context, Ok, Result};
use chrono::TimeZone;
use chrono::{offset::Utc, DateTime, Datelike};
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use rust_embed::RustEmbed;
use uuid::Uuid;

pub struct TimeStamp {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub time: String,
}

impl Default for TimeStamp {
    fn default() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let jst: DateTime<Tz> = utc.with_timezone(&Tokyo);
        let year = jst.year();
        let month = jst.month();
        let day = jst.day();
        let time = jst.time().format("%H:%M:%S").to_string();
        TimeStamp {
            year,
            month,
            day,
            time,
        }
    }
}

impl TimeStamp {
    pub fn now<Tz: TimeZone>(time_zone: &Tz) -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let jst: DateTime<Tz> = utc.with_timezone(&time_zone);
        let year = jst.year();
        let month = jst.month();
        let day = jst.day();
        let time = jst.time().format("%H:%M:%S").to_string();
        TimeStamp {
            year,
            month,
            day,
            time,
        }
    }

    pub fn iso_8601_format(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{}",
            self.year, self.month, self.day, self.time
        )
    }
}

pub struct GakuNote {
    pub home: PathBuf,
    pub timestamp: TimeStamp,
    app_name: String,
    version: String,
}

impl GakuNote {
    pub fn new(home: PathBuf, timestamp: TimeStamp) -> Self {
        let app_name = env!("CARGO_PKG_NAME").to_owned();
        let version = env!("CARGO_PKG_VERSION").to_owned();

        GakuNote {
            home: home,
            timestamp: timestamp,
            app_name: app_name,
            version: version,
        }
    }

    pub fn generator_name(&self) -> String {
        format!("{} {}", self.app_name, self.version)
    }

    pub fn save(&self, content: &str) -> Result<()> {
        let uuid = quiz_uuid();
        let file_name = format!("{}.html", uuid);

        let mut file_path = self.home.clone();
        file_path.push(file_name);
        println!("{:?}", file_path);

        let mut file = File::create(file_path)
            .with_context(|| anyhow!("Failed to create quiz file {}", self.home.display()))?;
        file.write_all(content.as_bytes()).with_context(|| {
            anyhow!("Failed to write quiz HTML to file: {}", self.home.display())
        })?;

        println!("{}", self.home.display());

        Ok(())
    }
}

pub fn get_home_dir() -> Result<String> {
    let home_dir =
        env::var("HOME").with_context(|| "Failed to retrieve HOME environment variable")?;
    Ok(home_dir)
}

pub fn get_qdo_path(home_dir: &str) -> Result<PathBuf> {
    let qdo_dir = PathBuf::from(home_dir).join(".qdo");
    Ok(qdo_dir)
}

pub fn gen_qdo_dir(home_dir: &str) -> Result<()> {
    let qdo_dir =
        get_qdo_path(&home_dir).with_context(|| "Failed to generate a full path to qdo")?;

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

pub fn get_quiz_directory_path(qdo_path: &PathBuf) -> Result<PathBuf> {
    let timestamp = TimeStamp::default();
    let TimeStamp {
        year, month, day, ..
    } = timestamp;

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
