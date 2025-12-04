fn count_neighbours(i: usize, j: usize, grid: &aoc::VecGrid) -> u64 {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
    .fold(0, |sum, (x, y)| {
        if let Some(b'@') = grid.get(y, x) {
            sum + 1
        } else {
            sum
        }
    })
}

fn main() {
    let input = aoc::input_bytes(4);
    let grid = aoc::VecGrid::new(input);

    let mut p1 = 0;

    for i in 0..grid.width() {
        for j in 0..grid.height() {
            if grid[j][i] == b'@' && count_neighbours(i, j, &grid) < 4 {
                p1 += 1;
            }
        }
    }

    let mut grid = grid;
    let mut p2 = 0;
    let mut added = 1;

    while added != 0 {
        added = 0;
        for i in 0..grid.width() {
            for j in 0..grid.height() {
                if grid[j][i] == b'@' && count_neighbours(i, j, &grid) < 4 {
                    added += 1;
                    grid[j][i] = b'.';
                }
            }
        }
        p2 += added;
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}
