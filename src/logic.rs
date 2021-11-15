use crate::{Battlesnake, Board, Coord, Game};
use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;

pub fn get_info() -> JsonValue {
    log::info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "",
        "color": "#888888",
        "head": "all-seeing",
        "tail": "mystic-moon",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    log::info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    log::info!("{} END", game.id);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Up,
    Down,
    Right,
}

impl Move {
    fn as_str(&self) -> &'static str {
        match self {
            Move::Left => "left",
            Move::Up => "up",
            Move::Down => "down",
            Move::Right => "right",
        }
    }
}

impl Coord {
    fn advance(self, m: Move) -> Option<Coord> {
        Some(match m {
            Move::Left => Coord { x: self.x.checked_sub(1)?, y: self.y },
            Move::Up => Coord { y: self.y.checked_add(1)?, x: self.x },
            Move::Down => Coord { x: self.x, y: self.y.checked_sub(1)? },
            Move::Right => Coord { x: self.x.checked_add(1)?, y: self.y },
        })
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, you: &Battlesnake) -> &'static str {
    let mut possible_moves: [Option<Move>; 4] = [
        Some(Move::Left),
        Some(Move::Up),
        Some(Move::Down),
        Some(Move::Right),
    ];

    //Don't turn around
    let my_head = &you.head;
    let my_neck = &you.body[1];
    if my_neck.x < my_head.x {
        // my neck is left of my head
        possible_moves[Move::Left as usize] = None;
    } else if my_neck.x > my_head.x {
        // my neck is right of my head
        possible_moves[Move::Right as usize] = None;
    } else if my_neck.y < my_head.y {
        // my neck is below my head
        possible_moves[Move::Down as usize] = None;
    } else if my_neck.y > my_head.y {
        // my neck is above my head
        possible_moves[Move::Up as usize] = None;
    }

    //Don't go out of bounds
    if my_head.x == 0 {
        possible_moves[Move::Down as usize] = None;
    }
    if my_head.x == board.height {
        possible_moves[Move::Up as usize] = None;
    }
    if my_head.y == 0 {
        possible_moves[Move::Left as usize] = None;
    }
    if my_head.y == board.width {
        possible_moves[Move::Right as usize] = None;
    }

    //Dont hit self or others
    let mut occupied: Vec<Coord> = vec![];
    for snake in board.snakes.iter() {
        occupied.extend(snake.body.iter().copied());
        occupied.push(snake.head);
    }
    occupied.push(you.head);
    occupied.extend(you.body.iter().copied());
    occupied.sort_unstable();
    occupied.dedup();
    possible_moves = possible_moves.map(|pm| {
        pm.filter(|&m| {
            //check under and overflows
            let n = match you.head.advance(m) {
                Some(n) => n,
                _ => return false,
            };
            //"keep" this move if it is not already occupied
            occupied.binary_search(&n).is_err()
        })
    });
    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.
    let moves = possible_moves.into_iter().flatten().collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    log::info!("{} MOVE {}", game.id, chosen);

    return chosen.as_str();
}
