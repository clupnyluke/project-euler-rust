use std::io::Read;

fn main() {
  let mut file = std::fs::File::open("triangle.txt").unwrap();
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents);
    res.unwrap();
    let mut rows: Vec<&str> = contents.split("\n").collect();
    let mut num_rows: Vec<Vec<u16>>;
    let mut i = 0;
    for row in rows {
        let nums: Vec<&str> = row.split(" ").collect();
        nums.into_iter().map(|x| x.parse::u16());
        i += 1;
    }
}
