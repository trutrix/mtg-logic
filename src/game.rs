use crate::{card::MTGCard, format::MTGFormat, player::MTGPlayer, zone::MTGZone};



pub struct MTGGame {
    pub players: Vec<MTGPlayer>,
    pub stack: Vec<MTGCard>
}


impl MTGGame {

    pub fn new(player_count: u8, format: MTGFormat) -> Self {
        let mut players = Vec::new();

        for i in 0..player_count {
            players.push(MTGPlayer { name: "DefaultName".to_string(), life: format.starting_life() as u64, counters: Vec::new(), zones: Vec::new() });
        }

        MTGGame { players , stack: Vec::new() }
    }
}