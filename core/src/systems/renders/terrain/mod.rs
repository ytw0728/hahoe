use gui::webgl::buffer::fillers::{
    BufferDataFiller, ColorBufferDataFiller, RectangleBufferDataFiller,
};
use gui::webgl::buffer::init::{BufferDataMaker, ColorBufferDataMaker, RectangleBufferDataMaker};
use gui::webgl::buffer::update::Updater;
use specs::{Read, ReadStorage, System};

pub struct RenderTerrainSystem;

const CANVAS_ID: &str = "canvas";

impl<'a> System<'a> for RenderTerrainSystem {
    // TODO: resource (time) 사용법 전달드리고 나면, 제거하기 (여기선 필요없음.)
    type SystemData = (
        Read<'a, crate::resources::physics::time::Time>,
        ReadStorage<'a, crate::components::terrain::Terrain>,
    );
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        // MEMO: resource는 이렇게 쓰면 됩니다.
        let (time, terrain) = data;
        let updater = Updater {};

        for terrain in terrain.join() {
            let colorBufferDataMaker = ColorBufferDataMaker {};
            let colorBufferData = colorBufferDataMaker.make_buffer_data(&terrain.bitmap);
            let colorBufferDataFiller = ColorBufferDataFiller {
                buffer_data: Some(colorBufferData),
            };

            colorBufferDataFiller.bind_buffer();
            colorBufferDataFiller.fill_with_buffer_data();

            let rectangleBufferDataMaker = RectangleBufferDataMaker {};
            let rectangleBufferData = rectangleBufferDataMaker.make_buffer_data(&terrain.bitmap);
            let rectangleBufferDataLength = rectangleBufferData.len();
            let rectangleBufferDataFiller = RectangleBufferDataFiller {
                buffer_data: Some(rectangleBufferData),
            };

            rectangleBufferDataFiller.bind_buffer();
            rectangleBufferDataFiller.fill_with_buffer_data();
            updater.draw((rectangleBufferDataLength / 3) as i32);
        }
    }
}
