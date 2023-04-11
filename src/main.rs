use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::time::Instant;
mod types;
use types::*;
const DUPLICATE_TILES: usize = 4;
const ROUNDS: u8 = 4 * 2;
const GAMES: usize = 1;

fn main() {
    let start_time = Instant::now();
    for game in 1..=GAMES {
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

        for i in 0..=3 {
            game_state.players[i].sort_hand();
        }

        //println!("Dora Indicator:");
        //print_tile(&game_state.dora_indicators[game_state.dora_index]);

        //println!("Player A's hand:");
        //print_hand(&game_state.players[0].get_hand());

        //println!("game {game}");

        let mut current_player_index: usize = 0;
        for round in 1..=ROUNDS {
            //println!("round {round}");
            let mut round_ongoing = true;
            let mut skip_draw = false;
            let mut skip_chi = false;
            while round_ongoing {
                let next_player_index = (current_player_index + 1) % 4;
                // Current player draws a tile
                game_state.players[0].hand.sort();
                let (tenpai0, waits0) = check_tenpai(&game_state.players[0].hand);

                if tenpai0 {
                    println!("Player A's hand:");
                    print_hand(&game_state.players[0].get_hand());
                    println!("Player A's Waits:");
                    print_hand(&waits0);
                }
                if skip_draw {
                    skip_draw = false;
                } else {
                    draw_tile(
                        &mut game_state.wall,
                        &mut game_state.players[current_player_index].hand,
                    );
                }
                // Current player may tsumo
                // Current player may kan
                // Current player discards a tile
                let strategy_input = game_state.clone();
                move_tile(
                    &mut game_state.players[current_player_index].hand,
                    &mut game_state.players[current_player_index].discards,
                    (game_state.players[current_player_index].strategy.discard)(strategy_input),
                );
                let discarded = *game_state.players[current_player_index]
                    .discards
                    .last()
                    .unwrap();

                // Other players may ron
                // Other players may pon
                for i in 0..=3 {
                    if i != current_player_index
                        && can_pon(&game_state.players[i].hand, &discarded)
                        && (game_state.players[i].strategy.call_pon)(game_state.clone())
                    {
                        //println!("some guy pon'd a {:?}", discarded);
                        game_state.players[i].hand.sort();
                        game_state.players[i].move_tile_to_open_hand(&discarded);
                        game_state.players[i].move_tile_to_open_hand(&discarded);
                        skip_chi = true;
                        break;
                    }
                }

                // Next player may chi
                /* Need to change how this works -- strategy has to answer what straight to combine the stolen tile with so a boolean answer wont be enough
                if !skip_chi
                    && can_chi(&game_state.players[next_player_index].hand, &discarded)
                    && (game_state.players[current_player_index].strategy.call_chi)(game_state.clone())
                {
                    draw_tile(
                        &mut game_state.players[current_player_index].discards,
                        &mut game_state.players[next_player_index].hand,
                    );
                    skip_draw = true;
                    //change player's hand to open and move the chi'd set to a open section of the hand
                };
                */
                skip_chi = false;

                // Check if the wall is empty
                if game_state.wall.is_empty() {
                    round_ongoing = false;
                }
                // Pass turn to the next player
                //print_hand(&game_state.players[current_player_index].hand);
                //print_hand(&game_state.players[current_player_index].open_hand);
                //println!("hand is open: {:?}", game_state.players[current_player_index].hand_is_open());
                current_player_index = next_player_index;
            }
        }
    }
    println!("Program took {:.2?} to execute", start_time.elapsed());
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
    for i in 0..=3 {
        change_dora_bool(&mut game_state.players[i].hand, dora_suit, dora_value);
        change_dora_bool(&mut game_state.players[i].open_hand, dora_suit, dora_value);
    }
}

fn change_dora_bool(tile_list: &mut [MahjongTile], dora_suit: Suit, dora_value: u8) {
    for tile in tile_list
        .iter_mut()
        .filter(|tile| tile.suit == dora_suit && tile.value == dora_value)
    {
        tile.is_dora = true;
    }
}

fn draw_tile(hand_from: &mut Vec<MahjongTile>, hand_to: &mut Vec<MahjongTile>) {
    if let Some(tile) = hand_from.pop() {
        hand_to.push(tile);
    }
}

fn move_tile(hand_from: &mut Vec<MahjongTile>, hand_to: &mut Vec<MahjongTile>, tile_index: usize) {
    let tile = hand_from.remove(tile_index);
    hand_to.push(tile);
}

