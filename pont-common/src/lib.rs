use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClientMessage {
    CreateRoom(String),
    JoinRoom(String, String),
    Chat(String),
    Play(Vec<(Piece, i32, i32)>),

    /*
    Swap(Vec<Piece>),
    */

    Disconnected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ServerMessage {
    JoinedRoom {
        room_name: String,
        players: Vec<(String, u32, bool)>,
        active_player: usize,
        board: Vec<((i32, i32), Piece)>,
        pieces: Vec<Piece>,
    },
    UnknownRoom(String),
    Chat {
        from: String,
        message: String,
    },
    Information(String),
    NewPlayer(String),
    PlayerDisconnected(usize),
    PlayerTurn(usize),
    Played(Vec<(Piece, i32, i32)>),
    MoveAccepted(Vec<Piece>),
    MoveRejected,
    PlayerScore {
        index: usize,
        delta: u32,
        total: u32,
    },


    /*
    Players {
        players: Vec<(String, usize)>,
        turn: usize,
    },
    YourTurn,
    NotYourTurn,
    Board(Board), // Used to send the initial board
    Draw(Vec<Piece>),
    InvalidMove(String),
    */
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Shape {
    Clover,
    Star,
    Square,
    Diamond,
    Cross,
    Circle,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Color {
    Orange,
    Yellow,
    Green,
    Red,
    Blue,
    Purple,
}

pub type Piece = (Shape, Color);

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub board: HashMap<(i32, i32), Piece>,
    pub bag: Vec<Piece>,
}

impl Game {
    pub fn play(&mut self, ps: &[(Piece, i32, i32)]) -> Option<u32> {
        let mut score = 0;
        for (p, x, y) in ps {
            if self.board.contains_key(&(*x, *y)) {
                return None;
            } else {
                self.board.insert((*x, *y), *p);
                score += 1;
            }
        }
        Some(score)
    }

    pub fn new() -> Game {
        use Color::*;
        use Shape::*;
        let mut bag = Vec::new();
        for c in &[Orange, Yellow, Green, Red, Blue, Purple] {
            for s in &[Clover, Star, Square, Diamond, Cross, Circle] {
                for _ in 0..3 {
                    bag.push((*s, *c));
                }
            }
        }
        bag.shuffle(&mut thread_rng());

        Game {
            board: HashMap::new(), bag
        }
    }

    pub fn deal(&mut self, n: usize) -> HashMap<Piece, usize> {
        let mut out = HashMap::new();
        for _ in 0..n {
            if let Some(p) = self.bag.pop() {
                *out.entry(p).or_insert(0) += 1;
            }
        }
        out
    }

    pub fn exchange(&mut self, pieces: Vec<Piece>) -> Option<Vec<Piece>> {
        if pieces.len() <= self.bag.len() {
            let mut out = Vec::new();
            for _ in 0..pieces.len() {
                out.push(self.bag.pop().unwrap());
            }
            for p in pieces.into_iter() {
                self.bag.push(p);
            }
            self.bag.shuffle(&mut thread_rng());
            Some(out)
        } else {
            None
        }
    }

    // Checks whether the given board is valid,
    // returning a vec of invalid piece locations
    pub fn invalid(board: &HashMap<(i32, i32), Piece>) -> Vec<(i32, i32)> {
        let mut todo: Vec<(i32, i32)> = board.keys().cloned().collect();
        let mut checked_h = HashSet::new();
        let mut checked_v = HashSet::new();

        let mut out = HashSet::new();
        let explore = |f: &dyn Fn(i32) -> (i32, i32)| {
            let mut out = Vec::new();
            for i in 0.. {
                let c = f(i);
                if let Some(piece) = board.get(&c) {
                    out.push((piece, c));
                } else {
                    break;
                }
            }
            for i in 1.. {
                let c = f(-i);
                if let Some(piece) = board.get(&c) {
                    out.push((piece, c));
                } else {
                    break;
                }
            }
            return out;
        };

        while let Some((x, y)) = todo.pop() {
            if !checked_h.contains(&(x, y)) {
                let row = explore(&|i| (x + i, y));
                for (_, c) in row.into_iter() {
                    checked_h.insert(c);
                }
            }
            if !checked_v.contains(&(x, y)) {
                let col = explore(&|i| (x, y + i));
                for (_, c) in col.into_iter() {
                    checked_v.insert(c);
                }
            }
        }
        out.into_iter().collect()
    }
}
