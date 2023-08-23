mod code_advent;
mod day2;
fn main() {
    let start = std::time::Instant::now();
    let answer = code_advent::solve::<day2::Subject>();
    print!("{:?} in {}", answer, 
        humantime::format_duration(start.elapsed()));
}

