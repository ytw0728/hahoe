use rust_3d::Point3D;
use super::particle::Particle;

pub struct ParticlePool {
    particles : Vec<Particle>,
}

impl ParticlePool {
    fn new() -> Self {
        ParticlePool {
            particles : Vec::new(),
        }
    }

    fn insert_particle(&mut self, number : u32, position : Point3D){
        self.particles.append(
                &mut (0..number).map(|_| -> Particle { Particle::new_point3d(&position)})
                .collect::<Vec<_>>()
            );
    }

    fn replace_particle(&mut self, particles : Vec<Particle>) {
        self.particles = particles;
    }

    fn get_ref_paticle(&self) -> &Vec<Particle>{
        &self.particles
    }
}

