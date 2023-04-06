use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Kaze,
    Sangen,
}

#[derive(Debug, Copy, Clone, Ord, Eq)]
pub struct MahjongTile {
    pub suit: Suit,
    pub value: u8,
    pub is_dora: bool,
}

impl PartialOrd for MahjongTile {
    fn partial_cmp(&self, other: &MahjongTile) -> Option<Ordering> {
        match self.suit.partial_cmp(&other.suit) {
            Some(Ordering::Equal) => self.value.partial_cmp(&other.value),
            other => other,
        }
    }
}

impl PartialEq for MahjongTile {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

#[derive(Debug, Clone)]
pub enum SeatWind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub points: i32,
    pub hand: Vec<MahjongTile>,
    pub discards: Vec<MahjongTile>,
    pub seat_wind: SeatWind,
    pub strategy: Strategy,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            points: 25000,
            hand: Vec::new(),
            discards: Vec::new(),
            seat_wind: SeatWind::East,
            strategy: Strategy::new(default_discard_strategy, default_tsumo_strategy),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Strategy {
    pub discard: fn(GameState) -> usize,
    pub tsumo: fn(GameState) -> bool,
}

impl Strategy {
    fn new(discard: fn(GameState) -> usize, tsumo: fn(GameState) -> bool) -> Strategy {
        Strategy { discard, tsumo }
    }
}

fn default_discard_strategy(game_state: GameState) -> usize {
    0
}

fn default_tsumo_strategy(game_state: GameState) -> bool {
    false
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: [Player; 4],
    pub wall: Vec<MahjongTile>,
    pub wall_dead: Vec<MahjongTile>,
    pub dora_indicators: Vec<MahjongTile>,
    pub dora_index: usize,
}

pub type Players = (Player, Player, Player, Player);

pub type Hands = (
    Vec<MahjongTile>, // Wall
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
);
