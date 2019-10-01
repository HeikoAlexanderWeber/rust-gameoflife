use crate::world::world::{World, BoundsError};

#[derive(Clone, Serialize, Deserialize)]
pub struct InMemWorld {
    #[allow(dead_code)] // id is not needed here
    id: String,
    bounds: (usize, usize),
    grid: Vec<Vec<bool>>,
}

impl InMemWorld {
    #[allow(dead_code)]
    pub fn new(id: String, bounds: (usize, usize)) -> Self {
        let mut grid = Vec::with_capacity(bounds.0);
        let y = vec![false; bounds.1];
        for _ in 0..bounds.0 {
            grid.push(y.clone());
        }
        Self {
            id,
            bounds,
            grid
        }
    }
}

#[typetag::serde]
impl World for InMemWorld {
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_bounds(&self) -> &(usize, usize) {
        &self.bounds
    }

    fn get(&self, coords: &(usize, usize)) -> Result<bool, BoundsError> {
        let result = std::panic::catch_unwind(|| { // catches possible out-of-bounds array access
            self.grid[coords.0][coords.1]
        });
        return match result {
            Ok(v) => Ok(v),
            Err(_) => Err(BoundsError{})
        }
    }

    fn set(&mut self, coords: &(usize, usize), alive: bool) -> Result<bool, BoundsError> {
        return match self.get(coords) {
            Ok(v) => {
                self.grid[coords.0][coords.1] = alive;
                Ok(v)
            }
            Err(_) => Err(BoundsError{})
        };
    }

    fn set_span(&mut self, span: (std::ops::Range::<usize>, std::ops::Range::<usize>), alive: bool) -> Result<(), BoundsError> {
        for x in span.0.clone() {
            for y in span.1.clone() {
                match self.set(&(x, y), alive) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }

    fn get_neighbours(&self, coords: &(usize, usize)) -> u8 {
        let mut count = 0;
        for x in -1i32..2 {
            for y in -1i32..2 {
                if x == 0 && y == 0 {
                    continue;
                }
                let idx_a = (coords.0 as i32) + x;
                let idx_b = (coords.1 as i32) + y;
                if idx_a < 0 || idx_b < 0 || 
                   idx_a >= (self.bounds.0 as i32) || idx_b >= (self.bounds.1 as i32) {
                    continue;
                }
                if self.grid[idx_a as usize][idx_b as usize] {
                    count += 1;
                }
            }
        }
        count
    }
}
