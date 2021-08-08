//! # Date
//!
//! Date module for metadata.
//! Since this library is no-std and dependency-free, it exposes an internal Date type with validation

/// ## Date
///
/// Date type for metadata
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    /// ### new
    ///
    /// Create a new date.
    /// Panics if date is invalid!
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        // Validate date
        match Self::validate(year, month, day) {
            Ok(dt) => dt,
            Err(err) => panic!("Invalid date: {}", err),
        }
    }

    // -- getter

    /// ### year
    ///
    /// get year
    pub fn year(&self) -> u16 {
        self.year
    }

    /// ### month
    ///
    /// get month
    pub fn month(&self) -> u8 {
        self.month
    }

    /// ### day
    ///
    /// get day
    pub fn day(&self) -> u8 {
        self.day
    }

    // -- validation

    /// ### validate
    ///
    /// Validate date and return a date if valid.
    /// If not valid return an error message
    fn validate(year: u16, month: u8, day: u8) -> Result<Self, &'static str> {
        if !Self::is_month_valid(month) {
            return Err("month must be in range [1-12]");
        }
        if !Self::is_day_valid(day) {
            return Err("day must be in range [1-31]");
        }
        if !Self::date_exists(day, month, year) {
            return Err("date doesn't exist");
        }
        Ok(Date { year, month, day })
    }

    /// ### is_month_valid
    ///
    /// Checks whether month is in range [1-12]
    fn is_month_valid(month: u8) -> bool {
        (1..=12).contains(&month)
    }

    /// ### is_day_valid
    ///
    /// Checks whether day is in range [1-31]
    fn is_day_valid(day: u8) -> bool {
        (1..=31).contains(&day)
    }

    /// ### date_exists
    ///
    /// Checks whether provided date actually exists in calendar
    fn date_exists(day: u8, month: u8, year: u16) -> bool {
        if month == 2 {
            // If month is february, check if day is less than 29 or 28 based on year
            let february_ceil: u8 = match Self::is_year_leap(year) {
                true => 29,
                false => 28,
            };
            // Day must be less or equal than ceil
            day <= february_ceil
        } else if [4, 6, 9, 11].contains(&month) {
            // If month is april, june, september or november, then must be less or equal than 30
            day <= 30
        } else {
            // Otherwise is valid
            true
        }
    }

    /// ### is_year_leap
    ///
    /// checks whether provided year is leap
    fn is_year_leap(year: u16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn validate_date() {
        assert!(Date::validate(2021, 08, 08).is_ok());
        assert!(Date::validate(2020, 02, 29).is_ok());
        assert!(Date::validate(2021, 02, 28).is_ok());
        assert!(Date::validate(2021, 12, 31).is_ok());
        assert!(Date::validate(2021, 04, 30).is_ok());
        assert!(Date::validate(1200, 02, 29).is_ok());
        assert!(Date::validate(2021, 04, 31).is_err());
        assert!(Date::validate(2021, 02, 29).is_err());
        assert!(Date::validate(1800, 02, 29).is_err());
    }

    #[test]
    fn new_date() {
        assert_eq!(
            Date::new(2021, 08, 08),
            Date {
                year: 2021,
                month: 8,
                day: 8
            }
        );
    }

    #[test]
    fn date_getters() {
        let date: Date = Date::new(2021, 08, 09);
        assert_eq!(date.day(), 9);
        assert_eq!(date.month(), 8);
        assert_eq!(date.year(), 2021);
    }

    #[test]
    #[should_panic]
    fn bad_date() {
        Date::new(2021, 04, 31);
    }
}
