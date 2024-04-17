fn possible_paths(spot_x: u128, spot_y: u128, rows: u128, columns: u128) -> u128 {
    let mut sum = 1;
    if spot_x + 1 != columns {
        sum += possible_paths(spot_x + 1, spot_y, rows, columns);
    }

    if spot_y + 1 != rows {
        sum += possible_paths(spot_x, spot_y + 1, rows, columns);
    }

    sum
}

fn lattice_traversals(row: u128, column: u128) -> u128 {
    1 + possible_paths(0, 0, row, column)
}

fn main() {
    println!("{}", lattice_traversals(20, 20));
}
