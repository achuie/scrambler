fn main() {}

struct Cube {
    cubies: Vec<i32>,
}

impl Cube {
    fn new() -> Self {
        Self {
            cubies: {
                let mut res = Vec::new();
                for i in 0..20 {
                    res.push(i);
                }

                res
            },
        }
    }

    fn mv(&mut self, turn: Turns) {
        match turn {}
    }
}

enum Turn {
    U,
    Up,
    D,
    Dp,
    R,
    Rp,
    L,
    Lp,
    F,
    Fp,
    B,
    Bp,
}
