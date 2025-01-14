// Target row: 2978
// Target column: 3083

fn main() {
    println!("{}", get_value_at(3083, 2978));
}

fn get_value_at(col: i32, row: i32) -> u64 {
    let target_index = get_cell_index(col, row);
    let target_value = (1..target_index).fold(20151125, |acc, _next| get_next(acc));
    target_value
}

fn get_cell_index(col: i32, row: i32) -> i32 {
    let triangle_size = col + row - 2;
    let triangle_count = triangle_size * (triangle_size + 1) / 2;
    triangle_count + col
}

fn get_next(input: u64) -> u64 {
    (input * 252533) % 33554393
}
