use std::collections::{HashMap, HashSet};

struct Knight {
    last_first: String,
    x: i32,
    y: i32,
    allowable_moves: [(i32, i32); 8],
}

impl Knight {
    fn new(x: i32, y: i32, name: String) -> Knight {
        Knight {
            last_first: name,
            x,
            y,
            allowable_moves: [
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ],
        }
    }

    fn solve(
        &self,
        knight: &Knight,
        grid: &[[char; 5]; 5],
        path: &mut Vec<(i32, i32)>,
        visited: &mut HashSet<(i32, i32)>,
        vowel_count: usize,
        vowel_map: &HashMap<char, bool>,
        grid_size: i32,
        total_paths: &mut usize,
    ) {
        // If we've visited 10 cells, count this as a valid path
        if path.len() == 10 {
            *total_paths += 1;
            return;
        }

        // Try all possible knight moves
        for (dx, dy) in knight.allowable_moves.iter() {
            let new_x = knight.x + dx;
            let new_y = knight.y + dy;

            // Check if the new move is within boundaries
            if is_within_boundary(new_x, new_y, grid_size) {
                let cell = grid[new_x as usize][new_y as usize];
                let mut new_vowel_count = vowel_count;

                // Check if the cell is empty
                if is_empty_space(new_x, new_y, grid) {
                    continue; // Skip this move
                }

                // Check if the cell is a vowel and if adding it exceeds the vowel limit
                if vowel_map.contains_key(&cell) {
                    if vowel_count >= 2 {
                        continue; // Skip this move
                    }
                    new_vowel_count += 1;
                }

                // Mark the current cell as visited
                visited.insert((new_x, new_y));
                path.push((new_x, new_y));

                // Create a new Knight instance with updated position
                let new_knight = Knight::new(new_x, new_y, knight.last_first.clone());

                // Recursively check for further moves & paths
                self.solve(
                    &new_knight,
                    grid,
                    path,
                    visited,
                    new_vowel_count,
                    vowel_map,
                    grid_size,
                    total_paths,
                );

                path.pop();
                visited.remove(&(new_x, new_y));
            }
        }
    }
}

fn main() {
    // Create Grid
    let grid = generate_grid();
    let grid_size: i32 = 5; // Known from AC

    // Create vowel map
    let vowel_map = generate_vowel_map();

    // Track total paths
    let mut total_paths = 0;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let mut visited = HashSet::new();
            let mut path = vec![(i as i32, j as i32)];
            let vowel_count = if vowel_map.contains_key(&grid[i as usize][j as usize]) {
                1
            } else {
                0
            };

            let knight = Knight::new(
                i.try_into().unwrap(),
                j.try_into().unwrap(),
                String::from("jutziJack"),
            );

            // Being calculating paths
            knight.solve(
                &knight,
                &grid,
                &mut path,
                &mut visited,
                vowel_count,
                &vowel_map,
                grid_size,
                &mut total_paths,
            );
        }
    }

    println!("Total paths: {}", total_paths);
}

fn is_within_boundary(x: i32, y: i32, grid_size: i32) -> bool {
    x >= 0 && x < grid_size && y >= 0 && y < grid_size // only works if grid is square, otherwise refactor to allow x & y sizes
}

fn is_empty_space(x: i32, y: i32, grid: &[[char; 5]; 5]) -> bool {
    grid[x as usize][y as usize] != 'x'
}

fn generate_vowel_map() -> HashMap<char, bool> {
    let mut map = HashMap::new();
    map.insert('a', true);
    map.insert('e', true);
    map.insert('i', true);
    map.insert('o', true);
    map.insert('u', true);
    map.insert('y', true);
    return map;
}

fn generate_grid() -> [[char; 5]; 5] {
    let grid: [[char; 5]; 5] = [
        ['a', 'b', 'c', 'x', 'e'],
        ['x', 'g', 'h', 'i', 'j'],
        ['k', 'l', 'x', 'n', 'o'],
        ['p', 'q', 'r', 's', 't'],
        ['u', 'v', 'x', 'x', 'y'],
    ];
    return grid;
}
