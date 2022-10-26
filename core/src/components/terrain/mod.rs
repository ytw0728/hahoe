use specs::{Component, VecStorage};
use terrain::model::pixel::Pixel;
use terrain::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Terrain {
    pub bitmap: Vec<Vec<Pixel>>,
    pub mesh: Mesh,
}
