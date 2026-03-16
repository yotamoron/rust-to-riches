use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn init_grid(grid: &mut Vec<[bool; 2]>) {
    (0..grid.len()).for_each(|idx| {
        let dice = rand::random_range(1..=100);
        if dice < 80 {
            let cell = grid.get_mut(idx as usize).unwrap();
            cell[0 as usize] = true;
        }
    });
}

fn get_neighbours_by_index(square_side_size: i32, vector_length: i32) -> HashMap<i32, Vec<i32>>{
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    (0..vector_length).for_each(|idx| {
        let row = idx / square_side_size;
        let column = idx % square_side_size;
        let mut neighbors: Vec<i32> = Vec::new();

        (-1..=1).for_each(|row_offset| {
            (-1..=1).for_each(|col_offset| {
                if row_offset != 0 || col_offset != 0 {
                    let new_row = row + row_offset;
                    let new_col = column + col_offset;

                    if 0 <= new_row && new_row < square_side_size && 0 <= new_col && new_col < square_side_size {
                        neighbors.push(new_row * square_side_size + new_col);
                    } 
                }
            });
        });

        map.insert(idx, neighbors);
    });
    map

}

fn display_current_gen(grid: &Vec<[bool; 2]>, current_gen: i32, square_side_size: i32) {
    let mut rendering = String::from("\x1B[2J\x1B[H");

    (0..square_side_size).for_each(|row| {
        (0..square_side_size).for_each(|col| {
            let idx:usize = (row * square_side_size + col) as usize;

            let cell = grid[idx];
            if cell[current_gen as usize] {
                rendering.push_str("X");
            } else {
                rendering.push_str(" ");
            }
        });
        rendering.push_str("\n");
    });
    println!("{rendering}");
}

fn calculate_next_life_status<'a, F>(
    current_cell_is_alive: bool,
    current_gen: i32, 
    cell_neighbors_indexes: &Vec<i32>,
    get_neighbor: F,
)  -> bool where F: Fn(usize) -> Option<&'a [bool; 2]> {
    let mut total_live = 0;

    for idx in cell_neighbors_indexes {
        if let Some(neighbor_life_status) = get_neighbor(*idx as usize) {
            if neighbor_life_status[current_gen as usize] {
                total_live += 1;
            }
        }
    }

    if current_cell_is_alive && (total_live < 2 || total_live > 3) {
        return false;
    }
    if !current_cell_is_alive && total_live == 3 {
        return true;
    }
    return current_cell_is_alive;
}

fn calculate_next_gen(grid: &mut Vec<[bool; 2]>, current_gen: i32, next_gen: i32, neighbors: &HashMap<i32, Vec<i32>>) {
    let len = grid.len() as i32;
    (0..len).for_each(|idx| {
        let cell = grid.get(idx as usize).unwrap();
        let cell_neighbors_indexes = neighbors.get(&idx).unwrap();
        let is_alive = calculate_next_life_status(
            cell[current_gen as usize], 
            current_gen, 
            cell_neighbors_indexes,
            |idx| grid.get(idx)
        );
        let cell = grid.get_mut(idx as usize).unwrap();
        
        cell[next_gen as usize] = is_alive;
    });
}

fn main() {
    let square_side_size = 150;
    let vector_length = square_side_size * square_side_size;
    let mut grid: Vec<[bool; 2]> = (0..vector_length).map(|_| [false, false]).collect();
    let neighbors = get_neighbours_by_index(square_side_size, vector_length);
    let mut current_gen = 0;

    init_grid(&mut grid);

    loop {
        let next_gen = (current_gen + 1) % 2;

        display_current_gen(&grid, current_gen, square_side_size);
        calculate_next_gen(&mut grid, current_gen, next_gen, &neighbors);

        current_gen = next_gen;
        sleep(Duration::from_millis(250));
    };
}