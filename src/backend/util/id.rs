use core::fmt;
use std::{
    fmt::Display,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rand::Rng;

use serde::{Deserialize, Serialize};

pub struct GameId(u32);

// impl GameId {
//     fn generate() -> Self {
//         Self(1)
//     }
// }

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

#[derive(Clone)]
pub struct OBGIdFactory {
    last_time: SystemTime,
    incr: u8,
}

impl OBGIdFactory {
    pub fn new() -> Self {
        Self {
            last_time: SystemTime::now(),
            incr: 0,
        }
    }

    pub fn generate(&mut self, id_type: IdType) -> OBGId {
        let since = SystemTime::now().duration_since(self.last_time).unwrap();

        if since.as_millis() == 0 {
            if self.incr == u8::MAX {
                thread::sleep(Duration::from_millis(1));
                self.incr = 0;
            } else {
                self.incr = self.incr + 1;
            }
        } else {
            self.incr = 0;
        }

        self.last_time = SystemTime::now();

        OBGId {
            time: self
                .last_time
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            id_type,
            incr: self.incr,
        }
    }
    pub fn generate_game(&mut self) -> GameId {
        let time = self
            .last_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        GameId((time << 16) | rand::thread_rng().gen::<u16>() as u32)
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OBGId {
    // Time in ms (first 48 bits)
    pub time: u64,
    // Id Type (8 bits)
    pub id_type: IdType,
    // Incremental fallback when multiple ids are created in 1ms (8 bits)
    pub incr: u8,
}

impl Into<u64> for OBGId {
    fn into(self) -> u64 {
        ((self.time as u64) << 16) | ((self.id_type as u64) << 8) | (self.incr as u64)
    }
}

impl Display for OBGId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", Into::<u64>::into(*self))
    }
}

impl From<u64> for OBGId {
    fn from(value: u64) -> Self {
        let time: u64 = (value >> 16) as u64;
        let id_type: IdType = (((value >> 8) & 0xFF) as u8).into();
        let incr: u8 = (value & 0xFF) as u8;
        Self {
            time,
            id_type,
            incr,
        }
    }
}

impl From<String> for OBGId {
    fn from(value: String) -> Self {
        let value = u64::from_str_radix(&value, 16).unwrap();
        let time: u64 = (value >> 16) as u64;
        let id_type: IdType = (((value >> 8) & 0xFF) as u8).into();
        let incr: u8 = (value & 0xFF) as u8;
        Self {
            time,
            id_type,
            incr,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum IdType {
    Unknown = 0,
    Game = 1,
    User = 2,
}

impl From<u8> for IdType {
    fn from(value: u8) -> Self {
        match value {
            1 => IdType::Game,
            2 => IdType::User,
            _ => IdType::Unknown,
        }
    }
}

mod tests {
    use super::{IdType::*, *};

    #[test]
    fn test_id() {
        let mut factory = OBGIdFactory::new();

        let types: Vec<IdType> = vec![Game, User, Unknown];

        types.iter().for_each(|id_type| {
            let id = factory.generate(*id_type);
            println!("Generated id: {} with type: {:?}", id, id.id_type);
        });
    }
}
