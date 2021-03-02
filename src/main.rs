fn main() {}

struct Cube {
    faces: [Face; 6],
}

impl Cube {
    fn new() -> Self {
        Cube {
            faces: {
                let mut res = [Face; 6];
                res[0] = Face::new(Color::Green);
                res[1] = Face::new(Color::Red);
                res[2] = Face::new(Color::Blue);
                res[3] = Face::new(Color::Orange);
                res[4] = Face::new(Color::White);
                res[5] = Face::new(Color::Yellow);

                res
            },
        }
    }

    fn mv(&self, turn: Turn) -> Self {
        match turn {
            Turn::U(prime, double) => Cube {
                faces: {
                    let mut res = [Face; 6];
                    res[0] = Face {
                        tiles: {
                            let mut ts = self.faces[0].tiles;
                            for i in 0..3 {
                                ts[i] = self.faces[3].tiles[i];
                            }

                            ts
                        },
                    };
                    res[4] = self.faces[4].rotate(prime, double);

                    res
                },
            },
        }
    }
}

struct Face {
    pub tiles: [[Color; 3]; 3],
}

impl Face {
    fn new(color: Color) -> Self {
        Face { tiles: [[color; 3]; 3] }
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
