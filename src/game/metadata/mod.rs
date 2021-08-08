//! # Metadata
//!
//! this module exposes game metadata, such as player data, location and date
//!

use alloc::string::{String, ToString};

// -- modules
mod date;
mod location;
mod player;

// -- export
pub use date::Date;
pub use location::Location;
pub use player::Player;

/// ## Metadata
///
/// This structure wraps the game metadata.
/// The attributes have been designed to be used in PGN (portable game notation) and to store all the
/// information for a match
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Metadata {
    /// Name of the tournament or match event.
    event: Option<String>,
    /// Location of the event
    site: Option<Location>,
    /// Starting date of the game
    date: Option<Date>,
    /// Round number
    round: Option<u8>,
    /// Data of player moving white pieces
    white: Option<Player>,
    /// Data of player moving black pieces
    black: Option<Player>,
    /// Result of the game
    result: Result,
}

/// ## ResultTag
///
/// Describes the result of the game
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Result {
    /// White won the game
    WhiteWins,
    /// Black won the game
    BlackWins,
    /// The game is drawn
    DrawnGame,
    /// The game is still in progress
    InProgress,
    /// The game has been abandoned
    Abandoned,
    /// Unknown result
    Unknown,
}

/// ## Country
///
/// Country as specified in the list of the "International Olympic Committee"
/// <https://en.wikipedia.org/wiki/List_of_IOC_country_codes>
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Country {
    Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Aruba,
    AmericanSamoa,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bangladesh,
    Barbados,
    Burundi,
    Belgium,
    Benin,
    Bermuda,
    Bhutan,
    BosniaAndHerzegovina,
    Belize,
    Belarus,
    Bolivia,
    Botswana,
    Brazil,
    Bahrain,
    Brunei,
    Bulgaria,
    BurkinaFaso,
    CentralAfricanRepublic,
    Cambodia,
    Canada,
    CaymanIslands,
    RepublicOfTheCongo,
    Chad,
    Chile,
    China,
    IvoryCoast,
    Cameroon,
    DemocraticRepublicOfTheCongo,
    CookIslands,
    Colombia,
    Comoros,
    CapeVerde,
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    CzechRepublic,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    Ecuador,
    Egypt,
    Eritrea,
    ElSalvador,
    Spain,
    Estonia,
    Ethiopia,
    Fiji,
    Finland,
    France,
    FederatedStatesOfMicronesia,
    Gabon,
    TheGambia,
    GreatBritain,
    GuineaBissau,
    Georgia,
    EquatorialGuinea,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    Guam,
    Guyana,
    Haiti,
    HongKong,
    Honduras,
    Hungary,
    Indonesia,
    India,
    Iran,
    Ireland,
    Iraq,
    Iceland,
    Israel,
    VirginIslands,
    Italy,
    BritishVirginIslands,
    Jamaica,
    Jordan,
    Japan,
    Kazakhstan,
    Kenya,
    Kyrgyzstan,
    Kiribati,
    SouthKorea,
    Kosovo,
    SaudiArabia,
    Kuwait,
    Laos,
    Latvia,
    Libya,
    Lebanon,
    Liberia,
    SaintLucia,
    Lesotho,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Madagascar,
    Morocco,
    Malaysia,
    Malawi,
    Moldova,
    Maldives,
    Mexico,
    Mongolia,
    MarshallIslands,
    NorthMacedonia,
    Mali,
    Malta,
    Montenegro,
    Monaco,
    Mozambique,
    Mauritius,
    Mauritania,
    Myanmar,
    Namibia,
    Nicaragua,
    Netherlands,
    Nepal,
    Nigeria,
    Niger,
    Norway,
    Nauru,
    NewZealand,
    Oman,
    Pakistan,
    Panama,
    Paraguay,
    Peru,
    Philippines,
    Palestine,
    Palau,
    PapuaNewGuinea,
    Poland,
    Portugal,
    NorthKorea,
    PuertoRico,
    Qatar,
    Romania,
    SouthAfrica,
    Russia,
    Rwanda,
    Samoa,
    Senegal,
    Seychelles,
    Singapore,
    SaintKittsAndNevis,
    SierraLeone,
    Slovenia,
    SanMarino,
    SolomonIslands,
    Somalia,
    Serbia,
    SriLanka,
    SouthSudan,
    SaoTomeAndPrincipe,
    Sudan,
    Switzerland,
    Suriname,
    Slovakia,
    Sweden,
    Eswatini,
    Syria,
    Tanzania,
    Tonga,
    Thailand,
    Tajikistan,
    Turkmenistan,
    EastTimor,
    Togo,
    ChineseTaipei,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Tuvalu,
    UnitedArabEmirates,
    Uganda,
    Ukraine,
    Uruguay,
    UnitedStates,
    Uzbekistan,
    Vanuatu,
    Venezuela,
    Vietnam,
    SaintVincentAndTheGrenadines,
    Yemen,
    Zambia,
    Zimbabwe,
}

