use rust_3d::Point3D;

pub struct Particle {
    position : Point3D,
}

impl Particle {
    fn new() -> Self {
        Particle {
            position : Point3D::new(0 as f64,0 as f64,0 as f64)
        }
    }
}

