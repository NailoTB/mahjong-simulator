use std::time::Instant;
mod types;
use types::*;
use types::mahjong_tile::*;
const ROUNDS: u8 = 4 * 2;
const GAMES: usize = 10;

fn main() {
    let start_time = Instant::now();
    let mut game_results: Vec<GameResult> = Vec::new();
    for game in 1..=GAMES {
        let mut players = initialize_players();

        let mut round = 0;
        'rounds: while round < ROUNDS {
            for player in players.iter().take(3 + 1) {
                if player.points < 0 {
                    break 'rounds;
                }
            }
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
            'round: while round_ongoing {
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
                    break 'round;
                } else {
                    draw_tile(
                        &mut board_tiles.wall,
                        &mut player_tiles.hand[current_player_index],
                    );
                }

                let strategy_input = StrategyInput {
                    hand: player_tiles.hand[current_player_index].clone(),
                    discards: player_tiles.discards.clone(),
                };

                // Current player may tsumo
                if is_complete(&player_tiles.hand[current_player_index])
                    && (players[current_player_index].strategy.tsumo)(strategy_input.clone())
                {
                    scoring_tsumo(&mut player_tiles, &mut players, current_player_index);
                    break 'round;
                }
                // Current player may kan
                // Current player discards a tile
                // Placeholder - Need to pass relevant vectors to strategies (hand, discards, dora indicator..)
                //let strategy_input = true;

                move_tile(
                    &mut player_tiles.hand[current_player_index],
                    &mut player_tiles.discards[current_player_index],
                    (players[current_player_index].strategy.discard)(strategy_input.clone()),
                );
                let discarded = *player_tiles.discards[current_player_index].last().unwrap();

                // Other players may ron
                // Other players may pon
                for i in 0..=3 {
                    if i != current_player_index
                        && can_pon(&player_tiles.hand[i], &discarded)
                        && (players[current_player_index].strategy.call_pon)(strategy_input.clone())
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
            "Player 1: {}, Player 2: {}, Player 3: {}, Player 4: {}, Total points deviation: {}",
            game_result.player_1_score,
            game_result.player_2_score,
            game_result.player_3_score,
            game_result.player_4_score,
            game_result.player_1_score
                + game_result.player_2_score
                + game_result.player_3_score
                + game_result.player_4_score
                - 100000, // Should be 0
        );
    }

    println!("Program took {:.2?} to execute", start_time.elapsed());
}

fn initialize_players() -> Vec<Player> {
    let pinfu = Strategy {
        discard: pinfu_hunter,
        call_chi: never_open_hand,
        call_pon: never_open_hand,
        ..Default::default()
    };
    let standard = Strategy {
        discard: standard_discarder,
        call_chi: never_open_hand,
        call_pon: never_open_hand,
        ..Default::default()
    };
    let a = Player {
        strategy: standard.clone(),
        ..Default::default()
    };
    let b = Player {
        seat_wind: SeatWind::South,
        strategy: pinfu,
        ..Default::default()
    };
    let c = Player {
        seat_wind: SeatWind::West,
        strategy: standard.clone(),
        ..Default::default()
    };
    let d = Player {
        seat_wind: SeatWind::North,
        strategy: standard,
        ..Default::default()
    };

    vec![a, b, c, d]
}

fn never_open_hand(_strat: StrategyInput) -> bool {
    false
}

fn pinfu_hunter(strat: StrategyInput) -> usize {
    let mut own_hand = strat.hand.clone();
    own_hand.sort();
    let partial_hand = get_partial_completion(&own_hand);
    if partial_hand.is_empty() {
        //println!("Partial hand is empty, hand was complete!");
        return 13;
    }
    if partial_hand.len() == 1 {
        return find_tile_in_hand(&strat.hand, &partial_hand[0]);
    }

    for tile in &partial_hand {
        if tile.suit == Suit::Sangen || tile.suit == Suit::Kaze {
            return find_tile_in_hand(&strat.hand, tile);
        }
    }
    let mut skip_following = false;
    for tile_index in 0..partial_hand.len() - 1 {
        let tile = &partial_hand[tile_index];
        let right = &partial_hand[tile_index + 1];
        if skip_following {
            skip_following = false;
            continue;
        }
        if tile.value + 1 == right.value && tile.suit == right.suit {
            //keep the tile and the next tile
            skip_following = true;
            continue;
        }
        return find_tile_in_hand(&strat.hand, tile);
    }
    for tile in &partial_hand {
        if tile.value == 1 || tile.value == 9 {
            return find_tile_in_hand(&strat.hand, tile);
        }
    }
    find_tile_in_hand(&strat.hand, &partial_hand[partial_hand.len() - 1])
}
fn standard_discarder(strat: StrategyInput) -> usize {
    let mut own_hand = strat.hand.clone();
    own_hand.sort();
    let partial_hand = get_partial_completion(&own_hand);
    if partial_hand.is_empty() {
        //println!("Partial hand is empty, hand was complete!");
        return 13;
    }
    find_tile_in_hand(&strat.hand, &partial_hand[partial_hand.len() - 1])
}

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

    if tenpai_players == 4 {
        change_winds = false;
    }

    if tenpai_players != 4 && noten_players != 4 {
        let winner_payout = 3000 / tenpai_players;
        for (index, player) in players.iter_mut().enumerate().take(3 + 1) {
            let (got_tenpai, _) = check_tenpai(&player_tiles.hand[index]);
            if got_tenpai {
                player.points += winner_payout;
                if player.seat_wind == SeatWind::East {
                    change_winds = false;
                }
            } else if noten_players == 2 {
                player.points -= winner_payout;
            } else if noten_players == 1 {
                player.points -= winner_payout * tenpai_players;
            } else {
                player.points -= winner_payout / noten_players;
            }
        }
    }

    if change_winds {
        for player in players {
            player.next_wind();
        }
    }
}

fn scoring_tsumo(
    player_tiles: &mut PlayerTiles,
    players: &mut Vec<Player>,
    winning_player_index: usize,
) {
    let is_dealer_win = players[winning_player_index].seat_wind == SeatWind::East;

    let (base_score, dealer_score) = calculate_hand_score(&player_tiles.hand[winning_player_index]);

    for (index, player) in players.iter_mut().enumerate().take(3 + 1) {
        if index == winning_player_index {
            match is_dealer_win {
                true => player.points += dealer_score * 3,
                false => player.points += dealer_score + base_score * 2,
            }
        } else if is_dealer_win || player.seat_wind == SeatWind::East {
            player.points -= dealer_score;
        } else {
            player.points -= base_score;
        }
    }

    if !is_dealer_win {
        for player in players {
            player.next_wind();
        }
    }
}

fn calculate_hand_score(hand: &[MahjongTile]) -> (i32, i32) {
    // TODO: Implement proper hand scoring calculation
    (1000, 2000)
}
