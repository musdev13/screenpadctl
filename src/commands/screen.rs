use anyhow::{Context, Result};
use std::fs;

const BL_POWER: &str = "/sys/devices/platform/asus-nb-wmi/backlight/asus_screenpad/bl_power";

// Canonical backlight power states from <linux/backlight.h>:
//   BACKLIGHT_POWER_ON  = 0
//   BACKLIGHT_POWER_OFF = 4
const POWER_ON: u8 = 0;
const POWER_OFF: u8 = 4;

fn read_power() -> Result<u8> {
    let value =
        fs::read_to_string(BL_POWER).with_context(|| format!("failed to read {BL_POWER}"))?;

    value
        .trim()
        .parse()
        .with_context(|| format!("invalid ScreenPad power value in {BL_POWER}"))
}

fn write_power(value: u8) -> Result<()> {
    fs::write(BL_POWER, value.to_string())
        .with_context(|| format!("failed to write {BL_POWER}"))?;

    Ok(())
}

pub fn enable() -> Result<()> {
    write_power(POWER_ON)?;
    println!("ScreenPad enabled");
    Ok(())
}

pub fn disable() -> Result<()> {
    write_power(POWER_OFF)?;
    println!("ScreenPad disabled");
    Ok(())
}

pub fn toggle() -> Result<()> {
    match read_power()? {
        POWER_ON => disable(),
        POWER_OFF => enable(),
        value => anyhow::bail!("unknown ScreenPad power state: {value}"),
    }
}

pub fn status() -> Result<()> {
    match read_power()? {
        POWER_ON => println!("ScreenPad: enabled"),
        POWER_OFF => println!("ScreenPad: disabled"),
        value => println!("ScreenPad: unknown ({value})"),
    }

    Ok(())
}