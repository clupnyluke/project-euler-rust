fn main() {
    let mut i = 0;
    let mut sum = 0;
    loop {
        if i >= 1000 {
            break;
        }
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i+=1;
    }
    println!("{}", sum)

    
}
