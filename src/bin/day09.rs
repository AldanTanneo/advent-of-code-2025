use itertools::Itertools;

type Tile = (u32, u32);

fn area(a: Tile, b: Tile) -> u64 {
    (a.0.abs_diff(b.0) as u64 + 1) * (a.1.abs_diff(b.1) as u64 + 1)
}

fn crossing_rectangle(a: Tile, b: Tile, x: Tile, y: Tile) -> bool {
    if x.0 == y.0 {
        (a.0.min(b.0) + 1..a.0.max(b.0)).contains(&x.0)
            && aoc::intersects(
                a.1.min(b.1) as u64 + 1..=a.1.max(b.1) as u64 - 1,
                x.1.min(y.1) as u64..=x.1.max(y.1) as u64,
            )
    } else if x.1 == y.1 {
        (a.1.min(b.1) + 1..a.1.max(b.1)).contains(&x.1)
            && aoc::intersects(
                a.0.min(b.0) as u64 + 1..=a.0.max(b.0) as u64 - 1,
                x.0.min(y.0) as u64..=x.0.max(y.0) as u64,
            )
    } else {
        panic!("not a straight line")
    }
}

fn main() {
    let tiles = aoc::input_lines(9)
        .filter_map(|t| {
            t.split_once(',')
                .map(|(x, y)| (aoc::parse_dec(x), aoc::parse_dec(y)))
        })
        .collect::<Vec<Tile>>();

    let p1 = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            tiles
                .get(i + 1..)
                .unwrap_or_else(|| &[][..])
                .iter()
                .map(|b| area(*a, *b))
        })
        .max()
        .unwrap();

    let p2 = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            tiles
                .get(i + 1..)
                .unwrap_or_else(|| &[][..])
                .iter()
                .filter_map(|b| {
                    if tiles
                        .iter()
                        .circular_tuple_windows::<(_, _)>()
                        .any(|(x, y)| crossing_rectangle(*a, *b, *x, *y))
                    {
                        None
                    } else {
                        Some(area(*a, *b))
                    }
                })
        })
        .max()
        .unwrap();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
