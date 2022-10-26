use gui::webgl::buffer::init::{BufferDataMaker, ColorBufferDataMaker, RectangleBufferDataMaker};
use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{bitmap::Bitmap, terrain::Terrain};

pub struct UpdateTerrainSystem;

impl<'a> System<'a> for UpdateTerrainSystem {
    type SystemData = (ReadStorage<'a, Bitmap>, WriteStorage<'a, Terrain>);

    fn run(&mut self, data: Self::SystemData) {
        let (bitmap, mut terrain) = data;

        for (bitmap, terrain) in (&bitmap, &mut terrain).join() {
            let color_buffer_data_maker = ColorBufferDataMaker {};
            let color_buffer_data = color_buffer_data_maker.make_buffer_data(&bitmap.bitmap);

            let rectangle_buffer_data_maker = RectangleBufferDataMaker {};
            let rectangle_buffer_data =
                rectangle_buffer_data_maker.make_buffer_data(&bitmap.bitmap);

            terrain.color_buffer_data = color_buffer_data;
            terrain.rectangle_buffer_data = rectangle_buffer_data;
        }
    }
}
