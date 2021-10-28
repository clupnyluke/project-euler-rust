fn main() {
    let mut terms: Vec<u128> = Vec::new();
    terms.push(1);
    terms.push(2);
    let mut sum = 2;
    loop {
        let next: u128 = terms.iter().sum();
        if next >= 4000000 {
            break;
        }
        if next % 2 == 0 {
            sum += next
        }
        terms.push(next);
        terms.remove(0);
    }
    print!("{}", sum)
}
