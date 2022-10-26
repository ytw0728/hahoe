use specs::{Builder, DispatcherBuilder, WorldExt};

use crate::{combinations::Combination, components, systems};

pub struct TerrainCombination;

impl Combination for TerrainCombination {
    fn init<'world, 'a, 'b>(world: &'world mut specs::World) -> specs::Dispatcher<'a, 'b> {
        // components (register)
        world.register::<components::bitmap::Bitmap>();
        world.register::<components::terrain::Terrain>();

        // MEMO: Terrain에서 entity는 쓰이지 않으나, 예시로 남겨둡니다.
        // // entities (build)

        world
            .create_entity()
            .with(components::bitmap::Bitmap {
                bitmap: terrain::test_runner1().unwrap(),
            })
            .with(components::terrain::Terrain {
                rectangle_buffer_data: vec![],
                color_buffer_data: vec![],
            })
            .build();

        // system들이 새팅된 dispatcher 를 반환한다.
        DispatcherBuilder::new()
            .with(
                systems::updates::terrain::UpdateTerrainSystem,
                "update_terrain_system",
                &[],
            )
            .with(
                systems::renders::terrain::RenderTerrainSystem {},
                "render_terrain_system",
                &["update_terrain_system"],
            )
            .build()
    }
}
