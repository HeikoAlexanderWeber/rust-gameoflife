use crate::recorder::recorder::Recorder;
use crate::world::world::World;

use redis::Commands;

pub struct RedisRecorder {
    conn: redis::Connection,
}

impl RedisRecorder {
    pub fn new() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://localhost")?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn,
        })
    }
}

impl Recorder for RedisRecorder {
    fn record(&mut self, data: (&str, &dyn World)) {
        let mut key = "gameoflife:iteration:".to_owned();
        key.push_str(&data.1.get_id().to_string());
        key.push(':');
        key.push_str(&data.0.to_string());
        self.conn.set::<String, String, String>(
            key, serde_json::to_string(&data.1).unwrap()).unwrap();
    }
}
