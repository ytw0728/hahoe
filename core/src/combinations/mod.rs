pub mod terrain;

use std::rc::Rc;

use gui;
use specs::{World, Component, System, Entity};


trait Combination<ComponentStorage, SystemData> {
    fn init(self, world: &World, basics: Rc<gui::GuiBasics>) -> CombinationRunnerWrapper<ComponentStorage, SystemData>;
}

trait CombinationRunner {
    fn run(self, world: &World) -> ();
}

// TODO: 저 components / systems를 Vector에 넣고 싶어요...
struct CombinationRunnerWrapper<'a, ComponentStorage, SystemData> {
    runner: Box<dyn CombinationRunner>,
    entities: Vec<Entity>,
    components: Vec<dyn Component<Storage = ComponentStorage>>,
    systems: Vec<dyn System<'a, SystemData = SystemData>>,
}

impl<'a, ComponentStorage, SystemData> CombinationRunner for CombinationRunnerWrapper<'a, ComponentStorage, SystemData> {
    fn run(self, world: &World) -> () {
        self.runner.run(world);
    }
}