// -- metadata implementation

impl Default for Metadata {
    fn default() -> Self {
        Self {
            event: None,
            site: None,
            date: None,
            round: None,
            white: None,
            black: None,
            result: Result::Unknown,
        }
    }
}

impl Metadata {
    // -- getters

    /// ### event
    ///
    /// Get metadata event
    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    /// ### site
    ///
    /// Get metadata site
    pub fn site(&self) -> Option<&Location> {
        self.site.as_ref()
    }

    /// ### date
    ///
    /// Get metadata date
    pub fn date(&self) -> Option<&Date> {
        self.date.as_ref()
    }

    /// ### round
    ///
    /// Get metadata round
    pub fn round(&self) -> Option<u8> {
        self.round
    }

    /// ### white_player
    ///
    /// Get white player
    pub fn white_player(&self) -> Option<&Player> {
        self.white.as_ref()
    }

    /// ### black_player
    ///
    /// Get black player
    pub fn black_player(&self) -> Option<&Player> {
        self.black.as_ref()
    }

    /// ### result
    ///
    /// Get result
    pub fn result(&self) -> Result {
        self.result
    }

    // -- setters

    /// ### set_result
    ///
    /// Set result to metadata
    pub fn set_result(&mut self, result: Result) {
        self.result = result;
    }

    // -- constructors

    /// ### with_event
    ///
    /// Set event to metadata
    pub fn with_event<S: AsRef<str>>(mut self, ev: S) -> Self {
        self.event = Some(ev.as_ref().to_string());
        self
    }

    /// ### with_site
    ///
    /// Set location to metadata
    pub fn with_site<S: AsRef<str>>(mut self, city: S, region: S, country: Country) -> Self {
        self.site = Some(Location::new(city, region, country));
        self
    }

    /// ### with_date
    ///
    /// Set date to metadata.
    /// Panics if date is invalid
    pub fn with_date(mut self, year: u16, month: u8, day: u8) -> Self {
        self.date = Some(Date::new(year, month, day));
        self
    }

    /// ### with_round
    ///
    /// Set round to metadata
    pub fn with_round(mut self, round: u8) -> Self {
        self.round = Some(round);
        self
    }

    /// ### with_white_player
    ///
    /// Set white player for metadata
    pub fn with_white_player(mut self, player: Player) -> Self {
        self.white = Some(player);
        self
    }

    /// ### with_black_player
    ///
    /// Set black player for metadata
    pub fn with_black_player(mut self, player: Player) -> Self {
        self.black = Some(player);
        self
    }

    /// ### with_result
    ///
    /// Build metadata with provided result
    pub fn with_result(mut self, result: Result) -> Self {
        self.result = result;
        self
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn metadata_default() {
        let metadata: Metadata = Metadata::default();
        assert_eq!(metadata.black, None);
        assert_eq!(metadata.date, None);
        assert_eq!(metadata.event, None);
        assert_eq!(metadata.result, Result::Unknown);
        assert_eq!(metadata.round, None);
        assert_eq!(metadata.site, None);
        assert_eq!(metadata.white, None);
    }

    #[test]
    fn metadata_builder() {
        let metadata: Metadata = Metadata::default()
            .with_black_player(Player::new("magnus", "carlsen", Country::Norway, 2882))
            .with_date(2021, 08, 08)
            .with_event("sagra della porchetta vegana")
            .with_result(Result::BlackWins)
            .with_round(1)
            .with_site("moimacco", "friuli-venezia giulia", Country::Italy)
            .with_white_player(Player::new("garri", "kasparov", Country::Russia, 2851));

        assert_eq!(
            metadata.black_player().unwrap(),
            &Player::new("magnus", "carlsen", Country::Norway, 2882)
        );
        assert_eq!(metadata.date().unwrap(), &Date::new(2021, 08, 08));
        assert_eq!(metadata.event().unwrap(), "sagra della porchetta vegana");
        assert_eq!(metadata.result(), Result::BlackWins);
        assert_eq!(metadata.round().unwrap(), 1);
        assert_eq!(metadata.site().unwrap().city(), "moimacco");
        assert_eq!(
            metadata.white_player().unwrap(),
            &Player::new("garri", "kasparov", Country::Russia, 2851)
        );
    }
}
