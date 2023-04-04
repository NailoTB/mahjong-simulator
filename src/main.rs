use rand::seq::SliceRandom;

fn main() {
    //let wall = initialize_wall();
    let (wall, wall_dead) = initialize_wall();

    println!("Wall:");

    for tile in &wall {
        println!("{:?}", tile);
    }

    println!("Dead wall:");

    for tile in &wall_dead {
        println!("{:?}", tile);
    }
}

#[derive(Debug, Copy, Clone)]
enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Kaze,
    Sangen,
}

#[derive(Debug, Copy, Clone)]
struct MahjongTile {
    suit: Suit,
    value: u8,
    is_dora: bool,
}

fn initialize_wall() -> (Vec<MahjongTile>, Vec<MahjongTile>) {
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
        .flat_map(|&x| std::iter::repeat(x).take(4))
        .collect();

    let mut rng = rand::thread_rng();
    wall.shuffle(&mut rng);

    let wall_dead = wall.split_off(wall.len() - 14);

    (wall, wall_dead)
}
