use crate::model;
use model::noise_param::*;
use model::pixel::*;
use model::terrain::*;

use rust_3d::*;

pub enum Result<T> {
    Ok(T),
    Err(String),
}

pub type Mesh = Mesh3D<Point3D, PointCloud3D<Point3D>, Vec<usize>>;

pub fn gen_bitmap() -> Option<Vec<Vec<Pixel>>> {
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

pub fn gen_mesh() -> Option<Mesh> {
    let mut mesh = Mesh3D::default();
    let terrain = gen_bitmap();
    if let Some(terrain) = terrain {
        let width = terrain.len();
        let height = terrain[0].len();
        for (r, row) in terrain.iter().enumerate() {
            for (c, pixel) in row.iter().enumerate() {
                let x = r as f64;
                let y = c as f64;
                let z = pixel.height;
                mesh.add_vertex(Point3D::new(x, y, z));
            }
        }
        for r in 0..width - 1 {
            for c in 0..height - 1 {
                let top_left = r * width + c;
                let top_right = top_left + 1;
                let bottom_left = top_left + width;
                let bottom_right = bottom_left + 1;
                let top_left = VId { val: top_left };
                let top_right = VId { val: top_right };
                let bottom_left = VId { val: bottom_left };
                let bottom_right = VId { val: bottom_right };
                mesh.try_add_connection(top_left, bottom_right, top_right)
                    .unwrap_or(FId { val: 0 });
                // 아무 의미 없고, 워닝 누르기 위한 unwrap
                mesh.try_add_connection(top_left, bottom_left, bottom_right)
                    .unwrap_or(FId { val: 0 });
            }
        }
        Some(mesh)
    } else {
        None
    }
}
