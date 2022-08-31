use std::collections::HashMap;

use super::{layer::Layer, noise_param::NoiseParam, pixel::Pixel};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum LayerType {
    Moisture,
    Height,
}

#[derive(Clone)]
pub struct Terrain {
    width: usize,
    height: usize,
    scale: f64,
    hash_layer: HashMap<LayerType, Option<Layer>>,
    map: Vec<Vec<Pixel>>,
}

impl Terrain {
    pub fn make_terrain(width: usize, height: usize, scale: f64) -> Terrain {
        Terrain {
            width,
            height,
            scale,
            hash_layer: HashMap::from([(LayerType::Moisture, None), (LayerType::Height, None)]),
            map: (0..width)
                .map(|_| {
                    (0..height)
                        .map(|_| -> Pixel { Pixel::make_dummy() })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn generate_height_layer(&mut self, noise_param: NoiseParam) {
        if let Some(height_layer) = self.hash_layer.get_mut(&LayerType::Height) {
            *height_layer = Some(Layer::make_layer(
                noise_param,
                self.width,
                self.height,
                self.scale,
            ));
        }

        let height_map = self
            .hash_layer
            .get(&LayerType::Height)
            .unwrap()
            .as_ref()
            .unwrap()
            .get_map();

        for y in 0..self.height {
            for x in 0..self.width {
                self.map[x][y].height = height_map[x][y];
            }
        }
    }

    pub fn generate_moisture_layer(&mut self, noise_param: NoiseParam) {
        if let Some(moisture_layer) = self.hash_layer.get_mut(&LayerType::Moisture) {
            *moisture_layer = Some(Layer::make_layer(
                noise_param,
                self.width,
                self.height,
                self.scale,
            ));
        }

        let moisture_map = self
            .hash_layer
            .get(&LayerType::Moisture)
            .unwrap()
            .as_ref()
            .unwrap()
            .get_map();

        for y in 0..self.height {
            for x in 0..self.width {
                self.map[x][y].moisture = moisture_map[x][y];
            }
        }
    }

    pub fn get_pixel_map(&self) -> Option<Vec<Vec<Pixel>>> {
        for layer_value in self.hash_layer.values() {
            if layer_value.is_none() {
                return None;
            }
        }

        Some(self.map.clone())
    }
}
