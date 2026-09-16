


pub enum MTGFormat {
    Standard,
    Commander
}


impl MTGFormat {
    pub fn starting_life(&self) -> u8 {
        match self {
            MTGFormat::Standard => 20,
            MTGFormat::Commander => 40,
        }
    }
}