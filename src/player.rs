use crate::{card::MTGCard, zone::MTGZone};


#[derive(Debug)]
pub struct MTGPlayer {
    pub battlefield: Vec<MTGCard>,
    pub command: Vec<MTGCard>,
    pub exile: Vec<MTGCard>,
    pub graveyard: Vec<MTGCard>,
    pub hand: Vec<MTGCard>,
    pub library: Vec<MTGCard>,
    pub stack: Vec<MTGCard>,
    pub extra_zones: Vec<MTGZone>
}