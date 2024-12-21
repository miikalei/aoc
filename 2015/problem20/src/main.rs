fn main() {
    // For a given integer n, the f(n) is 10 times the sum of its divisors (1 and n included)
    let mut n = 1;
    while f(n) < 29000000 {
        println!("{}: {}", n, f(n));
        n += 1;
    }
    println!("{}: {}", n, f(n));
}

fn f(n: u32) -> u32 {
    let mut s = 0;
    let limit = (n as f64).sqrt().floor() as u32;
    for i in 1..=limit {
        if n % i == 0 {
            if n / i <= 50 {
                s += i;
            }
            if i * i != n {
                let i_alt = n / i;
                if n / i_alt <= 50 {
                    s += i_alt;
                }
            }
        }
    }
    s * 11
}
