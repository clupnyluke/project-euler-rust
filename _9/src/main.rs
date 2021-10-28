fn main() {
    for a in 1..1000 {
        let b_c = 1000 - a;
        let mut stop = false;
        for b in 1..b_c {
            let c = b_c - b;
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                stop = true;
                break;
            }
        }
        if stop {
            break;
        }
    }
}
