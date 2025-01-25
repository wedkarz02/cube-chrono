use rand::seq::SliceRandom;

use crate::routes::scrambles::{Scramble, ScrambleKind};

pub fn generate(kind: ScrambleKind) -> Scramble {
    match kind {
        ScrambleKind::Three => Scramble {
            kind,
            sequence: generate_3x3(),
        },
    }
}

fn generate_3x3() -> String {
    let moves = ["U", "D", "R", "L", "F", "B"];
    let modifiers = ["", "'", "2"];
    let mut rng = rand::thread_rng();

    let length = 20;
    let mut scramble = Vec::new();
    let mut last_move = "";

    for _ in 0..length {
        let mut next_move;
        loop {
            next_move = moves
                .choose(&mut rng)
                .unwrap();
            if next_move != &last_move {
                break;
            }
        }

        let modifier = modifiers
            .choose(&mut rng)
            .unwrap();
        scramble.push(format!("{}{}", next_move, modifier));
        last_move = next_move;
    }

    scramble.join(" ")
}
