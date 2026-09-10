use crate::{card::MTGCard, zone::{MTGZone, MTGZoneType}};


#[derive(Debug)]
pub struct MTGPlayer {
    pub zones: Vec<MTGZone>
}