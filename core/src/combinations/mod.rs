pub mod terrain;

use specs::{Dispatcher, World};

pub trait Combination {
    fn init<'world, 'a, 'b>(world: &'world mut World) -> Dispatcher<'a, 'b>;
}
