use std::cmp::Reverse;

fn main() {
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    for line in aoc::input_lines(3) {
        let batteries: Vec<_> = line.into_bytes().into_iter().map(aoc::to_dec).collect();
        let n = batteries.len();

        let max = batteries[..n - 1]
            .iter()
            .enumerate()
            .map(|(i, elt)| *elt * 10 + batteries[i + 1..].iter().max().unwrap())
            .max()
            .unwrap();
        p1 += max as u64;

        let (mut j, mut max) = (0, 0);
        for i in (0..12).rev() {
            (j, max) = batteries[j..n - i]
                .iter()
                .enumerate()
                .map(|(idx, elt)| (j + idx + 1, max * 10 + *elt as u64))
                .max_by_key(|(idx, elt)| (*elt, Reverse(*idx)))
                .unwrap();
        }
        p2 += max;
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
