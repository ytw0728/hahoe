use model::attributes::{traits::*, *};
use model::noise_param::NoiseParam;
use model::pixel::Pixel;
use model::terrain::Terrain;

mod model;

pub fn test_runner1() -> Option<Vec<Vec<Pixel>>> {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    const SCALE: f64 = (WIDTH + HEIGHT) as f64 * 0.1347;

    let mut terrain = Terrain::make_terrain(WIDTH, HEIGHT, SCALE);

    let noise_height_layer = NoiseParam::make_noise(0.5, 2f32, 3, 2400);
    let noise_moisture_layer = NoiseParam::make_noise(0.25, 2f32, 3, 2400);

    terrain.generate_height_layer(noise_height_layer);
    terrain.generate_moisture_layer(noise_moisture_layer);

    terrain.get_pixel_map()
}

pub fn generate_habitality_map() -> Option<Vec<Vec<Habitability>>> {
    let map = test_runner1();
    if let Some(map) = map {
        Some(
            map.iter()
                .map(|row| {
                    row.iter()
                        .map(|pixel| Habitability::interpolate(pixel.height))
                        .collect()
                })
                .collect(),
        )
    } else {
        None
    }
}

pub fn generate_height_map() -> Option<Vec<Vec<Height>>> {
    let map = test_runner1();
    let height_interpolater = Height::interpolater(3f64, 24f64);

    if let Some(map) = map {
        Some(
            map.iter()
                .map(|row| {
                    row.iter()
                        .map(|pixel| height_interpolater(pixel.height))
                        .collect()
                })
                .collect(),
        )
    } else {
        None
    }
}
