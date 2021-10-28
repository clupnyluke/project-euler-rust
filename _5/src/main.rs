fn main() {
    let mut i = 1;
    loop {
        let mut divisible = true;
        for x in 2..20 {
            divisible = divisible && i % x == 0;
            if !divisible {
                break;
            }
        }
        if divisible {
            break;
        }
        i += 1;
    }
    println!("{}", i);
}
