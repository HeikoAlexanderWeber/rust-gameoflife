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

use std::borrow::Borrow;
use uuid::Uuid;

pub mod world;
use world::{
    in_mem_world::InMemWorld, 
    world::{World, BoundsError}};
pub mod recorder;
use recorder::{
    recorder::Recorder, 
    stub_recorder::StubRecorder,
    redis_recorder::RedisRecorder};

const WORLD_SIZE: (usize, usize)  = (16, 16);

struct GameOfLife {
    state: u64,
    world: Box<dyn World>,
    world_buffer: Box<dyn World>,
    recorder: Box<dyn Recorder>,
}

impl GameOfLife {
    fn new(size: (usize, usize), recorder: Box<dyn Recorder>) -> Self {
        let world_id = Uuid::new_v4().to_string().to_owned();
        let world = Box::new(InMemWorld::new(world_id, size));
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
        self.recorder.record((self.state.to_string().as_ref(), self.world.borrow()));
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

    fn print(&self) {
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
        .arg(
            clap::Arg::with_name("size")
			.long("size")
			.short("s")
			.multiple(false)
            .takes_value(true))
        .get_matches();
    let do_record = matches.is_present("record");
	let size = match matches.value_of("size") {
        Some(s) => {
            let xy: Vec<&str> = s.split(":").collect();
            (xy[0].parse::<usize>().unwrap(), xy[1].parse::<usize>().unwrap())
        },
        None => WORLD_SIZE,
    };

    let recorder: Box<dyn Recorder> = if do_record {
        Box::new(RedisRecorder::new().unwrap())
    } else {
        Box::new(StubRecorder::new())
    };

    let mut gol = GameOfLife::new(size, recorder);
    EntityFactory::glider((0, 0), &mut gol.world).unwrap();

    loop {
        gol.print();
        std::thread::sleep(std::time::Duration::from_millis(200));
        // glider needs 4 iterations to return to default state
        // after going forward 1 step
        gol.simulate(4);
    }
}
