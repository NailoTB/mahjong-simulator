use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Kaze,
    Sangen,
}

#[derive(Debug, Copy, Clone, Eq)]
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

impl Ord for MahjongTile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
    pub open_hand: Vec<MahjongTile>,
    pub discards: Vec<MahjongTile>,
    pub seat_wind: SeatWind,
    pub strategy: Strategy,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            points: 25000,
            hand: Vec::new(),
            open_hand: Vec::new(),
            discards: Vec::new(),
            seat_wind: SeatWind::East,
            strategy: Strategy::new(
                default_boolean_strategy,
                default_boolean_strategy,
                default_discard_strategy,
                default_boolean_strategy,
                default_boolean_strategy,
                default_boolean_strategy,
            ),
        }
    }
}

impl Player {
    pub fn move_tile_to_open_hand(&mut self, tile: &MahjongTile) -> bool {
        if let Some(index) = self.hand.iter().position(|t| t == tile) {
            self.open_hand.push(self.hand.remove(index));
            true
        } else {
            false
        }
    }
    pub fn hand_is_open(&self) -> bool {
        !self.open_hand.is_empty()
    }
    pub fn get_closed_hand(&self) -> &Vec<MahjongTile> {
        &self.hand
    }

    pub fn get_open_cards(&self) -> &Vec<MahjongTile> {
        &self.open_hand
    }
    pub fn get_hand(&self) -> Vec<MahjongTile> {
        let mut entire_hand = self.hand.clone();
        entire_hand.append(&mut self.open_hand.clone());
        entire_hand.sort();
        entire_hand
    }
    pub fn sort_hand(&mut self) {
        let _ = &self.hand.sort();
    }
}

#[derive(Debug, Clone)]
pub struct Strategy {
    pub call_chi: fn(GameState) -> bool,
    pub call_pon: fn(GameState) -> bool,
    pub discard: fn(GameState) -> usize,
    pub tsumo: fn(GameState) -> bool,
    pub kan: fn(GameState) -> bool,
    pub riichi: fn(GameState) -> bool,
}

impl Strategy {
    fn new(
        call_chi: fn(GameState) -> bool,
        call_pon: fn(GameState) -> bool,
        discard: fn(GameState) -> usize,
        tsumo: fn(GameState) -> bool,
        kan: fn(GameState) -> bool,
        riichi: fn(GameState) -> bool,
    ) -> Strategy {
        Strategy {
            call_chi,
            call_pon,
            discard,
            tsumo,
            kan,
            riichi,
        }
    }
}

fn default_discard_strategy(_game_state: GameState) -> usize {
    0
}

fn default_boolean_strategy(_game_state: GameState) -> bool {
    true
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
