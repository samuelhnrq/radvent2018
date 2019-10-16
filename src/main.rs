mod dayone;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;

use env_logger::Env;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::fmt::Debug;
use std::io::{self, Write};
use std::iter::repeat_with;
use std::result::Result;
use std::str::FromStr;

const DIAS_IMPLEMENTADOS: usize = 1;
const DIAS: [fn(String) -> (); DIAS_IMPLEMENTADOS] = [dayone::day_one];

fn read_stdin<T>(prompt: &'static str, err_msg: &'static str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    loop {
        print!("{}", prompt);
        // Reads from stdout with unwrap, because seriously if I can't trust stdin...
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let val = buffer.trim().parse::<T>();
        if val.is_err() {
            info!("{}", err_msg);
            buffer.clear();
        } else {
            return val.unwrap();
        }
    }
}

fn fetch_input(input_day: usize) -> Result<String, reqwest::Error> {
    let day = input_day + 1;
    let cli: reqwest::Client = Client::builder()
        .redirect(reqwest::RedirectPolicy::none())
        .build()?;
    let auth_cookie_value = format!(
        "session={}",
        std::env::var("ADVENT_TOKEN").unwrap_or(String::new())
    );
    if auth_cookie_value.trim() == "" {
        let m = concat!(
            "Auth cookie not found!\n",
            "Extract it from the network tab of the inspector of your browser."
        );
        panic!(m);
    }
    debug!("Found token {}", auth_cookie_value);
    let mut cookie_headers = HeaderMap::new();
    cookie_headers.insert(
        "cookie",
        HeaderValue::from_str(auth_cookie_value.as_str()).unwrap(),
    );
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    info!("Fetching input of day {}...", day);
    let txt: String = cli
        .get(url.as_str())
        .headers(cookie_headers)
        .send()?
        .text()?;
    info!("Input fetched successfully");
    Ok(txt.trim().to_string())
}

fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    info!("Which day do you want to run?");
    for i in 1..DIAS.len() + 1 {
        info!("{}# dia de natal", i);
    }
    let numb = repeat_with(|| read_stdin::<usize>("> ", "Invalid number"))
        .find(|&x| x > 0 && x <= DIAS.len())
        .unwrap_or(1)
        - 1;
    info!("Você selecionou o número: {}", numb + 1);
    info!("=======================");
    let input = fetch_input(numb).expect("Error during download of the puzzle input!");
    DIAS[numb](input);
    info!("Done!");
    read_stdin::<String>("Press any key to quit...", "");
    Ok(())
}
