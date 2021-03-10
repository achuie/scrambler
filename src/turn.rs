use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone)]
pub enum Turn {
    U(TurnType),
    D(TurnType),
    R(TurnType),
    L(TurnType),
    F(TurnType),
    B(TurnType),
}

impl Distribution<Turn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turn {
        let r = rng.gen_range(0..18);
        let turn_type = match r / 6 {
            0 => TurnType::Clock,
            1 => TurnType::Prime,
            _ => TurnType::Double,
        };

        match r % 6 {
            0 => Turn::U(turn_type),
            1 => Turn::D(turn_type),
            2 => Turn::R(turn_type),
            3 => Turn::L(turn_type),
            4 => Turn::F(turn_type),
            _ => Turn::B(turn_type),
        }
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::U(tt) => write!(f, "U{}", tt),
            Turn::D(tt) => write!(f, "D{}", tt),
            Turn::R(tt) => write!(f, "R{}", tt),
            Turn::L(tt) => write!(f, "L{}", tt),
            Turn::F(tt) => write!(f, "F{}", tt),
            Turn::B(tt) => write!(f, "B{}", tt),
        }
    }
}

#[derive(Clone)]
pub enum TurnType {
    Clock,
    Prime,
    Double,
}

impl std::fmt::Display for TurnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurnType::Clock => write!(f, ""),
            TurnType::Prime => write!(f, "\'"),
            TurnType::Double => write!(f, "2"),
        }
    }
}
