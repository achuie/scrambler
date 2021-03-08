use colored::Colorize;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

fn main() {
    let cube = Cube::new();
    cube.mv(Turn::U(TurnType::Clock)).mv(Turn::B(TurnType::Double)).print();
}

fn generate_random_turns(n_turns: u32) -> Vec<Turn> {
    (0..n_turns).map(|_| rand::random()).collect()
}

struct Cube {
    green: Face,
    red: Face,
    blue: Face,
    orange: Face,
    white: Face,
    yellow: Face,
}

impl Cube {
    fn new() -> Self {
        Cube {
            green: Face::new(Color::Green),
            red: Face::new(Color::Red),
            blue: Face::new(Color::Blue),
            orange: Face::new(Color::Orange),
            white: Face::new(Color::White),
            yellow: Face::new(Color::Yellow),
        }
    }

    fn print(&self) {
        for row in &self.white.tiles {
            print!("        ");
            for t in row {
                print!("{}", t);
            }
            print!("\n");
        }
        for row in 0..3 {
            print!(" ");
            for &face in [&self.orange, &self.green, &self.red, &self.blue].iter() {
                for t in &face.tiles[row] {
                    print!("{}", t);
                }
                print!(" ");
            }
            print!("\n");
        }
        for row in &self.yellow.tiles {
            print!("        ");
            for t in row {
                print!("{}", t);
            }
            print!("\n");
        }
    }

    fn looped_update(
        to_update: [&Face; 4],
        update_sections: [Triplet; 4],
        turn_type: &TurnType,
    ) -> Vec<Face> {
        to_update
            .iter()
            .enumerate()
            .map(|(i, &face)| {
                let other_idx = match turn_type {
                    TurnType::Clock => (i + 1) % 4,
                    TurnType::Prime => (i - 1) % 4,
                    TurnType::Double => (i + 2) % 4,
                };

                face.update_triplet(
                    &update_sections[i],
                    match turn_type {
                        TurnType::Double => to_update[other_idx]
                            .get_triplet(&update_sections[other_idx])
                            .iter()
                            .rev()
                            .map(|c| c.clone())
                            .collect::<Vec<Color>>(),
                        _ => to_update[other_idx].get_triplet(&update_sections[other_idx]),
                    }
                    .as_slice(),
                )
            })
            .collect()
    }

    fn mv(&self, turn: Turn) -> Self {
        match turn {
            Turn::U(turn_type) => {
                let to_update = [&self.green, &self.red, &self.blue, &self.orange];
                let update_sections = [Triplet::Top, Triplet::Top, Triplet::Top, Triplet::Top];
                let updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    green: updated[0].clone(),
                    red: updated[1].clone(),
                    blue: updated[2].clone(),
                    orange: updated[3].clone(),
                    white: self.white.rotate(&turn_type),
                    yellow: self.yellow.clone(),
                }
            },
            Turn::D(turn_type) => {
                let to_update = [&self.green, &self.orange, &self.blue, &self.red];
                let update_sections =
                    [Triplet::Bottom, Triplet::Bottom, Triplet::Bottom, Triplet::Bottom];
                let updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    green: updated[0].clone(),
                    red: updated[3].clone(),
                    blue: updated[2].clone(),
                    orange: updated[1].clone(),
                    white: self.white.clone(),
                    yellow: self.yellow.rotate(&turn_type),
                }
            },
            Turn::R(turn_type) => {
                let to_update = [&self.green, &self.yellow, &self.blue, &self.white];
                let update_sections =
                    [Triplet::Right, Triplet::Right, Triplet::Left, Triplet::Right];
                let updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    green: updated[0].clone(),
                    red: self.red.rotate(&turn_type),
                    blue: updated[2].clone(),
                    orange: self.orange.clone(),
                    white: updated[3].clone(),
                    yellow: updated[1].clone(),
                }
            },
            Turn::L(turn_type) => {
                let to_update = [&self.green, &self.white, &self.blue, &self.yellow];
                let update_sections = [Triplet::Left, Triplet::Left, Triplet::Right, Triplet::Left];
                let updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    green: updated[0].clone(),
                    red: self.red.clone(),
                    blue: updated[2].clone(),
                    orange: self.orange.rotate(&turn_type),
                    white: updated[1].clone(),
                    yellow: updated[3].clone(),
                }
            },
            Turn::F(turn_type) => {
                let to_update = [&self.white, &self.orange, &self.yellow, &self.red];
                let update_sections =
                    [Triplet::Bottom, Triplet::Right, Triplet::Top, Triplet::Left];
                let updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    green: self.green.rotate(&turn_type),
                    red: updated[3].clone(),
                    blue: self.blue.clone(),
                    orange: updated[1].clone(),
                    white: updated[0].clone(),
                    yellow: updated[2].clone(),
                }
            },
            Turn::B(turn_type) => {
                let to_update = [&self.white, &self.red, &self.yellow, &self.orange];
                let update_sections =
                    [Triplet::Top, Triplet::Right, Triplet::Bottom, Triplet::Left];
                let updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    green: self.green.clone(),
                    red: updated[1].clone(),
                    blue: self.blue.rotate(&turn_type),
                    orange: updated[3].clone(),
                    white: updated[0].clone(),
                    yellow: updated[2].clone(),
                }
            },
        }
    }
}

