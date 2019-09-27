extern crate uuid;
use uuid::Uuid;

extern crate redis;
use redis::Commands;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

const WORLD_SIZE: (usize, usize)  = (128, 128);

#[derive(Clone, Serialize)]
struct World {
    id: String,
    grid: Vec<Vec<bool>>,
}

impl World {
    fn new() -> Self {
        Self::new_with_id(Uuid::new_v4().to_string())
    }

    fn new_with_id(id: String) -> Self {
        let mut grid = Vec::with_capacity(WORLD_SIZE.0);
        let y = [false; WORLD_SIZE.1].to_vec();
        for _ in 0..WORLD_SIZE.0 {
            grid.push(y.clone());
        }
        World {
            id,
            grid
        }
    }
}

trait Recorder {
    fn record(&mut self, data: (i32, World));
}

struct RedisRecorder {
    conn: redis::Connection,
}

impl RedisRecorder {
    fn new() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://localhost")?;
        let conn = client.get_connection()?;
        Ok(RedisRecorder{
            conn,
        })
    }
}

impl Recorder for RedisRecorder {
    fn record(&mut self, data: (i32, World)) {
        let mut key = "gameoflife:iteration:".to_owned();
        key.push_str(&data.1.id.to_string());
        key.push(':');
        key.push_str(&data.0.to_string());
        self.conn.set::<String, String, String>(
            key, serde_json::to_string(&data.1).unwrap()).unwrap();
    }
}

trait GameOfLife {
    fn simulate(&mut self, cnt: i32) -> ();
    fn print(&self) -> std::io::Result<()>;
}

struct InMemGameOfLife {
    state: i32,
    world: World,
    world_buffer: World,
    recorder: Box<dyn Recorder>,
}

impl InMemGameOfLife {
    fn new() -> Self {
        InMemGameOfLife{
            state: 0,
            world: World::new(),
            world_buffer: World::new(),
            recorder: Box::new(RedisRecorder::new().unwrap()),
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
                if idx_a < 0 || idx_b < 0 || idx_a >= (WORLD_SIZE.0 as i32) || idx_b >= (WORLD_SIZE.1 as i32) {
                    continue;
                }
                if map.grid[idx_a as usize][idx_b as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn swap_buffers(&mut self) -> () {
        std::mem::replace(&mut self.world, self.world_buffer.clone());
        self.recorder.record((self.state, self.world.clone()));
        self.state += 1;
    }
}

impl GameOfLife for InMemGameOfLife {
    fn simulate(&mut self, cnt: i32) -> () {
        for _ in 0..cnt {
            for i in 0usize..WORLD_SIZE.0 {
                for k in 0usize..WORLD_SIZE.1 {
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

    fn print(&self) -> std::io::Result<()> {
        let mut data = String::new();
        for i in 0..WORLD_SIZE.0 {
            let mut tmp_str = String::new();
            for k in 0..WORLD_SIZE.1 {
                match self.world.grid[i][k] {
                    true => tmp_str.push('•'),
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
        let range_x = 0..WORLD_SIZE.0;
        let range_y = 0..WORLD_SIZE.1;
        if !range_x.contains(&coords.0) || 
           !range_x.contains(&(coords.0 +2)) || 
           !range_y.contains(&coords.1) || 
           !range_y.contains(&(coords.1 +2)) {
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

    let mut gol = InMemGameOfLife::new();
    EntityFactory::glider((0, 0), &mut gol.world).unwrap();

    loop {
        gol.print().unwrap();
        //std::thread::sleep(std::time::Duration::from_millis(10));
        // glider needs 4 iterations to return to default state
        // after going forward 1 step
        gol.simulate(4);
    }
}
