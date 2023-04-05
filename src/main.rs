use itertools::Itertools;
use rand::seq::SliceRandom;
use std::cmp::{Ordering, PartialOrd};
const DUPLICATE_TILES: usize = 4;

fn main() {
    let (mut player_a, mut player_b, mut player_c, mut player_d) = initialize_players();

    let (mut wall, wall_dead, dora_indicators) = initialize_wall();

    (
        wall,
        player_a.hand,
        player_b.hand,
        player_c.hand,
        player_d.hand,
    ) = draw_hands(wall);

    let mut game_state = GameState {
        players: [player_a, player_b, player_c, player_d],
        wall,
        wall_dead,
        dora_indicators,
        dora_index: 0,
    };

    flip_dora_indicator(&mut game_state);

    game_state.players[0].hand.sort();
    game_state.players[1].hand.sort();
    game_state.players[2].hand.sort();
    game_state.players[3].hand.sort();

    let testfunctio = find_pairs_melds(&game_state.players[0]);
    for res in testfunctio {
        println!("{:?}", res);
    }

    println!(
        "Dora Indicator: \n{:?}",
        game_state.dora_indicators[game_state.dora_index]
    );

    //println!("Wall:");
    //
    //for tile in &game_state.wall {
    //    println!("{:?}", tile);
    //}

    println!("Player A's hand:");
    for tile in &game_state.players[0].hand {
        println!("{:?}", tile);
    }

    let mut round_ongoing = true;
    let mut current_player_index: usize = 0;

    while round_ongoing {
        // Current player draws a tile
        move_tile(
            &mut game_state.wall,
            &mut game_state.players[current_player_index].hand,
        );
        // Current player may tsumo
        // Current player may kan
        // Current player discards a tile
        move_tile(
            &mut game_state.players[current_player_index].hand,
            &mut game_state.players[current_player_index].discards,
        );
        // Other players may ron
        // Other players may pon
        // Previous player may chi
        // Check if the wall is empty
        if game_state.wall.is_empty() {
            round_ongoing = false;
        }
        // Pass turn to the next player
        current_player_index = (current_player_index + 1) % 4;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Kaze,
    Sangen,
}

#[derive(Debug, Copy, Clone, Ord, Eq)]
struct MahjongTile {
    suit: Suit,
    value: u8,
    is_dora: bool,
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

fn initialize_wall() -> (Vec<MahjongTile>, Vec<MahjongTile>, Vec<MahjongTile>) {
    let mut wall: Vec<MahjongTile> = Vec::new();

    for suit_index in 1..=5 {
        let (suit, max_value) = match suit_index {
            1 => (Suit::Manzu, 9),
            2 => (Suit::Pinzu, 9),
            3 => (Suit::Souzu, 9),
            4 => (Suit::Kaze, 4),
            5 => (Suit::Sangen, 3),
            _ => panic!("Invalid suit!"),
        };

        for value in 1..=max_value {
            wall.push(MahjongTile {
                suit,
                value,
                is_dora: false,
            })
        }
    }

    wall = wall
        .iter()
        .flat_map(|&x| std::iter::repeat(x).take(DUPLICATE_TILES))
        .collect();

    let mut rng = rand::thread_rng();
    wall.shuffle(&mut rng);

    let mut wall_dead = wall.split_off(wall.len() - 14);
    let dora_indicators = wall_dead.split_off(wall_dead.len() - 10);

    (wall, wall_dead, dora_indicators)
}

#[derive(Debug)]
enum SeatWind {
    East,
    South,
    West,
    North,
}

#[derive(Debug)]
struct Player {
    points: i32,
    hand: Vec<MahjongTile>,
    discards: Vec<MahjongTile>,
    seat_wind: SeatWind,
}
impl Default for Player {
    fn default() -> Player {
        Player {
            points: 25000,
            hand: Vec::new(),
            discards: Vec::new(),
            seat_wind: SeatWind::East,
        }
    }
}

type Players = (Player, Player, Player, Player);

fn initialize_players() -> Players {
    let a: Player = Player {
        ..Default::default()
    };
    let b: Player = Player {
        seat_wind: SeatWind::South,
        ..Default::default()
    };
    let c: Player = Player {
        seat_wind: SeatWind::West,
        ..Default::default()
    };
    let d: Player = Player {
        seat_wind: SeatWind::North,
        ..Default::default()
    };

    (a, b, c, d)
}

#[derive(Debug)]
struct GameState {
    players: [Player; 4],
    wall: Vec<MahjongTile>,
    wall_dead: Vec<MahjongTile>,
    dora_indicators: Vec<MahjongTile>,
    dora_index: usize,
}

type Hands = (
    Vec<MahjongTile>, // Wall
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
);

fn draw_hands(mut wall: Vec<MahjongTile>) -> Hands {
    let a = wall.split_off(wall.len() - 13);
    let b = wall.split_off(wall.len() - 13);
    let c = wall.split_off(wall.len() - 13);
    let d = wall.split_off(wall.len() - 13);
    (wall, a, b, c, d)
}

fn flip_dora_indicator(game_state: &mut GameState) {
    let dora_indicator: &MahjongTile = &game_state.dora_indicators[game_state.dora_index];
    let dora_suit: Suit = dora_indicator.suit;

    let suit_modulo = match dora_suit {
        Suit::Manzu | Suit::Pinzu | Suit::Souzu => 9,
        Suit::Kaze => 4,
        Suit::Sangen => 3,
    };

    let dora_value: u8 = (dora_indicator.value) % (suit_modulo) + 1;

    change_dora_bool(&mut game_state.wall, dora_suit, dora_value);
    change_dora_bool(&mut game_state.wall_dead, dora_suit, dora_value);
    change_dora_bool(&mut game_state.dora_indicators, dora_suit, dora_value);
    change_dora_bool(&mut game_state.players[0].hand, dora_suit, dora_value);
    change_dora_bool(&mut game_state.players[1].hand, dora_suit, dora_value);
    change_dora_bool(&mut game_state.players[2].hand, dora_suit, dora_value);
    change_dora_bool(&mut game_state.players[3].hand, dora_suit, dora_value); //what a mess
}

fn change_dora_bool(tile_list: &mut [MahjongTile], dora_suit: Suit, dora_value: u8) {
    for tile in tile_list
        .iter_mut()
        .filter(|tile| tile.suit == dora_suit && tile.value == dora_value)
    {
        tile.is_dora = true;
    }
}

fn move_tile(hand_from: &mut Vec<MahjongTile>, hand_to: &mut Vec<MahjongTile>) {
    if let Some(tile) = hand_from.pop() {
        hand_to.push(tile);
    }
}

fn find_pairs_melds(player: &Player) -> Vec<Vec<MahjongTile>> {
    let mut results = Vec::new();
    let hand = &player.hand;
    for suitloop in [
        Suit::Manzu,
        Suit::Pinzu,
        Suit::Souzu,
        Suit::Kaze,
        Suit::Sangen,
    ] {
        let suit_tiles: Vec<_> = hand
            .iter()
            .filter(|&tile| tile.suit == suitloop)
            .cloned()
            .collect();
        let threes = suit_tiles.iter().combinations(3);
        let pairs = suit_tiles.iter().combinations(2);
        for three in threes {
            let three_vec: Vec<_> = three.into_iter().cloned().collect();

            if (three_vec.windows(2).all(|w| w[0] == w[1]) && !results.contains(&three_vec))
                || (three_vec[2].value - three_vec[0].value == 2
                    && three_vec[2].value - three_vec[1].value == 1
                    && suitloop != Suit::Kaze
                    && suitloop != Suit::Sangen
                    && !results.contains(&three_vec))
            {
                results.push(three_vec);
            }
        }
        for pair in pairs {
            let pair_vec: Vec<_> = pair.into_iter().cloned().collect();

            if pair_vec.windows(2).all(|w| w[0] == w[1]) && !results.contains(&pair_vec) {
                results.push(pair_vec);
            }
        }
    }
    results
}
