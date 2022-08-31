use super::{consts, kernels};
use ndarray::prelude::*;

pub struct Particle {
    pub id: usize,
    pub position: Array1<f32>,
    pub velocity: Array1<f32>,
    pub acceleration: Array1<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: Array1<f32>,
}

// Properties
impl Particle {
    pub fn distance_squared(&self, other: &Particle) -> f32 {
        let dp = &self.position - &other.position;
        dp.dot(&dp)
    }

    pub fn distance(&self, other: &Particle) -> f32 {
        self.distance_squared(other).sqrt()
    }

    pub fn density(&self, others: &[Particle]) -> f32 {
        let mut density = 0.0;
        for other in others {
            let distance_squared = self.distance_squared(other);
            if distance_squared < consts::DISTANCE_LIMIT_SQUARED {
                let distance = distance_squared.sqrt();
                density += kernels::poly_6(distance);
            }
        }
        density * consts::PARTICLE_MASS
    }

    pub fn pressure(&self, others: &[Particle]) -> f32 {
        let density = self.density(others);
        consts::PRESSURE_KAPPA * density
    }

    pub fn force_gracity(&self) -> Array1<f32> {
        let mut force = Array1::zeros(3);
        force[2] = -consts::GRAVITY;
        force
    }

    pub fn force_pressure(&self, others: &[Particle]) -> Array1<f32> {
        let mut force = Array1::<f32>::zeros(3);
        let p_i = self.density(others);
        for other in others {
            let distance_squared = self.distance_squared(other);
            if distance_squared < consts::DISTANCE_LIMIT_SQUARED {
                let gradient = kernels::grad_spiky(&self.position, &other.position);
                let rho_j = other.density(others);
                let p_j = other.pressure(others);
                let coeff = (p_i + p_j) / rho_j / 2.0;
                force = force + gradient * coeff;
            }
        }
        force * consts::PARTICLE_MASS
    }

    pub fn force_viscosity(&self, others: &[Particle]) -> Array1<f32> {
        let mut force = Array1::<f32>::zeros(3);
        for other in others {
            let distance_squared = self.distance_squared(other);
            if distance_squared < consts::DISTANCE_LIMIT_SQUARED {
                let distance = distance_squared.sqrt();
                let viscosity_term = kernels::laplacian_viscosity(distance);
                let rho_j = other.density(others);
                let dv = &other.velocity - &self.velocity;
                force = force + dv * viscosity_term / rho_j;
            }
        }
        force * consts::PARTICLE_MASS * consts::VISCOSITY_MU
    }

    pub fn force_surface_tension(&self, _others: &[Particle]) -> Array1<f32> {
        // TODO
        Array1::zeros(3)
    }
}

pub struct SimulationStepInput {
    pub force: Array1<f32>,
    pub density: f32,
    pub dt: f32,
}
// force and simulation
impl Particle {
    pub fn force(&self, others: &[Particle]) -> Array1<f32> {
        let mut force = self.force_gracity();
        force = force + self.force_pressure(others);
        force = force + self.force_viscosity(others);
        force = force + self.force_surface_tension(others);
        force
    }

    // Leapfrog integration
    // https://en.wikipedia.org/wiki/Leapfrog_integration
    pub fn integrate(&mut self, input: SimulationStepInput) {
        // a_i = A(x_i)
        let acc = input.force / input.density;
        // v_{i + 1/2} = v_{i - 1/2} +  a_i * dt
        let vel = &self.velocity + acc * input.dt / 2.0;
        // x_{i + 1} = x_{i} + v_{i + 1/2} * dt
        let pos = &self.position + &vel * input.dt;

        self.velocity = vel;
        self.position = pos;
    }
}