fn find_pairs_melds(hand: &[MahjongTile]) -> (Vec<Vec<MahjongTile>>, Vec<Vec<MahjongTile>>) {
    let (mut result_threes, mut result_pairs): (Vec<Vec<MahjongTile>>, Vec<Vec<MahjongTile>>) =
        (Vec::new(), Vec::new());
    let mut results = Vec::new();
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
                results.push(three_vec.clone());
                result_threes.push(three_vec);
            }
        }
        for pair in pairs {
            let pair_vec: Vec<_> = pair.into_iter().cloned().collect();

            if pair_vec.windows(2).all(|w| w[0] == w[1]) && !results.contains(&pair_vec) {
                results.push(pair_vec.clone());
                result_pairs.push(pair_vec);
            }
        }
    }
    (result_threes, result_pairs)
}

fn check_tenpai(hand: &[MahjongTile]) -> (bool, Vec<MahjongTile>) {
    let (threes, pairs) = find_pairs_melds(hand);
    let mut temp_hand = hand.to_owned();

    //Seven pairs
    if pairs.len() == 6 && threes.len() < 2 {
        for pair in &pairs {
            let is_subset = pair.iter().all(|pair| temp_hand.contains(pair));

            if is_subset {
                for tile in pair {
                    if let Some(tilepos) = temp_hand.iter().position(|x| x == tile) {
                        temp_hand.remove(tilepos);
                    }
                }
            }
        }
        return (pairs.contains(&temp_hand), temp_hand);
    }
    //Normal hands
    if threes.len() > 2 {
        for meld in &threes {
            let is_subset = meld.iter().all(|meld| temp_hand.contains(meld));

            if is_subset {
                for tile in meld {
                    if let Some(tilepos) = temp_hand.iter().position(|x| x == tile) {
                        temp_hand.remove(tilepos);
                    }
                }
            }
        }
        //Shabo
        if temp_hand.len() == 4 && pairs.len() > 1 {
            let mut paired = Vec::new();

            for pair in &pairs {
                let is_subset = temp_hand.iter().filter(|&tile| tile == &pair[0]).count() >= 2;
                if is_subset {
                    for tile in pair {
                        if let Some(tilepos) = temp_hand.iter().position(|x| x == tile) {
                            paired.push(tilepos);
                        }
                    }
                }
            }

            if paired.len() == 4 {
                temp_hand.remove(paired[0]);
                temp_hand.remove(paired[2]);
                return (true, temp_hand);
            }
        }
        let mut pair_removal_count = 0u32;

        for pair in &pairs {
            let is_subset = temp_hand.iter().filter(|&tile| tile == &pair[0]).count() >= 2;
            if is_subset {
                pair_removal_count += 1;
                for tile in pair {
                    if let Some(tilepos) = temp_hand.iter().position(|x| x == tile) {
                        temp_hand.remove(tilepos);
                    }
                }
            }
        }
        if pair_removal_count > 1 {
            return (false, temp_hand);
        }
        //Tanki
        if temp_hand.len() == 1 && pair_removal_count == 0 {
            return (true, temp_hand);
        }
        if temp_hand.len() == 2
            && temp_hand[0].suit == temp_hand[1].suit
            && temp_hand[0].suit != Suit::Sangen
            && temp_hand[0].suit != Suit::Kaze
        {
            if temp_hand[1].value - temp_hand[0].value == 1 {
                //Ghost tiles, ok if doraness is not taken into account in equivalences.

                //Ryanmen & Penchan
                let mut waits = vec![
                    MahjongTile {
                        value: temp_hand[0].value - 1,
                        suit: temp_hand[0].suit,
                        is_dora: false,
                    },
                    MahjongTile {
                        value: temp_hand[1].value + 1,
                        suit: temp_hand[1].suit,
                        is_dora: false,
                    },
                ];
                if temp_hand[1].value == 9 {
                    waits.remove(1);
                }
                if temp_hand[0].value == 1 {
                    waits.remove(0);
                }
                return (true, waits);
            }
            if temp_hand[1].value - temp_hand[0].value == 2 {
                //Kanchan
                let waits = vec![MahjongTile {
                    value: temp_hand[1].value - 1,
                    suit: temp_hand[1].suit,
                    is_dora: false,
                }];
                return (true, waits);
            }
        }
    }
    (false, temp_hand)
}

fn print_hand(tiles: &[MahjongTile]) {
    let mut result = String::new();
    let mut suits: Vec<Suit> = Vec::new();
    for suit in &[
        Suit::Manzu,
        Suit::Pinzu,
        Suit::Souzu,
        Suit::Kaze,
        Suit::Sangen,
    ] {
        let suit_tiles: Vec<&MahjongTile> = tiles.iter().filter(|t| t.suit == *suit).collect();
        if suit_tiles.is_empty() {
            continue;
        }
        result.push_str(
            &suit_tiles
                .iter()
                .map(|t| t.value.to_string())
                .collect::<String>(),
        );
        match suit {
            Suit::Manzu => result.push('m'),
            Suit::Pinzu => result.push('p'),
            Suit::Souzu => result.push('s'),
            Suit::Kaze => result.push('k'),
            Suit::Sangen => result.push('z'),
        }
        suits.push(*suit);
    }
    println!("{}", result);
}

