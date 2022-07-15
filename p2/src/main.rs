fn main() {
    let mut n = 1;
    let mut p = 1;
    let mut sum = 0;

    while n < 4_000_000 {
        n += p;
        p = n - p;

        if n % 2 == 0 {
            sum += n;
            println!("{}", sum)
        }
    }
}
