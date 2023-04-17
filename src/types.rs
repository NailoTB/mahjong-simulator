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
    pub seat_wind: SeatWind,
    pub strategy: Strategy,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            points: 25000,
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

#[derive(Debug, Clone)]
pub struct PlayerTiles {
    pub hand: Vec<Vec<MahjongTile>>,
    pub open_hand: Vec<Vec<MahjongTile>>,
    pub discards: Vec<Vec<MahjongTile>>,
}

impl Default for PlayerTiles {
    fn default() -> Self {
        let hand = vec![vec![]; 4];
        let open_hand = vec![vec![]; 4];
        let discards = vec![vec![]; 4];
        Self {
            hand,
            open_hand,
            discards,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Strategy {
    pub call_chi: fn(bool) -> bool,
    pub call_pon: fn(bool) -> bool,
    pub discard: fn(bool) -> usize,
    pub tsumo: fn(bool) -> bool,
    pub kan: fn(bool) -> bool,
    pub riichi: fn(bool) -> bool,
}

impl Strategy {
    fn new(
        call_chi: fn(bool) -> bool,
        call_pon: fn(bool) -> bool,
        discard: fn(bool) -> usize,
        tsumo: fn(bool) -> bool,
        kan: fn(bool) -> bool,
        riichi: fn(bool) -> bool,
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

fn default_discard_strategy(_strategy_input: bool) -> usize {
    0
}

fn default_boolean_strategy(_strategy_input: bool) -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct BoardTiles {
    pub wall: Vec<MahjongTile>,
    pub wall_dead: Vec<MahjongTile>,
    pub dora_indicators: Vec<MahjongTile>,
    pub dora_index: usize,
}

pub type Hands = (
    Vec<MahjongTile>, // Wall
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
);
