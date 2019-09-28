#[macro_use]
extern crate log;
extern crate env_logger;

extern crate uuid;
extern crate clap;
extern crate redis;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate typetag;

use uuid::Uuid;
use redis::Commands;

pub mod world;
use world::{
    in_mem_world::InMemWorld, 
    world::{World, BoundsError}};

const WORLD_SIZE: (usize, usize)  = (16, 16);

trait Recorder {
    fn record(&mut self, data: (u64, &Box<dyn World>));
}

struct StubRecorder {
}

impl StubRecorder {
    fn new() -> Self {
        Self{
        }
    }
}

impl Recorder for StubRecorder {
    fn record(&mut self, _: (u64, &Box<dyn World>)) {}
}

struct RedisRecorder {
    conn: redis::Connection,
}

impl RedisRecorder {
    fn new() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://localhost")?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn,
        })
    }
}

impl Recorder for RedisRecorder {
    fn record(&mut self, data: (u64, &Box<dyn World>)) {
        let mut key = "gameoflife:iteration:".to_owned();
        key.push_str(&data.1.get_id().to_string());
        key.push(':');
        key.push_str(&data.0.to_string());
        self.conn.set::<String, String, String>(
            key, serde_json::to_string(&data.1).unwrap()).unwrap();
    }
}

struct GameOfLife {
    state: u64,
    world: Box<dyn World>,
    world_buffer: Box<dyn World>,
    recorder: Box<dyn Recorder>,
}

impl GameOfLife {
    fn new(recorder: Box<dyn Recorder>) -> Self {
        let world_id = Uuid::new_v4().to_string().to_owned();
        let world = Box::new(InMemWorld::new(world_id, WORLD_SIZE));
        let world_buffer = world.clone();
        Self {
            state: 0,
            world,
            world_buffer,
            recorder
        }
    }

    fn swap_buffers(&mut self) -> () {
        std::mem::swap(&mut self.world, &mut self.world_buffer);
        self.recorder.record((self.state, &self.world));
        self.state += 1;
    }

    fn simulate(&mut self, cnt: u64) -> () {
        for _ in 0..cnt {
            for i in 0usize..self.world.get_bounds().0 {
                for k in 0usize..self.world.get_bounds().1 {
                    let neighbours = self.world.get_neighbours(&(i, k));
                    let mut alive = self.world.get(&(i, k)).unwrap();
                    match neighbours {
                        2 => {},
                        3 => {
                            alive = true;
                        },
                        _ => {
                            alive = false;
                        },
                    }
                    self.world_buffer.set(&(i, k), alive).unwrap();
                }
            }
            self.swap_buffers();
        }
    }

    fn print(&self) -> std::io::Result<()> {
        let mut data = String::new();
        for i in 0..self.world.get_bounds().0 {
            let mut tmp_str = String::new();
            for k in 0..self.world.get_bounds().1 {
                match self.world.get(&(i, k)).unwrap() {
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

struct EntityFactory;
impl EntityFactory {
    fn glider(coords: (usize, usize), world: &mut Box<dyn World>) -> std::result::Result<(), BoundsError> {
        world.set(&(coords.0+2, coords.1+0), true)?;
        world.set(&(coords.0+2, coords.1+1), true)?;
        world.set(&(coords.0+2, coords.1+2), true)?;
        world.set(&(coords.0+1, coords.1+2), true)?;
        world.set(&(coords.0+0, coords.1+1), true)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Program is running.");

    let matches = clap::App::new("GameOfLife").arg(
        clap::Arg::with_name("record")
            .long("record")
            .short("r")
            .multiple(false))
        .get_matches();
    let do_record = matches.is_present("record");
    let recorder: Box<dyn Recorder> = if do_record {
        Box::new(RedisRecorder::new().unwrap())
    } else {
        Box::new(StubRecorder::new())
    };

    let mut gol = GameOfLife::new(recorder);
    EntityFactory::glider((0, 0), &mut gol.world).unwrap();

    loop {
        gol.print().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
        // glider needs 4 iterations to return to default state
        // after going forward 1 step
        gol.simulate(4);
    }
}
