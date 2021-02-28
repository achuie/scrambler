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
            Turn::U(prime, double) => {
                if prime {
                    Cube {
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
                            res[4] = Face {
                                tiles: {
                                    let mut ts = [[Color::White; 3]; 3];
                                    // Top.
                                    for i in 0..3 {
                                        ts[0][i] = self.faces[4].tiles[i][2];
                                    }
                                    // Right.
                                    for i in 1..3 {
                                        ts[i][2] = self.faces[4].tiles[2][2 - i];
                                    }
                                    // Bottom.
                                    for i in 0..2 {
                                        ts[2][i] = self.faces[4].tiles[i][0];
                                    }
                                    // Left.
                                    ts[1][0] = self.faces[4].tiles[0][1];

                                    ts
                                },
                            };

                            res
                        },
                    }
                } else {
                    asdf
                }
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
