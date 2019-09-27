#[macro_use]
extern crate log;
extern crate env_logger;

const WORLD_SIZE: usize = 64;

#[derive(Copy, Clone)]
struct World {
    grid: [[bool; WORLD_SIZE]; WORLD_SIZE],
}

impl World {
    fn new() -> Self {
        World {
            grid: [[false; WORLD_SIZE]; WORLD_SIZE],
        }
    }
}

struct Recorder {
    data: Vec<(i32, World)>,
}

impl Recorder {
    fn new() -> Self {
        Recorder{
            data: Vec::new(),
        }
    }

    fn record(&mut self, data: (i32, World)) {
        self.data.push(data);
    }
}

struct GameOfLife {
    state: i32,
    world: World,
    world_buffer: World,
    recorder: Recorder,
}

impl GameOfLife {
    fn new() -> Self {
        GameOfLife{
            state: 0,
            world: World::new(),
            world_buffer: World::new(),
            recorder: Recorder::new(),
        }

    }

    fn get_neighbours(&self, coords: (usize, usize), map: &World) -> u8 {
        let mut count = 0;
        for x in -1i32..2 {
            for y in -1i32..2 {
                if x == 0 && y == 0 {
                    continue;
                }
                let idx_a = (coords.0 as i32) + x;
                let idx_b = (coords.1 as i32) + y;
                if idx_a < 0 || idx_b < 0 || idx_a >= (WORLD_SIZE as i32) || idx_b >= (WORLD_SIZE as i32) {
                    continue;
                }
                if map.grid[idx_a as usize][idx_b as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn simulate(&mut self, cnt: i32) -> () {
        for _ in 0..cnt {
            for i in 0usize..WORLD_SIZE {
                for k in 0usize..WORLD_SIZE {
                    let neighbours = self.get_neighbours((i, k), &self.world);
                    let mut alive = self.world.grid[i][k];
                    match neighbours {
                        0..=1 => {
                            alive = false;
                        },
                        2 => {},
                        3 => {
                            alive = true;
                        },
                        _ => {
                            alive = false;
                        },
                    }
                    self.world_buffer.grid[i][k] = alive;
                }
            }
            self.swap_buffers();
        }
    }

    fn swap_buffers(&mut self) -> () {
        std::mem::replace(&mut self.world, self.world_buffer);
        self.recorder.record((self.state, self.world.clone()));
        self.state += 1;
    }

    fn print(&self) -> std::io::Result<()> {
        let mut data = String::new();
        for i in 0..WORLD_SIZE {
            let mut tmp_str = String::new();
            for k in 0..WORLD_SIZE {
                match self.world.grid[i][k] {
                    true => tmp_str.push('o'),
                    false => tmp_str.push(' '),
                }
            }
            if tmp_str.trim().len() != 0 {
                data.push_str(&tmp_str);
                data.push('\n');
            }
        }
        
        println!("{}", data);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct WorldBoundsError;

struct EntityFactory {
}

impl EntityFactory {
    fn glider(coords: (usize, usize), world: &mut World) -> std::result::Result<(), WorldBoundsError> {
        let range = 0..WORLD_SIZE;
        if !range.contains(&coords.0) || !range.contains(&(coords.0 +2)) || !range.contains(&coords.1) || !range.contains(&(coords.1 +2)) {
            return Err(WorldBoundsError{});
        }
        world.grid[coords.0+2][coords.1+0] = true;
        world.grid[coords.0+2][coords.1+1] = true;
        world.grid[coords.0+2][coords.1+2] = true;
        world.grid[coords.0+1][coords.1+2] = true;
        world.grid[coords.0+0][coords.1+1] = true;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Program is running.");

    let mut gol = GameOfLife::new();
    EntityFactory::glider((0, 0), &mut gol.world).unwrap();

    loop {
        gol.print().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        // glider needs 4 iterations to return to default state
        // after going forward 1 step
        gol.simulate(4);
    }
}
