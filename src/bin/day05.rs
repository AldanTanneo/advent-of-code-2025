use std::ops::RangeInclusive;

fn main() {
    let input = aoc::input_str(5);
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .split_ascii_whitespace()
        .filter_map(|r| r.split_once('-'))
        .map(|(lo, hi)| aoc::parse_dec::<u64>(lo)..=aoc::parse_dec(hi))
        .collect::<Vec<_>>();
    let ids = ids
        .split_ascii_whitespace()
        .map(aoc::parse_dec::<u64>)
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|r| *r.start());
    let ranges = ranges;
    let mut union: Vec<RangeInclusive<u64>> = vec![];
    for r in ranges {
        if let Some(u) = union.last_mut()
            && u.end() >= r.start()
        {
            *u = *u.start()..=*r.end().max(u.end());
        } else {
            union.push(r);
        }
    }

    let p1 = ids
        .iter()
        .filter(|id| union.iter().any(|range| range.contains(id)))
        .count();

    let p2 = union.into_iter().map(|r| r.count()).sum::<usize>();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
