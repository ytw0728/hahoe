#[derive(Clone, Copy)]
pub struct NoiseParam {
    pub persistance: f32,
    pub lacunarity: f32,
    pub num_octaves: usize,
    pub seed: u64,
}

impl NoiseParam {
    pub fn make_noise(
        persistance: f32,
        lacunarity: f32,
        num_octaves: usize,
        seed: u64,
    ) -> NoiseParam {
        NoiseParam {
            persistance,
            lacunarity,
            num_octaves,
            seed,
        }
    }
}
