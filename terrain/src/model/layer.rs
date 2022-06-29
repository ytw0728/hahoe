use super::noise_param::NoiseParam;
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{prelude::StdRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct Layer {
    noise_param: NoiseParam,
    octave_offsets: Vec<(f64, f64)>,
    perlin: OpenSimplex,
    noise_map: Vec<Vec<f64>>,
}

impl Layer {
    pub fn make_layer(noise_param: NoiseParam, width: usize, height: usize, scale: f64) -> Layer {
        let mut layer = Layer {
            noise_param: noise_param,
            octave_offsets: Vec::new(),
            perlin: OpenSimplex::new(),
            noise_map: Vec::new(),
        };

        layer.generate_perlin();
        layer.generate_octave();
        layer.generate_map(width, height, scale);
        layer
    }

    pub fn get_map(&self) -> Vec<Vec<f64>> {
        self.noise_map.clone()
    }

    fn generate_perlin(&mut self) {
        self.perlin.set_seed(self.noise_param.seed as u32);
    }

    fn generate_octave(&mut self) {
        let mut rng: StdRng = StdRng::seed_from_u64(self.noise_param.seed as u64);
        self.octave_offsets = (0..self.noise_param.num_octaves)
            .map(|_| {
                let x: f64 = rng.gen_range(-100000f64..100000f64);
                let y: f64 = rng.gen_range(-100000f64..100000f64);
                (x, y)
            })
            .collect();
    }

    fn generate_map(&mut self, width: usize, height: usize, scale: f64) {
        self.noise_map = (0..width)
            .map(|_| (0..height).map(|_| 0f64).collect())
            .collect();

        for y in 0..height {
            for x in 0..width {
                let mut amplitude = 1f64;
                let mut frequency = 1f64;
                let mut noise_height = 0f64;

                for idx_octave in 0..self.noise_param.num_octaves {
                    let sample_x =
                        (x as f64) / scale * frequency + self.octave_offsets[idx_octave].0;
                    let sample_y =
                        (y as f64) / scale * frequency + self.octave_offsets[idx_octave].1;

                    let perlin_value = self.perlin.get([sample_x, sample_y]);
                    noise_height += perlin_value * amplitude;

                    amplitude *= self.noise_param.persistance as f64;
                    frequency *= self.noise_param.lacunarity as f64;
                }
                self.noise_map[x][y] = noise_height;
            }
        }
    }
}
