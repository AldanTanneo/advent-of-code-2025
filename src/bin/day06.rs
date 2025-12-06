use std::ops::{Add, Mul};

fn main() {
    let input = aoc::input_str(6);
    let lines = input.trim_end_matches('\n').split('\n').collect::<Vec<_>>();
    let ops = lines[lines.len() - 1]
        .bytes()
        .enumerate()
        .filter(|(_, b)| *b != b' ')
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for (idx, (pos, op)) in ops.iter().enumerate() {
        let next_pos = ops
            .get(idx + 1)
            .map(|x| x.0 - 1)
            .unwrap_or_else(|| lines[0].len());
        let (func, base): (fn(u64, u64) -> u64, u64) = match op {
            b'*' => (<u64 as Mul<u64>>::mul, 1),
            b'+' => (<u64 as Add<u64>>::add, 0),
            _ => panic!("unknown operation"),
        };

        p1 += (0..lines.len() - 1)
            .map(|i| aoc::parse_dec(lines[i][*pos..next_pos].trim_ascii()))
            .fold(base, func);

        p2 += (*pos..next_pos)
            .map(|j| {
                (0..lines.len() - 1)
                    .map(|i| lines[i].as_bytes()[j])
                    .filter(|b| *b != b' ')
                    .fold(0u64, |acc, b| acc * 10 + aoc::to_dec(b) as u64)
            })
            .fold(base, func);
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
