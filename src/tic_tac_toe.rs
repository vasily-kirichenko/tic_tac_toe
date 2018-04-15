use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Player {
    X,
    O,
}

use self::Player::*;

impl Player {
    fn swap(&mut self) -> () {
        match self {
            X => *self = O,
            O => *self = X
        }
    }

    fn image(&self) -> String {
        match self {
            X => "Cross".to_string(),
            O => "Nought".to_string()
        }
    }
}

pub enum GameResult {
    StillPlaying,
    Win(Player),
    Draw,
}

use self::GameResult::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Pos(i32, i32);

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos(x, y)
    }

    fn ui_text(&self) -> String {
        format!("{}{}", self.0, self.1)
    }
}

pub enum Msg {
    Play(Pos),
    Restart,
}

use self::Msg::*;

struct Line(Vec<Option<Player>>);

impl Line {
    fn get_winner(&self) -> Option<Player> {
        match self.0.as_slice() {
            [Some(X), Some(X), Some(X)] => Some(X),
            [Some(O), Some(O), Some(O)] => Some(O),
            _ => None
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: HashMap<Pos, Player>
}

impl Board {
    fn new() -> Self {
        Board { cells: HashMap::new() }
    }

    fn get_line(&self, line: &[Pos; 3]) -> Line {
        Line(line.iter()
            .map(|x| self.cells.get(x).map(|&x| x))
            .collect())
    }

    fn any_more_moves(&self) -> bool {
        self.cells.iter().count() < 9
    }

    fn set(&mut self, pos: Pos, player: Player) -> () {
        self.cells.insert(pos, player);
    }

    fn get(&self, pos: &Pos) -> Option<&Player> {
        self.cells.get(pos)
    }
}

#[derive(Debug)]
pub struct Model {
    next_up: Player,
    board: Board,
}

impl Model {
    pub fn new() -> Self {
        Model {
            next_up: X,
            board: Board::new(),
        }
    }

    pub fn get_game_result(&self) -> GameResult {
        let lines = [
            [Pos(0, 0), Pos(0, 1), Pos(0, 2)],
            [Pos(1, 0), Pos(1, 1), Pos(1, 2)],
            [Pos(2, 0), Pos(2, 1), Pos(2, 2)],
            [Pos(0, 0), Pos(1, 0), Pos(2, 0)],
            [Pos(0, 1), Pos(1, 1), Pos(2, 1)],
            [Pos(0, 2), Pos(1, 2), Pos(2, 2)],
            [Pos(0, 0), Pos(1, 1), Pos(2, 2)],
            [Pos(0, 2), Pos(1, 1), Pos(2, 0)]
        ];

        let line =
            lines.iter()
                .map(|line| self.board.get_line(line))
                .filter_map(|line| line.get_winner())
                .collect::<Vec<Player>>();

        let line_winner = line.first();

        match line_winner {
            Some(&p) => Win(p),
            _ if self.board.any_more_moves() => StillPlaying,
            _ => Draw
        }
    }

    fn get_message(&self) -> String {
        match self.get_game_result() {
            StillPlaying => format!("{:?} turn", self.next_up),
            Win(p) => format!("{:?} wins!", p),
            Draw => "It is a draw!".to_string()
        }
    }

    pub fn update(&mut self, msg: Msg, game_over: impl FnOnce(String) -> ()) -> () {
        match msg {
            Play(pos) => {
                self.board.set(pos, self.next_up);
                self.next_up.swap()
            }
            Restart => *self = Model::new()
        };

        match self.get_game_result() {
            StillPlaying => (),
            _ => game_over(self.get_message())
        }
    }

    fn can_play(&self, cell: Option<&Player>) -> bool {
        match (self.get_game_result(), cell) {
            (StillPlaying, None) => true,
            _ => false
        }
    }
}

pub struct ViewBinding;

mod binding {
    use tic_tac_toe::*;

    pub fn one_way<T>(name: &'static str, getter: impl FnOnce(Model) -> T) -> ViewBinding {
        ViewBinding
    }

    pub fn msg(name: &'static str, msg: Msg) -> ViewBinding {
        ViewBinding
    }
}

pub fn view() -> Vec<ViewBinding> {
    let positions = c![Pos(x, y), for x in 0..2, for y in 0..2];

    positions
        .iter()
        .flat_map(|&p| {
            vec![
                binding::msg("Play", Play(p)),
                binding::one_way("CanPlay", |m| m.can_play(m.board.get(&p))),
                binding::one_way("Image", |m|
                    m.board.get(&p).map(|x| x.image()).unwrap_or("".to_string()),
                )
            ]
        })
        .chain(
            vec![
                binding::one_way("TurnMessage", |m| m.get_message()),
                binding::msg("Restart", Restart),
            ]
        )
        .collect()
}