#[derive(Clone)]
struct Face {
    tiles: Vec<Vec<Color>>,
}

impl Face {
    fn new(color: Color) -> Self {
        Face { tiles: vec![vec!(color; 3); 3] }
    }

    fn get_triplet(&self, section: &Triplet) -> Vec<Color> {
        match section {
            Triplet::Top => self.tiles[0].clone(),
            Triplet::Right => self.tiles.iter().map(|row| row[2].clone()).collect(),
            Triplet::Bottom => self.tiles[2].clone(),
            Triplet::Left => self.tiles.iter().map(|row| row[0].clone()).collect(),
        }
    }

    fn update_triplet(&self, section: &Triplet, cubies: &[Color]) -> Face {
        Face {
            tiles: {
                match section {
                    Triplet::Top => {
                        vec![cubies.to_vec(), self.tiles[1].clone(), self.tiles[2].clone()]
                    },
                    Triplet::Right => {
                        let mut tile_array = self.tiles.clone();
                        for i in 0..3 {
                            tile_array[i][2] = cubies[i].clone();
                        }

                        tile_array
                    },
                    Triplet::Bottom => {
                        vec![self.tiles[0].clone(), self.tiles[1].clone(), cubies.to_vec()]
                    },
                    Triplet::Left => {
                        let mut tile_array = self.tiles.clone();
                        for i in 0..3 {
                            tile_array[i][0] = cubies[i].clone();
                        }

                        tile_array
                    },
                }
            },
        }
    }

    fn rotate(&self, turn_type: &TurnType) -> Face {
        Face {
            tiles: {
                let mut tile_array = vec![vec!(self.tiles[1][1].clone(); 3); 3];
                match turn_type {
                    TurnType::Clock => {
                        // 00 01 02    20 10 00
                        // 10 11 12 -> 21 11 01
                        // 20 21 22    22 12 02
                        for j in 0..3 {
                            for i in (0..3).rev() {
                                tile_array[j][2 - i] = self.tiles[i][j].clone();
                            }
                        }
                    },
                    TurnType::Prime => {
                        // 00 01 02    02 12 22
                        // 10 11 12 -> 01 11 21
                        // 20 21 22    00 10 20
                        for j in (0..3).rev() {
                            for i in 0..3 {
                                tile_array[2 - j][i] = self.tiles[i][j].clone();
                            }
                        }
                    },
                    TurnType::Double => {
                        // 00 01 02    22 21 20
                        // 10 11 12 -> 12 11 10
                        // 20 21 22    02 01 00
                        for i in (0..3).rev() {
                            for j in (0..3).rev() {
                                tile_array[2 - i][2 - j] = self.tiles[i][j].clone();
                            }
                        }
                    },
                }

                tile_array
            },
        }
    }
}

#[derive(Clone)]
enum Triplet {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone)]
enum Color {
    Green,
    Red,
    Blue,
    Orange,
    White,
    Yellow,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Green => write!(f, "{}", "\u{2588}\u{2588}".green()),
            Color::Red => write!(f, "{}", "\u{2588}\u{2588}".red()),
            Color::Blue => write!(f, "{}", "\u{2588}\u{2588}".blue()),
            Color::Orange => write!(f, "{}", "\u{2588}\u{2588}".truecolor(255, 102, 0)),
            Color::White => write!(f, "{}", "\u{2588}\u{2588}".bright_white()),
            Color::Yellow => write!(f, "{}", "\u{2588}\u{2588}".yellow()),
        }
    }
}

enum Turn {
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

enum TurnType {
    Clock,
    Prime,
    Double,
}
