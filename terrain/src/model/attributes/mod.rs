pub mod traits;

pub type Habitability = bool;

impl traits::DiscreteAttribute for Habitability {
    fn interpolate(t: f64) -> Self {
        if t < -0.7 || t > 0.7 {
            false
        } else {
            true
        }
    }
}

pub type Height = f64;

impl traits::ContinuousAttribute for f64 {
    fn interpolater(min: Self, max: Self) -> Box<dyn Fn(f64) -> Self> {
        Box::new(move |t| (min + (max - min) * t))
    }
}
