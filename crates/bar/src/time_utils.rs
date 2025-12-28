use chrono::{Local, Timelike};
use ratatui::style::Color;

/// Returns the default day start hour (6:00 AM)
pub fn default_day_start() -> u8 {
    6
}

/// Returns the default night start hour (6:00 PM)
pub fn default_night_start() -> u8 {
    18
}

/// Determines if current time is nighttime based on configured boundaries
///
/// Nighttime is considered any hour before day_start OR after night_start
///
/// # Arguments
/// * `day_start` - Hour when daytime begins (0-23)
/// * `night_start` - Hour when nighttime begins (0-23)
///
/// # Returns
/// `true` if current time is nighttime, `false` if daytime
pub fn is_nighttime(day_start: u8, night_start: u8) -> bool {
    let hour = Local::now().hour();
    hour < day_start as u32 || hour >= night_start as u32
}

/// Determines if current time is daytime based on configured boundaries
///
/// Daytime is considered any hour from day_start (inclusive) to night_start (exclusive)
///
/// # Arguments
/// * `day_start` - Hour when daytime begins (0-23)
/// * `night_start` - Hour when nighttime begins (0-23)
///
/// # Returns
/// `true` if current time is daytime, `false` if nighttime
pub fn is_daytime(day_start: u8, night_start: u8) -> bool {
    let hour = Local::now().hour();
    hour >= day_start as u32 && hour < night_start as u32
}

/// Returns appropriate color based on current time period
///
/// # Arguments
/// * `day_color` - Color to use during daytime
/// * `night_color` - Color to use during nighttime
/// * `day_start` - Hour when daytime begins (0-23)
/// * `night_start` - Hour when nighttime begins (0-23)
///
/// # Returns
/// `day_color` if current time is daytime, `night_color` if nighttime
pub fn get_time_based_color(
    day_color: Color,
    night_color: Color,
    day_start: u8,
    night_start: u8,
) -> Color {
    if is_daytime(day_start, night_start) {
        day_color
    } else {
        night_color
    }
}
