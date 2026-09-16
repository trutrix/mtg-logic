use crate::{card::MTGCard, player::MTGPlayer};


#[derive(Debug)]
pub enum MTGZoneType {
    Battlefield,
    Command,
    Exile,
    Graveyard,
    Hand,
    Library,
    Stack,
    // Ante, not sure who would actually want this
}

#[derive(Debug)]
pub struct MTGZone {
    pub type_: MTGZoneType,
    pub cards: Vec<MTGCard>
}