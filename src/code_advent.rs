use std::{error::Error, str::FromStr};
use reqwest::header::COOKIE;

pub trait Solution {
    type Input : FromStr<Err = String>;
    const DAY : u8;
    fn solve(input: Self::Input) -> Result<u32, Box<dyn Error>>;
}

pub fn solve<P:Solution>() -> Result<u32, Box<dyn Error>> {
    let resp = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?
        .get(format!("https://adventofcode.com/2022/day/{:?}/input", P::DAY))
        .header(COOKIE, "_ga=GA1.2.19743620.1692601480; _gid=GA1.2.1285583899.1692601480; _ga_MHSNPJKWC7=GS1.2.1692601480.1.1.1692601713.0.0.0; session=53616c7465645f5f006f509444acba3e035c1eeaa9454bae98c37ad96581fa02443b287ce297ad47457335dd716b287153637ae1250715adbac4f02fa963ce35")
        .send()?;
    if resp.status().is_success() {
        P::solve(resp.text()?.parse::<P::Input>()?)
    } else {
        Err(resp.text()?.into())
    }    
}
