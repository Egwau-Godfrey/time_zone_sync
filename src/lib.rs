//! # Timezone Converter
//! 
//! A Rust library for converting times between different timezones and getting timezone information.
//! This library provides functionality to:
//! 
//! - Convert times between any two timezones
//! - Get current time in different timezones
//! - Get timezone information including offset and DST status
//! - Calculate time differences between timezones
//! 
//! ## Example
//! ```rust
//! use timezone_converter::TimeZoneConverter;
//! 
//! let converter = TimeZoneConverter::new("America/New_York", "Europe/London").unwrap();
//! let current_time = converter.get_current_time_source().unwrap();
//! let converted_time = converter.convert(current_time).unwrap();
//! ```

use chrono::{DateTime, TimeZone as ChronoTimeZone, Utc, Duration, Offset};
use chrono_tz::{OffsetName, Tz};

/// A struct that handles timezone conversions between a source and target timezone
#[derive(Debug)]
pub struct TimeZoneConverter {
    /// The source timezone to convert from
    source_tz: Tz,
    /// The target timezone to convert to
    target_tz: Tz,
}

/// Represents detailed information about a timezone
#[derive(Debug)]
pub struct TimeZoneInfo {
    /// The name of the timezone (e.g., "America/New_York")
    name: String,
    /// The offset from UTC
    offset: Duration,
    /// Whether Daylight Saving Time is currently in effect
    is_dst: bool,
}

/// Possible errors that can occur during timezone operations
#[derive(Debug)]
pub enum Errors {
    /// Error when an invalid timezone identifier is provided
    InvalidTimeZone(String),
    /// Error when parsing datetime strings
    ParseError(String),
    /// Error during timezone conversion
    ConversionError(String),
}

impl TimeZoneConverter {
    /// Creates a new TimeZoneConverter instance
    /// 
    /// # Arguments
    /// 
    /// * `source` - The source timezone identifier (e.g., "America/New_York")
    /// * `target` - The target timezone identifier (e.g., "Europe/London")
    /// 
    /// # Returns
    /// 
    /// * `Result<TimeZoneConverter, Errors>` - A new TimeZoneConverter instance or an error
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use timezone_converter::TimeZoneConverter;
    /// 
    /// let converter = TimeZoneConverter::new("America/New_York", "Europe/London").unwrap();
    /// ```
    pub fn new(source: &str, target: &str) -> Result<Self, Errors> {
        let source_tz = source.parse::<Tz>().map_err(|_| Errors::InvalidTimeZone(source.to_string()))?;
        let target_tz = target.parse::<Tz>().map_err(|_| Errors::InvalidTimeZone(target.to_string()))?;

        Ok(Self {
            source_tz,
            target_tz
        })
    }

    /// Converts a datetime from the source timezone to the target timezone
    /// 
    /// # Arguments
    /// 
    /// * `datetime` - The datetime to convert
    /// 
    /// # Returns
    /// 
    /// * `Result<DateTime<Tz>, Errors>` - The converted datetime or an error
    pub fn convert<T: ChronoTimeZone>(&self, datetime: DateTime<T>) -> Result<DateTime<Tz>, Errors> {
        Ok(
            datetime.with_timezone(&self.target_tz)
        )
    }

    /// Gets the current time in the source timezone
    /// 
    /// # Returns
    /// 
    /// * `Result<DateTime<Tz>, Errors>` - The current time in the source timezone
    pub fn get_current_time_source(&self) -> Result<DateTime<Tz>, Errors> {
        Ok(
            Utc::now().with_timezone(&self.source_tz)
        )
    }

    /// Gets the current time in the target timezone
    /// 
    /// # Returns
    /// 
    /// * `Result<DateTime<Tz>, Errors>` - The current time in the target timezone
    pub fn get_current_time_target(&self) -> Result<DateTime<Tz>, Errors> {
        Ok(
            Utc::now().with_timezone(&self.target_tz)
        )
    }

    /// Gets detailed information about the source timezone
    /// 
    /// # Returns
    /// 
    /// * `Result<TimeZoneInfo, Errors>` - Information about the timezone including name, offset, and DST status
    pub fn get_timezone_info(&self) -> Result<TimeZoneInfo, Errors> {
        let now = Utc::now().with_timezone(&self.source_tz);
        let offset = now.offset();

        // Calculate the total offset in seconds
        let total_offset_seconds = offset.fix().local_minus_utc();
        // Determine if DST is in effect by checking the offset abbreviation
        let is_dst = match offset.abbreviation() {
            Some(abbr) => abbr.ends_with("DT"),
            None => false,
        };

        Ok(TimeZoneInfo {
            name: self.source_tz.name().to_string(),
            offset: Duration::seconds(total_offset_seconds as i64),
            is_dst,
        })
    }

    /// Gets the time difference between source and target timezones in hours
    /// 
    /// # Returns
    /// 
    /// * `Result<f64, Errors>` - The time difference in hours (positive if source is ahead)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use timezone_converter::TimeZoneConverter;
    /// 
    /// let converter = TimeZoneConverter::new("America/New_York", "Europe/London").unwrap();
    /// let difference = converter.get_time_difference().unwrap();
    /// println!("Time difference: {} hours", difference);
    /// ```
    pub fn get_time_difference(&self) -> Result<f64, Errors> {
        let now = Utc::now();
        
        // Get the offsets for both timezones
        let source_time = now.with_timezone(&self.source_tz);
        let target_time = now.with_timezone(&self.target_tz);
        
        let source_offset = source_time.offset().fix().local_minus_utc();
        let target_offset = target_time.offset().fix().local_minus_utc();
        
        // Convert seconds to hours (f64 for decimal hours)
        Ok((source_offset - target_offset) as f64 / 3600.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::America::New_York;
    use chrono_tz::Africa::Kampala;

    #[test]
    fn NewYorktoKampala() {
        let timezone = TimeZoneConverter::new("America/New_York", "Africa/Kampala").unwrap();
        // Create a specific time in New York
        let ny_time = New_York.with_ymd_and_hms(2024, 11, 4, 10, 0, 0).unwrap();
        let time = timezone.convert(ny_time).unwrap();
        // Print both times to verify the conversion
        println!("New York: {}", ny_time);
        println!("Kampala: {}", time);
    }

    #[test]
    fn NewYorkToKampala_current() {
        let timezone = TimeZoneConverter::new("America/New_York", "Africa/Kampala").unwrap();
        let ny_time = Utc::now().with_timezone(&New_York);
        let time = timezone.convert(ny_time).unwrap();
        println!("New York: {}", ny_time);
        println!("Kampala: {}", time);
    }

    #[test]
    fn get_timezone_info() {
        let timezone = TimeZoneConverter::new("America/New_York", "Africa/Kampala").unwrap();
        let info = timezone.get_timezone_info().unwrap();
        println!("{:?}", info);
    }

    #[test]
    fn get_time_difference() {
        let timezone = TimeZoneConverter::new("America/New_York", "Africa/Kampala").unwrap();
        let difference = timezone.get_time_difference().unwrap();
        println!("Time difference: {} hours", difference);
    }
}
