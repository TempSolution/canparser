use std::{collections::HashMap, fs::File, io::Read};

use can_dbc::Message;

pub mod payload;



pub fn read_dbc(f: &mut File) -> HashMap<u32, Message> {
    let mut buffer = Vec::<u8>::new();
    f.read_to_end(&mut buffer).unwrap();

    let dbc = can_dbc::DBC::from_slice(&buffer).expect("Failed to parse dbc file");
    let mut db = HashMap::<u32, Message>::new();
    for message in dbc.messages() {
        db.insert(message.message_id().0, message.clone());
    }
    return db;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
