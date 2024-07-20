use rand::random;
use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 35;
const HEIGHT: usize = 35;

fn main() {
    let mut map: Vec<bool> = vec![false; WIDTH * HEIGHT];

    for i in 0..(WIDTH * HEIGHT) {
        map[i] = random();
    }

    // Start game loop
    print_map(&mut map);

    loop {
        sleep(Duration::from_millis(500));
        step_map(&mut map);
        println!();
        print_map(&mut map);
    }
}

fn tile(state: bool) -> &'static str {
    if state {
        "██"
    } else {
        "░░"
    }
}

fn get_map_index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn print_map(map: &mut Vec<bool>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", tile(map[get_map_index(x, y)]))
        }
        println!()
    }
}

fn step_map(map: &mut Vec<bool>) {
    let mut new_map: Vec<bool> = vec![false; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Count the number of neighbors
            let i: usize = get_map_index(x, y);
            let mut neighbors: u8 = 0;

            for dx in -1..=1_isize {
                for dy in -1..=1_isize {
                    // Exclude the current cell
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    // Get the neighboring cell position
                    let px: isize = x as isize + dx;
                    let py: isize = y as isize + dy;

                    // Check if the neighboring cell is out of bounds
                    if px < 0 || py < 0 {
                        continue;
                    }

                    let px: usize = px as usize;
                    let py: usize = py as usize;

                    if px >= WIDTH || py >= HEIGHT {
                        continue;
                    }

                    // Check if the cell is alive
                    let i: usize = get_map_index(px, py);

                    if map[i] {
                        neighbors += 1;
                    }
                }
            }

            if map[i] {
                // Alive cell rules
                if neighbors == 2 || neighbors == 3 {
                    new_map[i] = true
                }
            } else {
                // Dead cell rules
                if neighbors == 3 {
                    new_map[i] = true
                }
            }
        }
    }

    *map = new_map;
}
