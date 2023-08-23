use reqwest::header::COOKIE;
use std::{env, error::Error, fs, fs::File, io, str::FromStr};

pub trait Problem {
    type Input: FromStr<Err = String>;
    const DAY: u8;
    fn solve(input: Self::Input) -> Result<u32, Box<dyn Error>>;
}

pub fn solve<P: Problem>() -> Result<u32, Box<dyn Error>> {
    P::solve(download(P::DAY)?.parse::<P::Input>()?)
}
fn download(day: u8) -> Result<String, Box<dyn Error>> {
    let mut path = env::temp_dir();
    path.set_file_name(format!("code_advent_day_{:?}.txt", day));
    let path = path.as_path();

    if let Err(_) = fs::metadata(path) {
        let mut http_response = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?
        .get(format!("https://adventofcode.com/2022/day/{:?}/input", day))
        .header(COOKIE, "_ga=GA1.2.19743620.1692601480; _gid=GA1.2.1285583899.1692601480; _ga_MHSNPJKWC7=GS1.2.1692601480.1.1.1692601713.0.0.0; session=53616c7465645f5f006f509444acba3e035c1eeaa9454bae98c37ad96581fa02443b287ce297ad47457335dd716b287153637ae1250715adbac4f02fa963ce35")
        .send()?;
        if http_response.status().is_success() {
            let mut cache_file =
                File::create(path).expect(format!("Unable to create file {:?}", path).as_str());
            io::copy(&mut http_response, &mut cache_file).expect("Unable to copy data");
        } else {
            return Err(http_response.text()?.into());
        }
    }
    Ok(fs::read_to_string(path).expect(format!("Unable to read file {:?}", path).as_str()))
}
