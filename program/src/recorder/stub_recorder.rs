use crate::recorder::recorder::Recorder;
use crate::world::world::World;

pub struct StubRecorder;

impl StubRecorder {
    pub fn new() -> Self {
        StubRecorder{}
    }
}

impl Recorder for StubRecorder {
    fn record(&mut self, _: (&str, &dyn World)) {}
}
