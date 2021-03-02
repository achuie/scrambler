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
                                    for j in 0..3 {
                                        for i in (0..3).rev() {
                                            ts[j][2 - i] = self.faces[4].tiles[i][j];
                                        }
                                    }

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
