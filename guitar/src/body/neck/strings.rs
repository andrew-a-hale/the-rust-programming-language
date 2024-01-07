#[derive(Debug)]
pub struct Strings {
    amount: u32,
    tuning: tuning::Tunings,
    gauge: Vec<u32>
}

impl Strings {
    pub fn new(amount: u32, gauge: Vec<u32>) -> Strings {
        let tuning = tuning::Tunings::Standard(vec![tuning::Notes::E]);
        Strings { amount, tuning, gauge }
    }
}

pub mod tuning {
    #[derive(Debug)]
    pub enum Notes {
        E,
        A,
        D,
        G,
        B
    }

    #[derive(Debug)]
    pub enum Tunings {
        Standard(Vec<Notes>)
    }
}