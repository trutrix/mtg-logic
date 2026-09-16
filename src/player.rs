use crate::{card::MTGCard, counter::MTGCounter, zone::{MTGZone, MTGZoneType}};


#[derive(Debug)]
pub struct MTGPlayer {
    pub name: String,
    pub life: u64,
    pub counters: Vec<MTGCounter>,
    pub zones: Vec<MTGZone>
}