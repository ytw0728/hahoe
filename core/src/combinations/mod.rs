pub mod terrain;
use specs::{World, Dispatcher};


pub trait Combination {
    fn init<'world, 'a, 'b>(world: &'world mut World) -> Dispatcher<'a, 'b>;
}
