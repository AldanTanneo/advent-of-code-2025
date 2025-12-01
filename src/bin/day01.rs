fn main() {
    let input = aoc::input_str(1);

    let mut pos: i32 = 50;
    let mut p1 = 0;
    let mut p2 = 0;

    input.split_ascii_whitespace().for_each(|l| {
        let mv = match l.as_bytes() {
            [b'L', n @ ..] => -aoc::parse_dec::<i32>(n),
            [b'R', n @ ..] => aoc::parse_dec::<i32>(n),
            _ => panic!("unknown direction"),
        };
        let old_pos = pos;
        pos += mv;

        p2 += pos.abs().div_euclid(100) + if old_pos != 0 && pos <= 0 { 1 } else { 0 };
        pos = pos.rem_euclid(100);

        if pos == 0 {
            p1 += 1;
        }
    });

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
