mod code_advent;
mod day2;
mod day3;
fn main() {
    // https://adventofcode.com/2022
    let start = std::time::Instant::now();
    let answer = code_advent::solve::<day3::Subject>();
    print!("{:?} in {}", answer, 
        humantime::format_duration(start.elapsed()));
}

