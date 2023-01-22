mod day1;
mod day2;

fn main() {
    // Day 1
    let start = std::time::Instant::now();
    day1::run_own();
    println!("Time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    day1::run_prime_agen();
    println!("Time: {:?}", start.elapsed());

    // Day 2
    let start = std::time::Instant::now();
    day2::run_own();
    println!("Time: {:?}", start.elapsed());
}
