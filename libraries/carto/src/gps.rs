// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct DilutionOfPrecision(f64);

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct DOPs {
    geometric: Option<DilutionOfPrecision>,
    horizontal: Option<DilutionOfPrecision>,
    position: Option<DilutionOfPrecision>,
    time: Option<DilutionOfPrecision>,
    vertical: Option<DilutionOfPrecision>,
}

impl DOPs {
    #[must_use]
    pub fn new() -> DOPs {
        Default::default()
    }
}

impl Display for DOPs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let print = |x:Option<DilutionOfPrecision>| {
            match x {
                Some(x) => format!("{:0.3}", x.0),
                None => "?".to_string()
            }
        };
        write!(f, "hdop: {} vdop: {} pdop: {} gdop: {} tdop: {}",
            print(self.horizontal),
            print(self.vertical),
            print(self.position),
            print(self.geometric),
            print(self.time)
        )
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use windows::Devices::Geolocation::Geocoordinate;
    use windows::Foundation::IReference;

    use crate::gps::{DilutionOfPrecision, DOPs};

    impl DOPs {
        pub fn maybe_from(coord: &Geocoordinate) -> Option<DOPs> {
            let Ok(sats) = coord.SatelliteData() else {
                return None;
            };

            let get_dop = |v:IReference<f64>| -> Option<DilutionOfPrecision> {
                v.GetDouble().ok().map(DilutionOfPrecision)
            };
            let geometric = sats.GeometricDilutionOfPrecision().ok().and_then(get_dop);
            let horizontal = sats.HorizontalDilutionOfPrecision().ok().and_then(get_dop);
            let position = sats.PositionDilutionOfPrecision().ok().and_then(get_dop);
            let time = sats.TimeDilutionOfPrecision().ok().and_then(get_dop);
            let vertical = sats.VerticalDilutionOfPrecision().ok().and_then(get_dop);

            Some(DOPs {
                geometric,
                horizontal,
                position,
                time,
                vertical,
            })
        }
    }
}