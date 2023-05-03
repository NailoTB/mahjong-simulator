pub mod mahjong_tile;
use mahjong_tile::*;

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
    pub id: usize,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            points: 25000,
            seat_wind: SeatWind::East,
            strategy: Strategy::default(),
            id: 0,
        }
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
