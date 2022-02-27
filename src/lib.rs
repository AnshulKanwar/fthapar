use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use reqwest::redirect::Policy;
use std::error::Error;

pub struct Config {
    enrollment_number: u32,
    password: String,
}

enum Password {
    Correct,
    Incorrect,
    Unknown,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let enrollment_number = args[1]
            .clone()
            .parse()
            .expect("Enter a valid Enrollment Number");
        let password = args[2].clone();

        Ok(Config {
            enrollment_number,
            password,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let result = check_password(config.enrollment_number, config.password);

    match result {
        Password::Correct => println!("Correct Password"),
        Password::Incorrect => println!("Incorrect Password"),
        Password::Unknown => println!("Password Could not be verified"),
    }

    Ok(())
}

fn check_password(enrollment_number: u32, password: String) -> Password {
    let res = make_request(enrollment_number, password);

    match res {
        Ok(res) => {
            match res.status().as_u16() {
                302 => Password::Correct,
                200 => Password::Incorrect,
                other => {
                    println!("Error: Received Status Code {}", other);
                    Password::Unknown
                }
            }
        },
        Err(_) => Password::Unknown,
    }
}

fn make_request(enrollment_number: u32, password: String) -> Result<Response, &'static str> {
    let client_builder = Client::builder().redirect(Policy::none());
    let client = client_builder.build().expect("Failed Building Client");

    let body = format!("txtuType=Member+Type&UserType=S&txtCode=Enrollment+No&MemberCode={}&txtPin=Password%2FPin&Password={}&BTNSubmit=Submit", enrollment_number, password);

    let mut res;
    let mut try_count = 5;

    while try_count > 0 {
        res = client
            .post("https://webkiosk.thapar.edu/CommonFiles/UserAction.jsp")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body.clone())
            .send();
        
        match res {
            Ok(res) => {
                println!("{:?}", res.status());
                return Ok(res)
            }
            Err(_) => try_count -= 1,
        }
    };

    Err("Couldn't Verify password")
}