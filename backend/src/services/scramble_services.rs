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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_3x3_scramble_length() {
        let scramble = generate(ScrambleKind::Three);

        // Check that the scramble sequence has exactly 20 moves
        assert_eq!(
            scramble
                .sequence
                .split_whitespace()
                .count(),
            20
        );
    }

    #[test]
    fn test_generate_scramble_kind() {
        let scramble = generate(ScrambleKind::Three);

        assert!(matches!(scramble.kind, ScrambleKind::Three));
    }

    #[test]
    fn test_generate_3x3_scramble_valid_moves() {
        let scramble = generate(ScrambleKind::Three);
        let valid_moves = ["U", "D", "R", "L", "F", "B"];
        let valid_modifiers = ["", "'", "2"];

        for move_str in scramble
            .sequence
            .split_whitespace()
        {
            let move_part = &move_str[..1];
            let modifier_part = &move_str[1..];

            assert!(valid_moves.contains(&move_part));
            assert!(valid_modifiers.contains(&modifier_part));
        }
    }

    #[test]
    fn test_generate_no_repeated_consecutive_moves() {
        let scramble = generate(ScrambleKind::Three);

        let moves: Vec<&str> = scramble
            .sequence
            .split_whitespace()
            .collect();
        for i in 0..moves.len() - 1 {
            assert_ne!(moves[i], moves[i + 1]);
        }
    }
}
