mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

    // Day 3
    let start = std::time::Instant::now();
    day3::run_own();
    println!("Time: {:?}", start.elapsed());

    // Day 4
    let start = std::time::Instant::now();
    day4::run_own();
    println!("Time: {:?}", start.elapsed());

    // Day 5
    let start = std::time::Instant::now();
    day5::run_own();
    println!("Time: {:?}", start.elapsed());
}
