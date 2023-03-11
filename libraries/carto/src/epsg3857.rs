use std::f64::consts::{PI, TAU};

use irox_units::{
    coordinate::{CartesianCoordinate, EllipticalCoordinate},
    geo::standards::WGS84_SHAPE,
    units::angle::{self, Angle},
};

use crate::proj::Projection;

pub struct SphericalMercatorProjection {
    zoom_level: u8,
}

impl SphericalMercatorProjection {
    pub fn new(zoom_level: u8) -> SphericalMercatorProjection {
        SphericalMercatorProjection { zoom_level }
    }

    pub fn tile_x(&self, coordinate: &EllipticalCoordinate) -> f64 {
        let lon_deg = coordinate.get_longitude().as_degrees().value();
        let offset = (lon_deg + 180.) / 360.;

        offset * (1 << self.zoom_level) as f64
    }

    pub fn tile_y(&self, coordinate: &EllipticalCoordinate) -> f64 {
        let lat_rad = coordinate.get_latitude().as_radians().value();
        let offset = (1. - (lat_rad.tan() + 1. / lat_rad.cos()).ln()) / 2.;
        offset * (1 << self.zoom_level) as f64
    }

    pub fn latitude(&self, tile_y: f64) -> Angle {
        let offset = 1. - (2. * tile_y) / (1 << self.zoom_level) as f64;
        Angle::new_radians((PI * offset).sinh().atan())
    }

    pub fn longitude(&self, tile_x: f64) -> Angle {
        let offset = tile_x / (1 << self.zoom_level) as f64;
        Angle::new_radians(offset * TAU - PI)
    }
}

impl Projection for SphericalMercatorProjection {
    fn get_center_coords(&self) -> &irox_units::coordinate::EllipticalCoordinate {
        &CENTER_COORDS
    }

    fn project_to_cartesian(
        &self,
        coord: &irox_units::coordinate::EllipticalCoordinate,
    ) -> irox_units::coordinate::CartesianCoordinate {
        let x = self.tile_x(coord) * TILE_TO_PIXEL;
        let y = self.tile_y(coord) * TILE_TO_PIXEL;
        let z = self.zoom_level as f64;

        CartesianCoordinate::new_meters(x, y, z)
    }

    fn project_to_elliptical(
        &self,
        coord: &irox_units::coordinate::CartesianCoordinate,
    ) -> irox_units::coordinate::EllipticalCoordinate {
        let lat = self.latitude(coord.get_y().as_meters().value());
        let lon = self.longitude(coord.get_x().as_meters().value());

        EllipticalCoordinate::new(lat, lon, WGS84_SHAPE)
    }
}

pub const UPPER_LEFT_COORDINATE_X: f64 = -180.0;
pub const UPPER_LEFT_COORDINATE_Y: f64 = 85.051_128_779_806_59;

pub const LOWER_LEFT_COORDINATE_X: f64 = -180.0;
pub const LOWER_LEFT_COORDINATE_Y: f64 = -85.051_128_779_806_59;

pub const UPPER_RIGHT_COORDINATE_X: f64 = -UPPER_LEFT_COORDINATE_X;
pub const UPPER_RIGHT_COORDINATE_Y: f64 = UPPER_LEFT_COORDINATE_Y;

pub const LOWER_RIGHT_COORDINATE_X: f64 = -LOWER_LEFT_COORDINATE_X;
pub const LOWER_RIGHT_COORDINATE_Y: f64 = LOWER_LEFT_COORDINATE_Y;

pub static CENTER_COORDS: EllipticalCoordinate =
    EllipticalCoordinate::new(angle::ZERO, angle::ZERO, WGS84_SHAPE);

// pub const BOUNDS: Bounds<CartesianCoordinate> = Bounds::new()

const TILE_TO_PIXEL: f64 = 40.743_665_431_525_21;

#[cfg(test)]
mod test {
    use super::SphericalMercatorProjection;

    #[test]
    pub fn test1() {
        let sm1 = SphericalMercatorProjection::new(1);

        assert_eq!(0.0, sm1.latitude(1.0).as_degrees().value());
        assert_eq!(0.0, sm1.longitude(1.0).as_degrees().value());
    }
}
