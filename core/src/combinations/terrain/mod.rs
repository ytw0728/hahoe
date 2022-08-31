use std::rc::Rc;

use specs::{System, Read};

use crate::{combinations::{ Combination, CombinationRunner }, components, systems};

use super::CombinationRunnerWrapper;


struct TerrainCombination;
impl <ComponentStorage, SystemData> Combination<ComponentStorage, SystemData> for TerrainCombination {
    fn init(self, world: &specs::World, basics: std::rc::Rc<gui::GuiBasics>) -> Box<dyn CombinationRunner> {

        // components (register)
        world.register::<components::terrain::Terrain>();

        // entities (build)
        let terrain = world.create_entity().with(components::terrain::Terrain { bitmap: terrain::test_runner1().unwrap() }).build();

        // systems (init)
        let mut render_terrain_system = systems::renders::terrain::RenderTerrainSystem { basics: Rc::clone(&basics) };
        let mut other_system = OtherSystem {};
        CombinationRunnerWrapper {
            runner: Box::new(TerrainCombinationRunner {}),
            components: vec![],
            systems: vec![render_terrain_system, other_system],
            entities: vec![],
        }
    }
}
struct TerrainCombinationRunner;
impl CombinationRunner for TerrainCombinationRunner {
    fn run(self, world: &specs::World) -> () {
        // 
    }
}


struct OtherSystem{}
impl<'a> System<'a> for OtherSystem {
    type SystemData = (Read<'a, crate::resources::physics::time::Time>);
    fn run(&mut self, data: Self::SystemData) {
        let (time) = data;
        time;
    }    
}