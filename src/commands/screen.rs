use anyhow::{Context, Result};
use std::fs;

const BL_POWER: &str = "/sys/devices/platform/asus-nb-wmi/backlight/asus_screenpad/bl_power";

fn read_power() -> Result<u8> {
    let value =
        fs::read_to_string(BL_POWER).with_context(|| format!("failed to read {BL_POWER}"))?;

    Ok(value
        .trim()
        .parse()
        .context("invalid ScreenPad power value")?)
}

fn write_power(value: u8) -> Result<()> {
    fs::write(BL_POWER, value.to_string())
        .with_context(|| format!("failed to write {BL_POWER}"))?;

    Ok(())
}

pub fn enable() -> Result<()> {
    write_power(1)?;
    println!("ScreenPad enabled");
    Ok(())
}

pub fn disable() -> Result<()> {
    write_power(0)?;
    println!("ScreenPad disabled");
    Ok(())
}

pub fn toggle() -> Result<()> {
    match read_power()? {
        1 => disable(),
        0 => enable(),
        value => anyhow::bail!("unknown ScreenPad power state: {value}"),
    }
}

pub fn status() -> Result<()> {
    match read_power()? {
        1 => println!("ScreenPad: enabled"),
        0 => println!("ScreenPad: disabled"),
        value => println!("ScreenPad: unknown ({value})"),
    }

    Ok(())
}
