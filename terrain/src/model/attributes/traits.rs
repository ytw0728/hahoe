pub trait DiscreteAttribute {
    fn interpolate(t: f64) -> Self;
}

// Currying을 달성하기 위한 Box...?
// https://stackoverflow.com/questions/36414576/returning-a-closure-from-a-trait-method-involving-generics-in-rust
pub trait ContinuousAttribute {
    fn interpolater(min: Self, max: Self) -> Box<dyn Fn(f64) -> Self>;
}
