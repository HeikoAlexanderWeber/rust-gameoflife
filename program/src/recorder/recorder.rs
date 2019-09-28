use crate::world::world::World;

pub trait Recorder {
    fn record(&mut self, world: (&str, &dyn World));
}
