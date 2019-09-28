use std::result::Result;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct BoundsError;

pub trait World {
    fn get_id(&self) -> String;
    fn get_bounds(&self) -> (usize, usize);

    fn get(&self, coords: &(usize, usize)) -> Result<bool, BoundsError>;
    fn set(&mut self, coords: &(usize, usize), alive: bool) -> Result<bool, BoundsError>;
    fn set_span(&mut self, span: (Range::<usize>, Range::<usize>), alive: bool) -> Result<(), BoundsError>;

    fn get_neighbours(&self, coords: &(usize, usize)) -> u8;
}
