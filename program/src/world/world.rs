#[derive(Debug, Clone)]
pub struct BoundsError;

pub trait World {
    fn get_id(&self) -> String;
    fn get_bounds(&self) -> (usize, usize);

    fn get(&self, coords: &(usize, usize)) -> std::result::Result<bool, BoundsError>;
    fn set(&mut self, coords: &(usize, usize), alive: bool) -> std::result::Result<bool, BoundsError>;
    fn set_span(&mut self, span: (std::ops::Range::<usize>, std::ops::Range::<usize>), alive: bool) -> std::result::Result<(), BoundsError>;

    fn get_neighbours(&self, coords: &(usize, usize)) -> u8;
}
