use gui::webgl::buffer::fillers::{
    BufferDataFiller, ColorBufferDataFiller, RectangleBufferDataFiller,
};
use gui::webgl::buffer::update::Updater;
use specs::{Join, Read, ReadStorage, System};

pub struct RenderTerrainSystem;

impl<'a> System<'a> for RenderTerrainSystem {
    // TODO: resource (time) 사용법 전달드리고 나면, 제거하기 (여기선 필요없음.)
    type SystemData = (
        Read<'a, crate::resources::physics::time::Time>,
        ReadStorage<'a, crate::components::terrain::Terrain>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // MEMO: resource는 이렇게 쓰면 됩니다.
        let (time, terrain) = data;
        let updater = Updater {};

        for terrain in terrain.join() {
            let colorBufferDataFiller = ColorBufferDataFiller {
                buffer_data: Some(&terrain.color_buffer_data),
            };

            colorBufferDataFiller.bind_buffer();
            colorBufferDataFiller.fill_with_buffer_data();

            let rectangleBufferDataFiller = RectangleBufferDataFiller {
                buffer_data: Some(&terrain.rectangle_buffer_data),
            };

            rectangleBufferDataFiller.bind_buffer();
            rectangleBufferDataFiller.fill_with_buffer_data();
            updater.draw((terrain.rectangle_buffer_data.len() / 3) as i32);
        }
    }
}
