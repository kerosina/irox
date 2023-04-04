//!
//! ITRS GRS80 Ellipsoid ca 1979
//!
use crate::geo::ellipse::Ellipse;
use irox_units::units::length::Length;

/// ITRS GRS80 Semi-major axis
pub const GRS80_SEMI_MAJOR_LENGTH: Length = Length::new_meters(6378137.0);
/// ITRS GRS80 Inverse flattening
pub const GRS80_INVERSE_FLATTENING: f64 = 298.257_222_101;
/// ITRS GRS80 Ellipse params
pub const GRS80_PARAMS: Ellipse = Ellipse::new(GRS80_SEMI_MAJOR_LENGTH, GRS80_INVERSE_FLATTENING);
