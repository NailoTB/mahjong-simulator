use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::time::Instant;
mod types;
use types::*;
const DUPLICATE_TILES: usize = 4;
const ROUNDS: u8 = 4 * 2;
const GAMES: usize = 10;

fn main() {
    let start_time = Instant::now();
    let mut game_results: Vec<GameResult> = Vec::new();
    for game in 1..=GAMES {
        let mut players = initialize_players();

        let mut round = 0;
        while round < ROUNDS {
            let mut player_tiles = PlayerTiles::default();

            let (mut wall, wall_dead, dora_indicators) = initialize_wall();

            (
                wall,
                player_tiles.hand[0],
                player_tiles.hand[1],
                player_tiles.hand[2],
                player_tiles.hand[3],
            ) = draw_hands(wall);

            let mut board_tiles = BoardTiles {
                wall,
                wall_dead,
                dora_indicators,
                dora_index: 0,
            };

            flip_dora_indicator(&mut board_tiles, &mut player_tiles);

            for i in 0..=3 {
                player_tiles.hand[i].sort();
            }

            let mut round_ongoing = true;
            let mut skip_draw = false;
            let mut skip_chi = false;
            let mut current_player_index: usize = (round % 4).into();
            while round_ongoing {
                let next_player_index = (current_player_index + 1) % 4;
                // Current player draws a tile
                player_tiles.hand[current_player_index].sort();

                if skip_draw {
                    skip_draw = false;
                } else if board_tiles.wall.is_empty() {
                    let player_1_wind = players[0].seat_wind.clone();
                    scoring_tenpai(&mut player_tiles, &mut players);
                    if players[0].seat_wind != player_1_wind {
                        round += 1;
                    }
                    round_ongoing = false;
                    break;
                } else {
                    draw_tile(
                        &mut board_tiles.wall,
                        &mut player_tiles.hand[current_player_index],
                    );
                }

                // Current player may tsumo
                // Current player may kan
                // Current player discards a tile
                // Placeholder - Need to pass relevant vectors to strategies (hand, discards, dora indicator..)
                let strategy_input = true;

                move_tile(
                    &mut player_tiles.hand[current_player_index],
                    &mut player_tiles.discards[current_player_index],
                    (players[current_player_index].strategy.discard)(strategy_input),
                );
                let discarded = *player_tiles.discards[current_player_index].last().unwrap();

                // Other players may ron
                // Other players may pon
                for i in 0..=3 {
                    if i != current_player_index
                        && can_pon(&player_tiles.hand[i], &discarded)
                        && (players[current_player_index].strategy.call_pon)(strategy_input)
                    {
                        //println!("some guy pon'd a {:?}", discarded);
                        player_tiles.hand[i].sort();
                        remove_pon_tiles(&mut player_tiles.hand[i], &discarded);
                        for _ in 1..=3 {
                            player_tiles.open_hand[i].push(discarded);
                        }
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
                // Pass turn to the next player
                //print_hand(&game_state.players[current_player_index].hand);
                //print_hand(&game_state.players[current_player_index].open_hand);
                //println!("hand is open: {:?}", game_state.players[current_player_index].hand_is_open());
                current_player_index = next_player_index;
            }
        }
        let game_result = GameResult {
            player_1_score: players[0].points,
            player_2_score: players[1].points,
            player_3_score: players[2].points,
            player_4_score: players[3].points,
        };
        game_results.push(game_result);
    }
    for game_result in &game_results {
        println!(
            "Player 1: {}, Player 2: {}, Player 3: {}, Player 4: {}",
            game_result.player_1_score,
            game_result.player_2_score,
            game_result.player_3_score,
            game_result.player_4_score,
        );
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

fn initialize_players() -> Vec<Player> {
    let a = Player {
        ..Default::default()
    };
    let b = Player {
        seat_wind: SeatWind::South,
        ..Default::default()
    };
    let c = Player {
        seat_wind: SeatWind::West,
        ..Default::default()
    };
    let d = Player {
        seat_wind: SeatWind::North,
        ..Default::default()
    };

    vec![a, b, c, d]
}

fn draw_hands(mut wall: Vec<MahjongTile>) -> Hands {
    let a = wall.split_off(wall.len() - 13);
    let b = wall.split_off(wall.len() - 13);
    let c = wall.split_off(wall.len() - 13);
    let d = wall.split_off(wall.len() - 13);
    (wall, a, b, c, d)
}

//fn flip_dora_indicator(game_state: &mut GameState) {
fn flip_dora_indicator(board_tiles: &mut BoardTiles, player_tiles: &mut PlayerTiles) {
    let dora_indicator: &MahjongTile = &board_tiles.dora_indicators[board_tiles.dora_index];
    let dora_suit: Suit = dora_indicator.suit;

    let suit_modulo = match dora_suit {
        Suit::Manzu | Suit::Pinzu | Suit::Souzu => 9,
        Suit::Kaze => 4,
        Suit::Sangen => 3,
    };

    let dora_value: u8 = (dora_indicator.value) % (suit_modulo) + 1;

    change_dora_bool(&mut board_tiles.wall, dora_suit, dora_value);
    change_dora_bool(&mut board_tiles.wall_dead, dora_suit, dora_value);
    change_dora_bool(&mut board_tiles.dora_indicators, dora_suit, dora_value);
    for i in 0..=3 {
        change_dora_bool(&mut player_tiles.hand[i], dora_suit, dora_value);
        change_dora_bool(&mut player_tiles.open_hand[i], dora_suit, dora_value);
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
fn is_subset<T: PartialEq + Clone>(superset: &[T], subset: &[T]) -> bool {
    let mut temp_vec: Vec<T> = superset.to_vec();

    let mut removed_count = 0;
    for subset_element in subset {
        let mut found = false;
        for (i, superset_element) in temp_vec.iter().enumerate() {
            if *superset_element == *subset_element {
                temp_vec.remove(i);
                removed_count += 1;
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    removed_count == subset.len()
}

fn find_lowest_shanten_states(hand: &[MahjongTile]) -> Vec<Vec<Vec<MahjongTile>>> {
    let mut hand_structure: Vec<Vec<Vec<MahjongTile>>> = Vec::new();
    let (melds, mut pairs) = find_pairs_melds(hand);
    pairs.extend(melds);

    let plain_powerset = vector_powerset(&pairs)[1..].to_vec();
    let mut true_powerset = plain_powerset.clone();
    for meld in &pairs {
        if meld.len() == 2 {
            continue;
        }
        for subset in &plain_powerset {
            let mut extended_subset = subset.clone();
            extended_subset.push((*meld).clone());
            true_powerset.push(extended_subset);
        }
    }
    for meld_set in &true_powerset {
        let mut subset_check = true;
        let mut temp_hand = hand.to_vec();
        for meld in meld_set {
            let is_subset = is_subset(&temp_hand, meld);
            if is_subset {
                for tile in meld {
                    if let Some(tilepos) = temp_hand.iter().position(|x| x == tile) {
                        temp_hand.remove(tilepos);
                    }
                }
            } else {
                subset_check = false;
                break;
            }
        }

        if !subset_check {
            continue;
        }
        hand_structure.push(meld_set.clone());
    }

    hand_structure
}

fn vector_powerset<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<Vec<T>>> {
    if v.is_empty() {
        return vec![vec![]];
    }
    let mut ps = vector_powerset(&v[1..]);
    let item = &v[0];
    let mut new_ps = Vec::new();
    for subset in &ps {
        let mut new_subset = subset.clone();
        new_subset.push(item.clone());
        new_ps.push(new_subset);
    }
    ps.append(&mut new_ps);
    ps
}
fn clear_melds_from_hand(
    hand: &mut Vec<MahjongTile>,
    meldset: &Vec<Vec<MahjongTile>>,
    skip_pairs: bool,
) {
    for meld in meldset {
        if meld.len() == 2 && skip_pairs {
            continue;
        }

        for tile in meld {
            if let Some(tilepos) = hand.iter().position(|x| x == tile) {
                hand.remove(tilepos);
            }
        }
    }
}
fn check_tenpai(hand: &[MahjongTile]) -> (bool, Vec<MahjongTile>) {
    let temp_hand = hand.to_vec();
    let (is_tenpai, mut waits) = find_wait(&temp_hand);
    waits.sort();
    (is_tenpai, waits)
}

fn find_wait(hand: &[MahjongTile]) -> (bool, Vec<MahjongTile>) {
    let mut waits: Vec<MahjongTile> = Vec::new();
    for tile in hand {
        if tile.suit != Suit::Kaze && tile.suit != Suit::Sangen {
            for side in 0..=1 {
                let mut temp_hand = hand.to_vec();
                let test_tile = MahjongTile {
                    value: tile.value - 1 + side * 2,
                    suit: tile.suit,
                    is_dora: false,
                };
                if waits.contains(&test_tile) {
                    continue;
                }

                if (tile.value == 1 && side == 1)
                    || (tile.value == 9 && side == 0)
                    || (tile.value != 1 && tile.value != 9)
                {
                    temp_hand.push(test_tile);
                    if is_complete_alt(&temp_hand) {
                        waits.push(test_tile);
                    }
                }
            }
        }
        let mut temp_hand = hand.to_vec();
        let same_tile = MahjongTile {
            value: tile.value,
            suit: tile.suit,
            is_dora: false,
        };

        if waits.contains(&same_tile) {
            continue;
        }

        temp_hand.push(same_tile);
        if is_complete_alt(&temp_hand) {
            waits.push(same_tile);
        }
    }
    (!waits.is_empty(), waits)
}

fn is_complete(hand: &[MahjongTile]) -> bool {
    let mut first_copy = hand.to_vec();
    first_copy.sort();
    let hand_structure = find_lowest_shanten_states(&first_copy);

    for meldset in &hand_structure {
        let mut temp_hand = first_copy.to_vec();
        clear_melds_from_hand(&mut temp_hand, meldset, false);

        if temp_hand.is_empty() {
            return true;
        }
    }
    false
}
fn is_complete_alt(hand: &[MahjongTile]) -> bool {
    let mut first_copy = hand.to_vec();
    first_copy.sort();

    let (melds, mut pairs) = find_pairs_melds(&first_copy);
    pairs.extend(melds.clone());
    let n_melds = pairs.len();

    for meld1 in &melds {
        let mut second_copy = first_copy.to_vec();

        for tile in meld1 {
            if let Some(tilepos) = second_copy.iter().position(|x| x == tile) {
                second_copy.remove(tilepos);
            }
        }

        for start_index in 0..n_melds {
            let mut third_copy = second_copy.to_vec();

            for meld_index in 0..n_melds {
                let meld2 = &pairs[(start_index + meld_index) % n_melds];

                if is_subset(&third_copy, meld2) {
                    for tile in meld2 {
                        if let Some(tilepos) = third_copy.iter().position(|x| x == tile) {
                            third_copy.remove(tilepos);
                        }
                    }
                } else {
                    continue;
                }
            }
            if third_copy.is_empty() {
                return true;
            }
        }
    }
    false
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

fn remove_pon_tiles(deck: &mut Vec<MahjongTile>, card_to_remove: &MahjongTile) {
    let mut tiles_removed = 0;
    let mut i = 0;
    while i < deck.len() && tiles_removed < 2 {
        if &deck[i] == card_to_remove {
            deck.remove(i);
            tiles_removed += 1;
        } else {
            i += 1;
        }
    }
}

fn scoring_tenpai(player_tiles: &mut PlayerTiles, players: &mut Vec<Player>) {
    let mut tenpai_players = 0;
    let mut noten_players = 0;
    for i in 0..=3 {
        let (got_tenpai, _) = check_tenpai(&player_tiles.hand[i]);
        if got_tenpai {
            tenpai_players += 1;
        } else {
            noten_players += 1;
        }
    }

    let mut change_winds = true;

    if tenpai_players > 0 {
        let winner_payout = 3000 / tenpai_players;
        for i in 0..=3 {
            let (got_tenpai, _) = check_tenpai(&player_tiles.hand[i]);
            if got_tenpai {
                players[i].points += winner_payout;
                if players[i].seat_wind == SeatWind::East {
                    change_winds = false;
                }
            } else {
                players[i].points -= winner_payout / noten_players;
            }
        }
    }

    if change_winds {
        for player in players {
            player.next_wind();
        }
    }
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

    let expected_output = vec![];

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

    let expected_output = vec![];

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
        MahjongTile { suit: Suit::Pinzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },   
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 8, is_dora: false },
    ];

    let mut tile1 = MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false };
    let mut tile2 = MahjongTile { suit: Suit::Sangen, value: 1, is_dora: false };
    let mut tile3 = MahjongTile { suit: Suit::Manzu, value: 7, is_dora: false };
    let mut tile4 = MahjongTile { suit: Suit::Pinzu, value: 7, is_dora: false };

    hand.sort();

    assert_eq!(can_chi(&hand, &tile1), true);
    assert_eq!(can_chi(&hand, &tile2), false);
    assert_eq!(can_chi(&hand, &tile3), true);
    assert_eq!(can_chi(&hand, &tile4), true);

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

#[test]
#[rustfmt::skip]
fn test_super_tenpai() {

    let mut hand = vec![
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
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
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },

        ];

    hand.sort();
    let (tenpai1, waits2) = check_tenpai(&hand);
    print_hand(&hand);
    print_hand(&waits2);
    assert_eq!(waits2, expected_output);
    assert_eq!(tenpai1, true);

}

#[test]
#[rustfmt::skip]
fn test_super_tenpai_sanmen() {

    let mut hand = vec![
        MahjongTile { suit: Suit::Pinzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];

    let expected_output = vec![
        MahjongTile { suit: Suit::Pinzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 7, is_dora: false },

        ];

    hand.sort();
    let (tenpai1, waits2) = check_tenpai(&hand);
    print_hand(&hand);
    print_hand(&waits2);
    assert_eq!(waits2, expected_output);
    assert_eq!(tenpai1, true);

}

#[test]
#[rustfmt::skip]
fn test_super_tenpai_happoubijin() {
    let start_time = Instant::now();

    let mut hand = vec![
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 7, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];

    let expected_output = vec![
        MahjongTile { suit: Suit::Souzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },
        ];

    hand.sort();
    let (tenpai1, mut waits2) = check_tenpai(&hand);
    print_hand(&hand);
    print_hand(&waits2);
    assert_eq!(waits2, expected_output);
    assert_eq!(tenpai1, true);
    println!("Test took {:.2?} to execute", start_time.elapsed());

}
