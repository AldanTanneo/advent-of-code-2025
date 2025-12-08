use hashbrown::HashSet;

type Box3D = (u32, u32, u32);

fn distance(a: &Box3D, b: &Box3D) -> u64 {
    (a.0.abs_diff(b.0) as u64).pow(2)
        + (a.1.abs_diff(b.1) as u64).pow(2)
        + (a.2.abs_diff(b.2) as u64).pow(2)
}

const FIRST_N: usize = 1000;

fn main() {
    let boxes: Vec<Box3D> = aoc::input_lines(8)
        .map(|b| {
            let (x, rest) = b.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            (aoc::parse_dec(x), aoc::parse_dec(y), aoc::parse_dec(z))
        })
        .collect();

    let mut connections = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            boxes
                .get(i + 1..)
                .unwrap_or(&[])
                .into_iter()
                .map(move |b| (a, b))
        })
        .filter(|(a, b)| a != b)
        .collect::<Vec<_>>();
    connections.sort_unstable_by_key(|&(a, b)| distance(a, b));

    let mut circuits: Vec<HashSet<Box3D>> = vec![];
    for &(a, b) in connections.iter().take(FIRST_N) {
        let mut joined = HashSet::from([*a, *b]);
        for set in circuits.extract_if(.., |h| h.contains(a) || h.contains(b)) {
            joined.extend(set);
        }
        circuits.push(joined);
    }
    circuits.sort_unstable_by_key(|h| h.len());

    let p1 = circuits
        .iter()
        .rev()
        .take(3)
        .fold(1, |prod, h| prod * h.len());

    let mut p2 = 0;
    for &(a, b) in connections.iter().skip(FIRST_N) {
        let mut joined = HashSet::from([*a, *b]);
        for set in circuits.extract_if(.., |h| h.contains(a) || h.contains(b)) {
            joined.extend(set);
        }
        circuits.push(joined);

        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            p2 = a.0 as u64 * b.0 as u64;
            break;
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
