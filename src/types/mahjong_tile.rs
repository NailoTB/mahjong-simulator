use itertools::Itertools;
use rand::seq::SliceRandom;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashSet;
const DUPLICATE_TILES: usize = 4;

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

pub type Hands = (
    Vec<MahjongTile>, // Wall
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
    Vec<MahjongTile>,
);

pub fn initialize_wall() -> (Vec<MahjongTile>, Vec<MahjongTile>, Vec<MahjongTile>) {
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

pub fn draw_hands(mut wall: Vec<MahjongTile>) -> Hands {
    let a = wall.split_off(wall.len() - 13);
    let b = wall.split_off(wall.len() - 13);
    let c = wall.split_off(wall.len() - 13);
    let d = wall.split_off(wall.len() - 13);
    (wall, a, b, c, d)
}

pub fn draw_tile(hand_from: &mut Vec<MahjongTile>, hand_to: &mut Vec<MahjongTile>) {
    if let Some(tile) = hand_from.pop() {
        hand_to.push(tile);
    }
}

pub fn move_tile(
    hand_from: &mut Vec<MahjongTile>,
    hand_to: &mut Vec<MahjongTile>,
    tile_index: usize,
) {
    let tile = hand_from.remove(tile_index);
    hand_to.push(tile);
}

pub fn print_hand(tiles: &[MahjongTile]) {
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

pub fn print_tile(tile: &MahjongTile) {
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

pub fn find_tile_in_hand(hand: &[MahjongTile], tile: &MahjongTile) -> usize {
    for (tile_index, _) in hand.iter().enumerate() {
        if tile == &hand[tile_index] {
            return tile_index;
        }
    }
    println!("tile not found");
    14
}
pub fn find_pairs_melds(hand: &[MahjongTile]) -> (Vec<Vec<MahjongTile>>, Vec<Vec<MahjongTile>>) {
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
pub fn is_subset<T: PartialEq + Clone>(superset: &[T], subset: &[T]) -> bool {
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

pub fn check_tenpai(hand: &[MahjongTile]) -> (bool, Vec<MahjongTile>) {
    let temp_hand = hand.to_vec();
    let (is_tenpai, mut waits) = find_wait(&temp_hand);
    waits.sort();
    (is_tenpai, waits)
}

pub fn find_wait(hand: &[MahjongTile]) -> (bool, Vec<MahjongTile>) {
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
                    if is_complete(&temp_hand) {
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
        if is_complete(&temp_hand) {
            waits.push(same_tile);
        }
    }
    (!waits.is_empty(), waits)
}

pub fn get_partial_completion(hand: &[MahjongTile]) -> Vec<MahjongTile> {
    let mut first_copy = hand.to_vec();
    let mut partial_hand = hand.to_vec();

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
            let mut pair_counter = 0;

            for meld_index in 0..n_melds {
                let meld2 = &pairs[(start_index + meld_index) % n_melds];
                if meld2.len() == 2{
                    pair_counter += 1;
                }
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
            if third_copy.len() < partial_hand.len() && pair_counter <= 2 {
                partial_hand = third_copy;
            }
        }
    }
    partial_hand
}

pub fn is_complete(hand: &[MahjongTile]) -> bool {
    let mut first_copy = hand.to_vec();
    first_copy.sort();

    let (melds, mut pairs) = find_pairs_melds(&first_copy);

    if melds.len() < 2 {
        //gotta be careful, since ryanpeikou has the same shape
        let mut second_copy = first_copy.to_vec();
        for pair in &pairs {
            for tile in pair {
                if let Some(tilepos) = second_copy.iter().position(|x| x == tile) {
                    second_copy.remove(tilepos);
                }
            }
        }
        return second_copy.is_empty();
    }

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
            let mut pair_counter = 0;
            let mut third_copy = second_copy.to_vec();
            for meld_index in 0..n_melds {
                let meld2 = &pairs[(start_index + meld_index) % n_melds];

                if is_subset(&third_copy, meld2) {
                    if meld2.len() == 2 {
                        pair_counter += 1;
                    }
                    for tile in meld2 {
                        if let Some(tilepos) = third_copy.iter().position(|x| x == tile) {
                            third_copy.remove(tilepos);
                        }
                    }
                } else {
                    continue;
                }
            }
            if third_copy.is_empty() && pair_counter == 1 {
                return true;
            }
        }
    }
    false
}

pub fn construct_unique_meld_set(hand: &[MahjongTile]) -> Vec<Vec<MahjongTile>> {
    let mut first_copy = hand.to_vec();
    first_copy.sort();

    let (melds, mut pairs) = find_pairs_melds(&first_copy);

    pairs.extend(melds.clone());
    let n_melds = pairs.len();
    let mut result_tensor = Vec::new();

    for meld1 in &melds {
        let mut second_copy = first_copy.to_vec();
        for tile in meld1 {
            if let Some(tilepos) = second_copy.iter().position(|x| x == tile) {
                second_copy.remove(tilepos);
            }
        }

        for start_index in 0..n_melds {
            let mut pair_counter = 0;
            let mut results = Vec::new();
            results.push(meld1.clone());

            let mut third_copy = second_copy.to_vec();
            for meld_index in 0..n_melds {
                let meld2 = &pairs[(start_index + meld_index) % n_melds];

                if is_subset(&third_copy, meld2) {
                    if meld2.len() == 2 {
                        pair_counter += 1;
                    }
                    results.push(meld2.clone());

                    for tile in meld2 {
                        if let Some(tilepos) = third_copy.iter().position(|x| x == tile) {
                            third_copy.remove(tilepos);
                        }
                    }
                } else {
                    continue;
                }
            }
            if third_copy.is_empty() && pair_counter == 1 {
                result_tensor.push(results);
            }
        }
    }
    let mut cleaned_tensor: Vec<Vec<_>> = result_tensor
        .iter()
        .map(|inner_vec| {
            let mut cloned_vec = inner_vec.clone();
            cloned_vec.sort();
            cloned_vec
        })
        .collect();

    cleaned_tensor.sort();
    cleaned_tensor.dedup();
    // if cleaned_tensor.len() != 1 {
    //     println!("Omg you found a rare hand!!");
    //     println!("Printing meldtensor:");
    //     for meldlist in &cleaned_tensor {
    //         println!("Melds:");
    //         for meld in meldlist {
    //             print_hand(&meld);
    //         }
    //     }
    // }
    if cleaned_tensor.is_empty() {
        println!("Wtf there is no hand");
        print_hand(hand);
    }
    cleaned_tensor[0].clone()
}

pub fn remove_pon_tiles(deck: &mut Vec<MahjongTile>, card_to_remove: &MahjongTile) {
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

pub fn can_chi(hand: &[MahjongTile], tile: &MahjongTile) -> bool {
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

pub fn can_pon(hand: &[MahjongTile], tile: &MahjongTile) -> bool {
    hand.iter().filter(|&t| t == tile).count() >= 2
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
    let (tenpai1, waits2) = check_tenpai(&hand);
    print_hand(&hand);
    print_hand(&waits2);
    assert_eq!(waits2, expected_output);
    assert_eq!(tenpai1, true);

}
#[test]
#[rustfmt::skip]
fn chiitoi_completion() {

    let mut hand = vec![
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 8, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 9, is_dora: false },   
        MahjongTile { suit: Suit::Pinzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Sangen, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Kaze, value: 1, is_dora: false },
    ];
    print_hand(&hand);
    hand.sort();
    let complete = is_complete(&hand);
    assert_eq!(complete, true);

}
#[test]
#[rustfmt::skip]
fn pair_mix_completion() {
        //1112344m33p22345s
    let mut hand = vec![
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
    ];
    print_hand(&hand);
    hand.sort();
    let complete = is_complete(&hand);
    assert_eq!(complete, false);

}
#[test]
#[rustfmt::skip]
fn test_nobetan() {
        //1112344m33p22345s
    let mut hand = vec![
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
    ];
    print_hand(&hand);
    hand.sort();


    let expected_output = vec![
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },
        ];


    let (_, waits) = check_tenpai(&hand);
    assert_eq!(waits, expected_output);

}

#[test]
#[rustfmt::skip]
fn test_hand_construction() {
    let mut hand = vec![
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
    ];
    print_hand(&hand);
    hand.sort();

    let hand_melds = construct_unique_meld_set(&hand);
    for meld in hand_melds {
        print_hand(&meld);
}
    let expected_output = vec![
        MahjongTile { suit: Suit::Manzu, value: 1, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },
        ];

}

#[test]
#[rustfmt::skip]
fn test_hand_iipeikou_const() {
    let mut hand = vec![
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 4, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Pinzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 2, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 3, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 4, is_dora: false },   
        MahjongTile { suit: Suit::Souzu, value: 5, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 6, is_dora: false },
        MahjongTile { suit: Suit::Souzu, value: 7, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 9, is_dora: false },
        MahjongTile { suit: Suit::Manzu, value: 9, is_dora: false },
    ];
    print_hand(&hand);
    hand.sort();

    let hand_melds = construct_unique_meld_set(&hand);
    for meld in hand_melds {
        print_hand(&meld);
}


}