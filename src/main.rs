fn main() {}

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

    fn looped_update(to_update: [Face; 4], update_sections: [Triplet; 4]) -> [Face; 4] {
        to_update
            .iter()
            .enumerate()
            .map(|(i, face)| {
                let other_idx = {
                    if double {
                        (i + 2) % 4
                    } else {
                        if prime {
                            (i - 1) % 4
                        } else {
                            (i + 1) % 4
                        }
                    }
                };

                face.update_triplet(
                    update_sections[i],
                    to_update[other_idx].get_triplet(update_sections[other_idx]),
                )
            })
            .collect()
    }

    fn mv(&self, turn: Turn) -> Self {
        match turn {
            Turn::U { prime, double } => {
                let to_update = [self.green, self.red, self.blue, self.orange];
                let update_sections = [Triplet::Top; 4];
                let updated = Cube::looped_update(to_update, update_sections);

                Cube {
                    green: updated[0],
                    red: updated[1],
                    blue: updated[2],
                    orange: updated[3],
                    white: self.white.rotate(prime, double),
                    yellow: self.yellow.clone(),
                }
            },
            Turn::D { prime, double } => {
                let to_update = [self.green, self.red, self.blue, self.orange];
                let update_sections = [Triplet::Bottom; 4];
                let updated = Cube::looped_update(to_update, update_sections);

                Cube {
                    green: updated[0],
                    red: updated[1],
                    blue: updated[2],
                    orange: updated[3],
                    white: self.white.clone(),
                    yellow: self.yellow.rotate(prime, double),
                }
            },
            Turn::R { prime, double } => {
                let to_update = [self.green, self.yellow, self.blue, self.white];
                let update_sections =
                    [Triplet::Right, Triplet::Right, Triplet::Left, Triplet::Right];
                let updated = Cube::looped_update(to_update, update_sections);

                Cube {
                    green: updated[0],
                    red: self.red.rotate(prime, double),
                    blue: updated[2],
                    orange: self.orange.clone(),
                    white: updated[3],
                    yellow: updated[1],
                }
            },
            Turn::L { prime, double } => {
                let to_update = [self.green, self.yellow, self.blue, self.white];
                let update_sections = [Triplet::Left, Triplet::Left, Triplet::Right, Triplet::Left];
                let updated = Cube::looped_update(to_update, update_sections);

                Cube {
                    green: updated[0],
                    red: self.red.clone(),
                    blue: updated[2],
                    orange: self.orange.rotate(prime, double),
                    white: updated[3],
                    yellow: updated[1],
                }
            },
            Turn::F { prime, double } => {
                let to_update = [self.white, self.orange, self.yellow, self.red];
                let update_sections =
                    [Triplet::Bottom, Triplet::Right, Triplet::Top, Triplet::Left];
                let updated = Cube::looped_update(to_update, update_sections);

                Cube {
                    green: self.green.rotate(prime, double),
                    red: updated[3],
                    blue: self.blue.clone(),
                    orange: updated[1],
                    white: updated[0],
                    yellow: updated[2],
                }
            },
            Turn::B { prime, double } => {
                let to_update = [self.white, self.orange, self.yellow, self.red];
                let update_sections =
                    [Triplet::Top, Triplet::Left, Triplet::Bottom, Triplet::Right];
                let updated = Cube::looped_update(to_update, update_sections);

                Cube {
                    green: self.green.clone(),
                    red: updated[3],
                    blue: self.blue.rotate(prime, double),
                    orange: updated[1],
                    white: updated[0],
                    yellow: updated[2],
                }
            },
        }
    }
}

#[derive(Copy)]
struct Face {
    tiles: [[Color; 3]; 3],
}

impl Face {
    fn new(color: Color) -> Self {
        Face { tiles: [[color; 3]; 3] }
    }

    fn get_triplet(&self, section: Triplet) -> [Color; 3] {
        match section {
            Triplet::Top => self.tiles[0],
            Triplet::Right => self.tiles.iter().map(|row| row[2]).collect(),
            Triplet::Bottom => self.tiles[2],
            Triplet::Left => self.tiles.iter().map(|row| row[0]).collect(),
        }
    }

    fn update_triplet(&self, section: Triplet, cubies: [Color; 3]) -> Face {
        Face {
            tiles: {
                match section {
                    Triplet::Top => [cubies, self.tiles[1], self.tiles[2]],
                    Triplet::Right => {
                        let mut tile_array = self.tiles.clone();
                        for i in 0..3 {
                            tile_array[i][2] = cubies[i];
                        }

                        tile_array
                    },
                    Triplet::Bottom => [self.tiles[0], self.tiles[1], cubies],
                    Triplet::Left => {
                        let mut tile_array = self.tiles.clone();
                        for i in 0..3 {
                            tile_array[i][0] = cubies[i];
                        }
                    },
                }
            },
        }
    }

    fn rotate(&self, prime: bool, double: bool) -> Face {
        Face {
            tiles: {
                let mut tile_array = [[self.tiles[1][1]; 3]; 3];
                if double {
                    // 00 01 02    22 21 20
                    // 10 11 12 -> 12 11 10
                    // 20 21 22    02 01 00
                    for i in (0..3).rev() {
                        for j in (0..3).rev() {
                            tile_array[2 - i][2 - j] = self.tiles[i][j];
                        }
                    }
                } else {
                    if prime {
                        // 00 01 02    02 12 22
                        // 10 11 12 -> 01 11 21
                        // 20 21 22    00 10 20
                        for j in (0..3).rev() {
                            for i in 0..3 {
                                tile_array[2 - j][i] = self.tiles[i][j];
                            }
                        }
                    } else {
                        // 00 01 02    20 10 00
                        // 10 11 12 -> 21 11 01
                        // 20 21 22    22 12 02
                        for j in 0..3 {
                            for i in (0..3).rev() {
                                tile_array[j][2 - i] = self.tiles[i][j];
                            }
                        }
                    }
                }

                tile_array
            },
        }
    }
}

enum Triplet {
    Top,
    Right,
    Bottom,
    Left,
}

enum Color {
    Green,
    Red,
    Blue,
    Orange,
    White,
    Yellow,
}

enum Turn {
    U { prime: bool, double: bool },
    D { prime: bool, double: bool },
    R { prime: bool, double: bool },
    L { prime: bool, double: bool },
    F { prime: bool, double: bool },
    B { prime: bool, double: bool },
}
