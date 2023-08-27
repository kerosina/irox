//!
//! Structs around the watch command

/// Controls 'raw' mode
pub enum RawMode {
    /// In NMEA mode, returns the raw strings
    /// In binary mode, provides hex encoded strings
    AsciiDumped = 1,

    /// In binary mode, reports data verbatim without hex encoding
    RawBinary = 2,
}

/// This command sets watcher mode.
pub struct Watch {
    enable: Option<bool>,
    json: Option<bool>,
    nmea: Option<bool>,
    raw: Option<RawMode>,
    scaled: Option<bool>,
    split24: Option<bool>,
    pps: Option<bool>,
    remote: Option<String>,
}
