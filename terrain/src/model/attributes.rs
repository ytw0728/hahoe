pub mod traits;

pub type Habitability = bool;

impl traits::DiscreteAttribute for Habitability {
    fn interpolate(t: f64) -> Self {
        !(-0.7..=0.7).contains(&t)
    }
}

pub type Height = f64;

impl traits::ContinuousAttribute for f64 {
    fn interpolater(min: Self, max: Self) -> Box<dyn Fn(f64) -> Self> {
        Box::new(move |t| (min + (max - min) * t))
    }
}
