use specs::{Component, VecStorage};
use terrain::model::pixel::Pixel;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Bitmap {
    pub bitmap: Vec<Vec<Pixel>>,
}
