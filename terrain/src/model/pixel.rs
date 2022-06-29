#[derive(Debug, Clone)]
pub struct Pixel {
    pub height: f64,
    pub moisture: f64,
    pub habitance: bool,
}

impl Pixel {
    pub fn make_dummy() -> Pixel {
        Pixel {
            height: 0f64,
            moisture: 0f64,
            habitance: false,
        }
    }
}
