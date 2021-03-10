mod cube;
mod turn;

use crate::cube::Cube;
use crate::turn::Turn;
use clap::{crate_version, App, Arg};
use std::mem::discriminant;

fn main() {
    let matches = App::new("scrambler")
        .about("Scramble Generator for Rubik's Cube")
        .version(crate_version!())
        .arg(
            Arg::with_name("ALGORITHM")
                .help("Method used to generate scramble")
                .index(1)
                .possible_values(&["rand", "ida"])
                .required(true),
        )
        .arg(
            Arg::with_name("num_moves")
                .short("n")
                .value_name("NATURAL")
                .default_value("25")
                .help("Number of random moves to generate"),
        )
        .get_matches();

    let num_turns: u32 = {
        let num_str = matches.value_of("num_moves").unwrap();

        num_str
            .parse()
            .unwrap_or_else(|_| panic!("*** Malformed number of moves '{}' ***", num_str))
    };

    let mut cube = Cube::new();
    let turns = generate_random_turns(num_turns);

    for t in turns {
        cube = cube.mv(t);
    }
    cube.print()
}

fn generate_random_turns(n_turns: u32) -> Vec<Turn> {
    let mut prev: Turn = rand::random();
    (0..n_turns)
        .map(|_| {
            let mut t: Turn = rand::random();
            while discriminant(&t) == discriminant(&prev) {
                t = rand::random();
            }
            prev = t.clone();

            t
        })
        .collect()
}
