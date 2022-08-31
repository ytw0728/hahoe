use super::particles::{Particle, SimulationStepInput};
use ndarray::prelude::*;
use std::collections::HashMap;
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new(particles: Vec<Particle>) -> ParticleSystem {
        ParticleSystem { particles }
    }
}

impl ParticleSystem {
    pub fn tick(&mut self, dt: f32) {
        let mut forces: HashMap<usize, (f32, Array1<f32>)> = HashMap::new();
        for particle in &self.particles {
            let force = particle.force(&self.particles);
            let density = particle.density(&self.particles);
            forces.insert(particle.id, (density, force));
        }
        for particle in &mut self.particles {
            let (density, force) = forces.get(&particle.id).unwrap();
            particle.tick(SimulationStepInput {
                density: *density,
                force: force,
                dt,
            });
        }
    }
}
