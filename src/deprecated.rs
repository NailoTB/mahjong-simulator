
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
fn is_complete_slow(hand: &[MahjongTile]) -> bool {
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