use rust_3d::Point3D;
use super::particle::{Particle, self};

pub struct ParticlePool {
    particles : Vec<Particle>,
}

impl ParticlePool {
    fn new() -> Self {
        ParticlePool {
            particles : Vec::new(),
        }
    }

    fn InsertParticle(&mut self, number : u32, position : Point3D){
        self.particles.append(
                &mut (0..number).map(|_| -> Particle { Particle::new_Point3D(&position)})
                .collect::<Vec<_>>()
            );
    }

    fn ReplaceParticle(&mut self, particles : Vec<Particle>) {
        self.particles = particles;
    }

    fn GetRefPaticle(&self) -> &Vec<Particle>{
        &self.particles
    }
}

