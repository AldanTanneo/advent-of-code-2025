use std::ops::RangeInclusive;

fn intersect(a: RangeInclusive<u64>, b: RangeInclusive<u64>) -> RangeInclusive<u64> {
    *a.start().max(b.start())..=*a.end().min(b.end())
}

fn main() {
    let input = aoc::input_str(2);
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    for range in input.trim_ascii().split(',') {
        let (lo, hi) = range.split_once('-').unwrap();
        let lo: u64 = aoc::parse_dec(lo);
        let hi: u64 = aoc::parse_dec(hi);

        let log_lo = lo.ilog10() + 1;
        let log_hi = hi.ilog10() + 1;

        let mut duplicates = vec![];

        for log in log_lo..=log_hi {
            for q in (2..=log).filter(|q| log % q == 0) {
                let mul = (10u128.pow((log / q) * q) - 1) / (10u128.pow(log / q) - 1);
                let mul = mul as u64;
                let range = 10u64.pow(log / q - 1)..=10u64.pow(log / q) - 1;

                for n in intersect(lo.div_ceil(mul)..=hi / mul, range) {
                    let res = n * mul;
                    if let Err(idx) = duplicates.binary_search(&res) {
                        duplicates.insert(idx, res);
                        if q == 2 {
                            p1 += res;
                        }
                        p2 += res;
                    }
                }
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