fn print_tile(tile: &MahjongTile) {
    let mut result = String::new();
    result.push_str(&tile.value.to_string());
    match &tile.suit {
        Suit::Manzu => result.push('m'),
        Suit::Pinzu => result.push('p'),
        Suit::Souzu => result.push('s'),
        Suit::Kaze => result.push('k'),
        Suit::Sangen => result.push('z'),
    }

    println!("{}", result);
}
#[test]
#[rustfmt::skip]
fn test_find_pairs_melds() {
    let mut input = vec![
        MahjongTile { suit: Suit::Sangen, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },   
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
    ];

    let expected_output = vec![
        vec![
        MahjongTile { suit: Suit::Manzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 9, is_dora: false },
        ],
        vec![
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
        ],
        vec![
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        ],
    ];

    input.sort();
    let (test_meld1, test_meld2) = find_pairs_melds(&input);
    let mut output = Vec::new();
    output.extend(test_meld1);
    output.extend(test_meld2);

    assert_eq!(output, expected_output);

}

fn can_chi(hand: &[MahjongTile], tile: &MahjongTile) -> bool {
    if tile.suit == Suit::Kaze || tile.suit == Suit::Sangen {
        return false;
    }
    let suit_tiles: Vec<&MahjongTile> = hand
        .iter()
        .filter(|t| t.suit == tile.suit && t.value != tile.value)
        .collect();

    if suit_tiles.len() < 2 {
        return false;
    }

    // Convert values into HashSet to remove possible duplicates
    let mut values: HashSet<u8> = suit_tiles.iter().map(|t| t.value).collect();
    values.insert(tile.value);

    let mut values: Vec<u8> = values.into_iter().collect();
    values.sort();

    for i in 0..(values.len() - 2) {
        if values[i] + 1 == values[i + 1] && values[i + 1] + 1 == values[i + 2] {
            return true;
        }
    }

    false
}

fn can_pon(hand: &[MahjongTile], tile: &MahjongTile) -> bool {
    hand.iter().filter(|&t| t == tile).count() >= 2
}

#[test]
#[rustfmt::skip]
fn test_tenpai() {
    let mut input = vec![
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];

    let expected_output = vec![
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false }
        ];

    input.sort();
    let (tenpai, waits) = check_tenpai(&input);

    for tile in &waits {
        println!("{:?}", tile);
    }
    assert_eq!(waits, expected_output);
    assert_eq!(tenpai, false);

}

#[test]
#[rustfmt::skip]
fn test_shabo() {
    let mut input = vec![
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];

    let expected_output = vec![
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false }
        ];

    input.sort();
    let (tenpai, waits) = check_tenpai(&input);

    for tile in &waits {
        println!("{:?}", tile);
    }
    assert_eq!(waits, expected_output);
    assert_eq!(tenpai, true);

}

#[test]
#[rustfmt::skip]
fn test_false_shabo() {
    let mut input = vec![
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },   
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];

    let expected_output = vec![
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 7, is_dora: false }
        ];

    input.sort();
    let (tenpai, waits) = check_tenpai(&input);

    for tile in &waits {
        println!("{:?}", tile);
    }
    assert_eq!(waits, expected_output);
    assert_eq!(tenpai, false);

}

#[test]
#[rustfmt::skip]
fn test_kanchan() {
    let mut input = vec![
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];

    let expected_output = vec![
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        ];

    input.sort();
    let (tenpai, waits) = check_tenpai(&input);

    for tile in &waits {
        println!("{:?}", tile);
    }
    assert_eq!(waits, expected_output);
    assert_eq!(tenpai, true);

}
#[test]
#[rustfmt::skip]
fn test_can_chi() {
    let mut hand = vec![
        MahjongTile { suit: Suit::Sangen, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },   
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
    ];

    let mut tile1 = MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false };
    let mut tile2 = MahjongTile { suit: Suit::Sangen, value: 1, is_dora: false };
    let mut tile3 = MahjongTile { suit: Suit::Manzu, value: 7, is_dora: false };

    hand.sort();

    assert_eq!(can_chi(&hand, &tile1), true);
    assert_eq!(can_chi(&hand, &tile2), false);
    assert_eq!(can_chi(&hand, &tile3), true);

}

#[test]
#[rustfmt::skip]
fn test_can_pon() {
    let mut hand = vec![
        MahjongTile { suit: Suit::Sangen, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },   
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
    ];

    let mut tile1 = MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false };
    let mut tile2 = MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false };
    let mut tile3 = MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false };

    hand.sort();

    assert_eq!(can_pon(&hand, &tile1), true);
    assert_eq!(can_pon(&hand, &tile2), true);
    assert_eq!(can_pon(&hand, &tile3), false);

}
