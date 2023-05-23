use std::f64::consts::PI;

/// Covert degrees to radians
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * (PI / 180.)
}

/// Covert radians to degrees
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * (180. / PI)
}
