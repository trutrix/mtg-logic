use crate::card::MTGCard;


#[derive(Debug)]
pub enum MTGZoneType {
    Battlefield,
    Command,
    Exile,
    Graveyard,
    Hand,
    Library,
    Stack
}

#[derive(Debug)]
pub struct MTGZone {
    pub type_: MTGZoneType,
    pub cards: Vec<MTGCard>
}