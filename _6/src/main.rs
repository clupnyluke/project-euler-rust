fn main() {
    let mut sum = 0;
    let mut square_sum = 0;
    for i in 1..101 {
        sum += i;
        square_sum += i * i;
    }
    println!("Difference: {}", sum * sum - square_sum)
}
