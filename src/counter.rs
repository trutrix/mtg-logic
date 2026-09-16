
#[derive(Debug)]
pub struct MTGCounter {
    pub count: u64,
    pub type_: MTGCounterType
}


#[derive(Debug)]
pub enum MTGCounterType {
    PlusPlus,
    MinusMinus,
    PlusMinus,
    MinusPlus,
    Poison
}