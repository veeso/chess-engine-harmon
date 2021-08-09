//! # Location
//!
//! Describes a location, according to chess standard notations

use alloc::string::{String, ToString};

use super::Country;

/// ## Location
///
/// The location of the match
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    city: String,
    region: String,
    country: Country,
}

impl Location {
    /// ### new
    ///
    /// Instantiates a new `Location`
    pub fn new<S: AsRef<str>>(city: S, region: S, country: Country) -> Self {
        Self {
            city: city.as_ref().to_string(),
            region: region.as_ref().to_string(),
            country,
        }
    }

    /// ### city
    ///
    /// Return reference to city
    pub fn city(&self) -> &str {
        self.city.as_ref()
    }

    /// ### region
    ///
    /// Return reference to region
    pub fn region(&self) -> &str {
        self.region.as_ref()
    }

    /// ### country
    ///
    /// Get country
    pub fn country(&self) -> Country {
        self.country
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn location() {
        let location: Location = Location {
            city: "moimacco".to_string(),
            region: "friuli-venezia giulia".to_string(),
            country: Country::Italy,
        };
        assert_eq!(location.city(), "moimacco");
        assert_eq!(location.country(), Country::Italy);
        assert_eq!(location.region(), "friuli-venezia giulia");
        let location: Location = Location::new("moimacco", "friuli-venezia giulia", Country::Italy);
        assert_eq!(location.city(), "moimacco");
        assert_eq!(location.country(), Country::Italy);
        assert_eq!(location.region(), "friuli-venezia giulia");
    }
}
