use super::consts;
use crate::consts::DISTANCE_LIMIT_SQUARED;
use ndarray::prelude::*;

const PI: f32 = std::f32::consts::PI;

pub fn is_close_enough(r_squared: f32) -> bool {
    r_squared <= DISTANCE_LIMIT_SQUARED
}

pub fn poly_6(r: f32) -> f32 {
    let magic_number: f32 = 315.0 / 64.0 / PI / consts::DISTANCE_LIMIT.powi(9);
    (DISTANCE_LIMIT_SQUARED - r * r).powi(3) * magic_number
}

pub fn spiky(r: f32) -> f32 {
    let magic_number = 15.0 / PI / consts::DISTANCE_LIMIT.powi(6);

    (consts::DISTANCE_LIMIT - r).powi(3) * magic_number
}

pub fn grad_spiky(r1: &Array1<f32>, r2: &Array1<f32>) -> Array1<f32> {
    let dr = r1 - r2;
    let distance = dr.dot(&dr).sqrt();
    let magic_number = -45.0 / PI / consts::DISTANCE_LIMIT.powi(6);
    let nominator = (consts::DISTANCE_LIMIT - distance).powi(2) * magic_number;
    magic_number * nominator / distance * dr
}

pub fn viscosity(r: f32) -> f32 {
    let magic_number = 15.0 / 2.0 / PI / consts::DISTANCE_LIMIT.powi(3);

    let ratio = r / consts::DISTANCE_LIMIT;
    let nominator = -ratio.powi(3) / 2.0 + ratio.powi(2) + ratio / 2.0 - 1.0;

    nominator * magic_number
}

pub fn laplacian_viscosity(r: f32) -> f32 {
    let magic_number = 45.0 / PI / consts::DISTANCE_LIMIT.powi(6);
    (consts::DISTANCE_LIMIT - r) * magic_number
}
