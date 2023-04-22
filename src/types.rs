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

#[derive(Debug, Clone, PartialEq)]
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
            strategy: Strategy::default(),
        }
    }
}

impl Player {
    pub fn next_wind(&mut self) {
        self.seat_wind = match self.seat_wind {
            SeatWind::East => SeatWind::South,
            SeatWind::South => SeatWind::West,
            SeatWind::West => SeatWind::North,
            SeatWind::North => SeatWind::East,
        };
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
    pub call_chi: fn(StrategyInput) -> bool,
    pub call_pon: fn(StrategyInput) -> bool,
    pub discard: fn(StrategyInput) -> usize,
    pub tsumo: fn(StrategyInput) -> bool,
    pub kan: fn(StrategyInput) -> bool,
    pub riichi: fn(StrategyInput) -> bool,
}

impl Strategy {
    fn new(
        call_chi: fn(StrategyInput) -> bool,
        call_pon: fn(StrategyInput) -> bool,
        discard: fn(StrategyInput) -> usize,
        tsumo: fn(StrategyInput) -> bool,
        kan: fn(StrategyInput) -> bool,
        riichi: fn(StrategyInput) -> bool,
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

impl Default for Strategy {
    fn default() -> Strategy {
        Strategy {
            call_chi: default_boolean_strategy,
            call_pon: default_boolean_strategy,
            discard: default_discard_strategy,
            tsumo: default_boolean_strategy,
            kan: default_boolean_strategy,
            riichi: default_boolean_strategy,
        }
    }
}

fn default_discard_strategy(_strategy_input: StrategyInput) -> usize {
    0
}

fn default_boolean_strategy(_strategy_input: StrategyInput) -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct StrategyInput {
    pub hand: Vec<MahjongTile>,
    pub discards: Vec<Vec<MahjongTile>>,
}

#[derive(Debug, Clone)]
pub struct BoardTiles {
    pub wall: Vec<MahjongTile>,
    pub wall_dead: Vec<MahjongTile>,
    pub dora_indicators: Vec<MahjongTile>,
    pub dora_index: usize,
}

pub struct GameResult {
    pub player_1_score: i32,
    pub player_2_score: i32,
    pub player_3_score: i32,
    pub player_4_score: i32,
}

pub type Hands = (
    Vec<MahjongTile>, // Wall
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
);
