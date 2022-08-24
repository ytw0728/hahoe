use rust_3d::Point3D;
use nanoid::nanoid;

pub struct Particle {
    id : String, //nanoid
    position : Point3D,
}

impl Particle {
    fn new() -> Self {
        Particle {
            id : nanoid!(),
            position : Point3D::new(0 as f64,0 as f64,0 as f64)
        }
    }
    pub fn new_Point3D(point3d : &Point3D) -> Self {
        let mut this = Particle::new();
        this.position = point3d.clone();
        this
    }
    
    pub fn new_position(x : f64, y : f64, z : f64) -> Self{
        let mut this = Particle::new();
        this.position = Point3D::new(x,y,z);
        this
    }
}

