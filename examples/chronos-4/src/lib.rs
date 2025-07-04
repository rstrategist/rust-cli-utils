use chrono::{DateTime, Local, TimeZone, SecondsFormat};

pub fn generate_timestamp(offset_seconds: i64, format: &str) -> Result<String, String> {
    let now = Local::now();
    let target_time = now + chrono::Duration::seconds(offset_seconds);

    match format.to_lowercase().as_str() {
        "rfc3339" => Ok(target_time.to_rfc3339_opts(SecondsFormat::Secs, true)),
        "default" => Ok(target_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        other => Err(format!("Unsupported format: {}", other)),
    }
}