use anyhow::{Context, Result};
use std::fs;

const BRIGHTNESS: &str = "/sys/devices/platform/asus-nb-wmi/backlight/asus_screenpad/brightness";

const ACTUAL_BRIGHTNESS: &str =
    "/sys/devices/platform/asus-nb-wmi/backlight/asus_screenpad/actual_brightness";

const MAX_BRIGHTNESS: &str =
    "/sys/devices/platform/asus-nb-wmi/backlight/asus_screenpad/max_brightness";

fn read_value(path: &str) -> Result<i32> {
    let value = fs::read_to_string(path).with_context(|| format!("failed to read {path}"))?;

    Ok(value
        .trim()
        .parse()
        .with_context(|| format!("invalid value in {path}"))?)
}

fn write_value(path: &str, value: i32) -> Result<()> {
    fs::write(path, value.to_string()).with_context(|| format!("failed to write {path}"))?;

    Ok(())
}

pub fn status() -> Result<()> {
    let brightness = read_value(BRIGHTNESS)?;
    let actual = read_value(ACTUAL_BRIGHTNESS)?;
    let max = read_value(MAX_BRIGHTNESS)?;

    let percent = if max > 0 { brightness * 100 / max } else { 0 };

    println!("Brightness: {brightness}/{max} ({percent}%)");
    println!("Actual:     {actual}/{max}");

    Ok(())
}

pub fn set(value: &str) -> Result<()> {
    let current = read_value(BRIGHTNESS)?;
    let max = read_value(MAX_BRIGHTNESS)?;

    let new_value = if let Some(value) = value.strip_prefix('+') {
        let amount: i32 = value.parse().context("invalid brightness increment")?;

        current + amount
    } else if let Some(value) = value.strip_prefix('-') {
        let amount: i32 = value.parse().context("invalid brightness decrement")?;

        current - amount
    } else {
        value.parse().context("invalid brightness value")?
    };

    let new_value = new_value.clamp(0, max);

    write_value(BRIGHTNESS, new_value)?;

    println!("Brightness: {new_value}/{max}");

    Ok(())
}
