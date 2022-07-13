fn main() {
    let mut sum = 0;

    for n in 0..1000 {
        match (n % 5 == 0) || (n % 3 == 0) {
            true => sum += n,
            false => {}
        };
    }

    print!("{}", sum)
}
