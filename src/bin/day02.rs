use hashbrown::HashSet;

fn main() {
    let input = aoc::input_str(2);
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    let mut duplicates = HashSet::new();

    for range in input.trim_ascii().split(',') {
        let (lo_str, hi_str) = range.split_once('-').unwrap();
        let lo: u64 = aoc::parse_dec(lo_str);
        let hi: u64 = aoc::parse_dec(hi_str);

        let log_lo = lo_str.len() as u32;
        let log_hi = hi_str.len() as u32;

        for log in log_lo..=log_hi {
            for q in (2..=log).filter(|q| log % q == 0) {
                let p = log / q;
                let pow = 10u64.pow(p);
                let mul = (0..q).fold(0, |acc, _| acc * pow + 1);
                let range = pow / 10..=pow - 1;

                for n in aoc::intersect(lo.div_ceil(mul)..=hi / mul, range) {
                    let res = n * mul;
                    if !duplicates.contains(&res) {
                        duplicates.insert(res);
                        if q == 2 {
                            p1 += res;
                        }
                        p2 += res;
                    }
                }
            }
        }

        duplicates.clear();
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
