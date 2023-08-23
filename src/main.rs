use reqwest::header::COOKIE;
use std::error::Error;
mod problem2;
fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?
        .get("https://adventofcode.com/2022/day/2/input")
        .header(COOKIE, "_ga=GA1.2.19743620.1692601480; _gid=GA1.2.1285583899.1692601480; _ga_MHSNPJKWC7=GS1.2.1692601480.1.1.1692601713.0.0.0; session=53616c7465645f5f006f509444acba3e035c1eeaa9454bae98c37ad96581fa02443b287ce297ad47457335dd716b287153637ae1250715adbac4f02fa963ce35")
        .send()?;

    let result = if resp.status().is_success() {
        resp.text()?.parse::<problem2::Input>()?.solve()
    } else {
        let err = resp.text()?;
        Err(err.into())
    };
    print!("{:?}", result);
    Ok(())
}
