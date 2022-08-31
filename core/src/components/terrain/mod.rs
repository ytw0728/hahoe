use specs::{VecStorage, Component};
use terrain::model::pixel::Pixel;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Terrain {
    pub bitmap: Vec<Vec<Pixel>>
}
