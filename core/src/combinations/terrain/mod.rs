use specs::{Builder, DispatcherBuilder, WorldExt};

use crate::{combinations::Combination, components, systems};

pub struct TerrainCombination;

impl Combination for TerrainCombination {
    fn init<'world, 'a, 'b>(world: &'world mut specs::World) -> specs::Dispatcher<'a, 'b> {
        // components (register)
        world.register::<components::terrain::Terrain>();

        // MEMO: Terrain에서 entity는 쓰이지 않으나, 예시로 남겨둡니다.
        // // entities (build)
        let terrain = world
            .create_entity()
            .with(components::terrain::Terrain {
                bitmap: terrain::test_runner1().unwrap(),
            })
            .build();

        // system들이 새팅된 dispatcher 를 반환한다. 현재 RenderTerrainSystem만 세팅되어있다.
        DispatcherBuilder::new()
            .with(
                systems::renders::terrain::RenderTerrainSystem {},
                "render_terrain_system",
                &[],
            )
            .build()
    }
}
