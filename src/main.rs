use rand::seq::SliceRandom;

const DUPLICATE_TILES: usize = 4;

fn main() {
    let (mut player_a, mut player_b, mut player_c, mut player_d) = initialize_players();

    let (mut wall, wall_dead) = initialize_wall();

    (
        wall,
        player_a.hand,
        player_b.hand,
        player_c.hand,
        player_d.hand,
    ) = draw_hands(wall);

    println!("Wall:");

    for tile in wall {
        println!("{:?}", tile);
    }

    println!("Dead wall:");

    for tile in &wall_dead {
        println!("{:?}", tile);
    }

    println!("Player A's hand:");

    for tile in player_a.hand {
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
        .flat_map(|&x| std::iter::repeat(x).take(DUPLICATE_TILES))
        .collect();

    let mut rng = rand::thread_rng();
    wall.shuffle(&mut rng);

    let wall_dead = wall.split_off(wall.len() - 14);

    (wall, wall_dead)
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
    dicards: Vec<MahjongTile>,
    seat_wind: SeatWind,
}
impl Default for Player {
    fn default() -> Player {
        Player {
            points: 25000,
            hand: Vec::new(),
            dicards: Vec::new(),
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
