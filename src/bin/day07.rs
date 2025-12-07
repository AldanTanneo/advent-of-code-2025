use hashbrown::HashMap;

fn main() {
    let manifold = aoc::VecGrid::new(aoc::input_bytes(7));

    let start = manifold[0].iter().position(|b| *b == b'S').unwrap();
    let mut map = HashMap::<usize, u64>::from([(start, 1)]);
    let mut old_map = HashMap::new();

    let mut p1: u32 = 0;

    for line in manifold.iter().skip(1) {
        (map, old_map) = (old_map, map);
        for (beam, num) in old_map.drain() {
            match line[beam] {
                b'.' => {
                    *map.entry(beam).or_default() += num;
                }
                b'^' => {
                    p1 += 1;
                    *map.entry(beam - 1).or_default() += num;
                    *map.entry(beam + 1).or_default() += num;
                }
                _ => panic!("unexpected element"),
            }
        }
    }

    let p2 = map.values().sum::<u64>();

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
