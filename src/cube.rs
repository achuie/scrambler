use crate::turn::{Turn, TurnType};
use colored::Colorize;
use std::mem::discriminant;

pub struct Cube {
    moves: Vec<Turn>,
    green: Face,
    red: Face,
    blue: Face,
    orange: Face,
    white: Face,
    yellow: Face,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            moves: vec![],
            green: Face::new(Color::Green),
            red: Face::new(Color::Red),
            blue: Face::new(Color::Blue),
            orange: Face::new(Color::Orange),
            white: Face::new(Color::White),
            yellow: Face::new(Color::Yellow),
        }
    }

    pub fn print(&self) {
        println!();
        for t in &self.moves {
            print!(" {}", t);
        }
        print!("\n\n");
        for row in &self.white.tiles {
            print!("        ");
            for t in row {
                print!("{}", t);
            }
            println!();
        }
        println!();
        for row in 0..3 {
            print!(" ");
            for &face in [&self.orange, &self.green, &self.red, &self.blue].iter() {
                for t in &face.tiles[row] {
                    print!("{}", t);
                }
                print!(" ");
            }
            println!();
        }
        println!();
        for row in &self.yellow.tiles {
            print!("        ");
            for t in row {
                print!("{}", t);
            }
            println!();
        }
    }

    fn looped_update(
        to_update: [&Face; 4],
        update_sections: [Triplet; 4],
        turn_type: &TurnType,
    ) -> Vec<Option<Face>> {
        to_update
            .iter()
            .enumerate()
            .map(|(i, &face)| {
                let other_idx = match turn_type {
                    TurnType::Clock => (i + 1) % 4,
                    TurnType::Prime => ((((i as isize - 1) % 4) + 4) % 4) as usize,
                    TurnType::Double => (i + 2) % 4,
                };

                Some(face.update_triplet(
                    &update_sections[i],
                    &update_sections[other_idx],
                    to_update[other_idx].get_triplet(&update_sections[other_idx]).as_slice(),
                ))
            })
            .collect()
    }

    pub fn mv(&self, turn: Turn) -> Self {
        match turn {
            Turn::U(turn_type) => {
                let to_update = [&self.green, &self.red, &self.blue, &self.orange];
                let update_sections = [Triplet::Top, Triplet::Top, Triplet::Top, Triplet::Top];
                let mut updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    moves: self
                        .moves
                        .iter()
                        .chain(vec![Turn::U(turn_type.clone())].iter())
                        .map(|t| t.clone())
                        .collect(),
                    green: updated[0].take().unwrap(),
                    red: updated[1].take().unwrap(),
                    blue: updated[2].take().unwrap(),
                    orange: updated[3].take().unwrap(),
                    white: self.white.rotate(&turn_type),
                    yellow: self.yellow.clone(),
                }
            },
            Turn::D(turn_type) => {
                let to_update = [&self.green, &self.orange, &self.blue, &self.red];
                let update_sections =
                    [Triplet::Bottom, Triplet::Bottom, Triplet::Bottom, Triplet::Bottom];
                let mut updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    moves: self
                        .moves
                        .iter()
                        .chain(vec![Turn::D(turn_type.clone())].iter())
                        .map(|t| t.clone())
                        .collect(),
                    green: updated[0].take().unwrap(),
                    red: updated[3].take().unwrap(),
                    blue: updated[2].take().unwrap(),
                    orange: updated[1].take().unwrap(),
                    white: self.white.clone(),
                    yellow: self.yellow.rotate(&turn_type),
                }
            },
            Turn::R(turn_type) => {
                let to_update = [&self.green, &self.yellow, &self.blue, &self.white];
                let update_sections =
                    [Triplet::Right, Triplet::Right, Triplet::Left, Triplet::Right];
                let mut updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    moves: self
                        .moves
                        .iter()
                        .chain(vec![Turn::R(turn_type.clone())].iter())
                        .map(|t| t.clone())
                        .collect(),
                    green: updated[0].take().unwrap(),
                    red: self.red.rotate(&turn_type),
                    blue: updated[2].take().unwrap(),
                    orange: self.orange.clone(),
                    white: updated[3].take().unwrap(),
                    yellow: updated[1].take().unwrap(),
                }
            },
            Turn::L(turn_type) => {
                let to_update = [&self.green, &self.white, &self.blue, &self.yellow];
                let update_sections = [Triplet::Left, Triplet::Left, Triplet::Right, Triplet::Left];
                let mut updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    moves: self
                        .moves
                        .iter()
                        .chain(vec![Turn::L(turn_type.clone())].iter())
                        .map(|t| t.clone())
                        .collect(),
                    green: updated[0].take().unwrap(),
                    red: self.red.clone(),
                    blue: updated[2].take().unwrap(),
                    orange: self.orange.rotate(&turn_type),
                    white: updated[1].take().unwrap(),
                    yellow: updated[3].take().unwrap(),
                }
            },
            Turn::F(turn_type) => {
                let to_update = [&self.white, &self.orange, &self.yellow, &self.red];
                let update_sections =
                    [Triplet::Bottom, Triplet::Right, Triplet::Top, Triplet::Left];
                let mut updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    moves: self
                        .moves
                        .iter()
                        .chain(vec![Turn::F(turn_type.clone())].iter())
                        .map(|t| t.clone())
                        .collect(),
                    green: self.green.rotate(&turn_type),
                    red: updated[3].take().unwrap(),
                    blue: self.blue.clone(),
                    orange: updated[1].take().unwrap(),
                    white: updated[0].take().unwrap(),
                    yellow: updated[2].take().unwrap(),
                }
            },
            Turn::B(turn_type) => {
                let to_update = [&self.white, &self.red, &self.yellow, &self.orange];
                let update_sections =
                    [Triplet::Top, Triplet::Right, Triplet::Bottom, Triplet::Left];
                let mut updated = Cube::looped_update(to_update, update_sections, &turn_type);

                Cube {
                    moves: self
                        .moves
                        .iter()
                        .chain(vec![Turn::B(turn_type.clone())].iter())
                        .map(|t| t.clone())
                        .collect(),
                    green: self.green.clone(),
                    red: updated[1].take().unwrap(),
                    blue: self.blue.rotate(&turn_type),
                    orange: updated[3].take().unwrap(),
                    white: updated[0].take().unwrap(),
                    yellow: updated[2].take().unwrap(),
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
        Face { tiles: vec![vec![color; 3]; 3] }
    }

    fn get_triplet(&self, section: &Triplet) -> Vec<Color> {
        match section {
            Triplet::Top => self.tiles[0].clone(),
            Triplet::Right => self.tiles.iter().map(|row| row[2].clone()).collect(),
            Triplet::Bottom => self.tiles[2].clone(),
            Triplet::Left => self.tiles.iter().map(|row| row[0].clone()).collect(),
        }
    }

    fn update_triplet(&self, section: &Triplet, other_section: &Triplet, cubies: &[Color]) -> Face {
        let reverse = match section {
            Triplet::Top => {
                discriminant(other_section) == discriminant(&Triplet::Left)
                    || discriminant(other_section) == discriminant(&Triplet::Bottom)
            },
            Triplet::Right => {
                discriminant(other_section) == discriminant(&Triplet::Left)
                    || discriminant(other_section) == discriminant(&Triplet::Bottom)
            },
            Triplet::Bottom => {
                discriminant(other_section) == discriminant(&Triplet::Right)
                    || discriminant(other_section) == discriminant(&Triplet::Top)
            },
            Triplet::Left => {
                discriminant(other_section) == discriminant(&Triplet::Right)
                    || discriminant(other_section) == discriminant(&Triplet::Top)
            },
        };

        let prepared_cubies: Vec<Color> = if reverse {
            cubies.into_iter().rev().map(|c| c.clone()).collect()
        } else {
            cubies.to_vec()
        };

        Face {
            tiles: {
                match section {
                    Triplet::Top => {
                        vec![prepared_cubies, self.tiles[1].clone(), self.tiles[2].clone()]
                    },
                    Triplet::Right => {
                        let mut tile_array = self.tiles.clone();
                        for i in 0..3 {
                            tile_array[i][2] = prepared_cubies[i].clone();
                        }

                        tile_array
                    },
                    Triplet::Bottom => {
                        vec![self.tiles[0].clone(), self.tiles[1].clone(), prepared_cubies]
                    },
                    Triplet::Left => {
                        let mut tile_array = self.tiles.clone();
                        for i in 0..3 {
                            tile_array[i][0] = prepared_cubies[i].clone();
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
                let mut tile_array = vec![vec![self.tiles[1][1].clone(); 3]; 3];
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

#[derive(Clone, PartialEq)]
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
            Color::Green => write!(f, "{}", "\u{2588}\u{2589}".truecolor(38, 203, 51)),
            Color::Red => write!(f, "{}", "\u{2588}\u{2589}".truecolor(178, 36, 36)),
            Color::Blue => write!(f, "{}", "\u{2588}\u{2589}".truecolor(32, 80, 173)),
            Color::Orange => write!(f, "{}", "\u{2588}\u{2589}".truecolor(255, 102, 0)),
            Color::White => write!(f, "{}", "\u{2588}\u{2589}".truecolor(239, 239, 239)),
            Color::Yellow => write!(f, "{}", "\u{2588}\u{2589}".truecolor(255, 251, 0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_top() {
        let cube = Cube::new().mv(Turn::U(TurnType::Clock));

        let expected: Vec<Color> = vec![
            // Green
            vec![Color::Red; 3],
            vec![Color::Green; 6],
            // Red
            vec![Color::Blue; 3],
            vec![Color::Red; 6],
            // Blue
            vec![Color::Orange; 3],
            vec![Color::Blue; 6],
            // Orange
            vec![Color::Green; 3],
            vec![Color::Orange; 6],
            // White
            vec![Color::White; 9],
            // Yellow
            vec![Color::Yellow; 9],
        ]
        .into_iter()
        .flatten()
        .collect();
        let cube_tiles: Vec<Color> = vec![
            cube.green.tiles.into_iter().flatten().collect::<Vec<Color>>(),
            cube.red.tiles.into_iter().flatten().collect::<Vec<Color>>(),
            cube.blue.tiles.into_iter().flatten().collect::<Vec<Color>>(),
            cube.orange.tiles.into_iter().flatten().collect::<Vec<Color>>(),
            cube.white.tiles.into_iter().flatten().collect::<Vec<Color>>(),
            cube.yellow.tiles.into_iter().flatten().collect::<Vec<Color>>(),
        ]
        .into_iter()
        .flatten()
        .collect();

        assert!(expected.iter().zip(cube_tiles.iter()).all(|(a, b)| a == b));
    }
}
