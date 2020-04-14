#[derive(Clone, Copy)]
enum Player {
    X,
    O,
}

struct Game {
    board: [Option<Player>; 9],
    player: Player,
}

enum Outcome {
    Ongoing,
    XWon,
    OWon,
    Draw,
}

impl Outcome {
    fn winner(player: Player) -> Self {
        match player {
            Player::X => Self::XWon,
            Player::O => Self::OWon,
        }
    }
}

impl Game {
    fn new() -> Self {
        Game {
            board: [None; 9],
            player: Player::X,
        }
    }

    fn move(self, action: u8) -> (Self, Outcome) {
    }
}
