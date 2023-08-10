use dotenv::dotenv;
use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Responder, get, middleware::Logger};
use awc::Client;
use chrono::{Datelike, NaiveDate};
use core::panic;
use serde::{Deserialize, Serialize};
use std::env;
use validator::{Validate, ValidationError};
use regex::Regex;
use log::{info, error};
use actix_cors::Cors;

mod helpers;

// Struct that represents the JSON payload that is sent to the API
#[derive(Serialize, Deserialize, Validate)]
struct Input {
    #[validate(custom = "validate_order_number")]
     order_number: String,
    // + any other request fields that are required
}

fn validate_australian_mobile_number(phone_number: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^\+?61\d{9}$").unwrap();
    if re.is_match(phone_number) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_australian_mobile_number");
        error.message = Some(format!("Invalid Australian mobile number {}", phone_number).into());
        Err(error)
    }
}

fn validate_order_number(order_number: &str) -> Result<(), ValidationError> {
    // TODO - Validate the order number using the validator crate by enumerating the rules
    let re = Regex::new(r"^(MH|MY|MC|MB|MR)\d{5}V?$").unwrap();
    if re.is_match(order_number) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_order_number");
        error.message = Some(format!("Invalid order number {}", order_number).into());
        Err(error)    
    }
}

fn validate_state(state: &str) -> Result<(), ValidationError> {
    match state {
        "NSW" | "VIC" | "QLD" | "SA" | "WA" | "TAS" | "NT" | "ACT" => Ok(()),
        _ => {
            let mut error = ValidationError::new("invalid_state");
            error.message = Some(format!("Invalid state: {}", state).into());
            Err(error)
        }
    }
}

// Struct that represents the JSON payload that is returned from the API
#[derive(Serialize, Deserialize, Validate)]
struct ResponsePayload {
    #[validate(length(min = 1))]
    customer_id: String,
    customer_first_name: String,
    customer_last_name: String,
    #[validate(email)]
    customer_email: Option<String>,
    #[validate(custom = "validate_australian_mobile_number")]
    customer_primary_phone: Option<String>,
    #[validate(custom = "validate_australian_mobile_number")]
    customer_secondary_phone: Option<String>,
    #[validate(length(min = 1))]
    delivery_date: String,
    #[validate(length(min = 1))]
    order_number: String,
    customer_delivery_address: String,
    #[validate(length(min = 4, max = 4))]
    customer_delivery_postcode: String,
    #[validate(length(min = 1))]
    customer_delivery_suburb: String,
    #[validate(custom = "validate_state")]
    customer_delivery_state: String,
    
    // + any other response fields that are required
}

enum Week {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl Week {
    fn as_str(&self) -> &'static str {
        match *self {
            Week::First => "First",
            Week::Second => "Second",
            Week::Third => "Third",
            Week::Fourth => "Fourth",
            Week::Fifth => "Fifth",
        }
    }
}

impl From<u32> for Week {
    fn from(num: u32) -> Self {
        match num {
            1 => Week::First,
            2 => Week::Second,
            3 => Week::Third,
            4 => Week::Fourth,
            5 => Week::Fifth,
            _ => panic!("Invalid week number"), // or handle this error appropriately
        }
    }
}

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn as_str(&self) -> &'static str {
        match *self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }
}

impl From<u32> for Month {
    fn from(num: u32) -> Self {
        match num {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Invalid month number"), // or handle this error appropriately
        }
    }
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}

#[derive(Serialize)]
struct GPT3Request {
    prompt: String,
    max_tokens: u32,
    temperature: f64,
    top_p: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
    stop: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GPT3Response {
    choices: Vec<Choice>,
}

#[post("/message")]
async fn send_message(input: web::Json<Input>) -> Result<HttpResponse, Error> {
    // Create an Actix Web client and make the request
    let client = Client::default();

    // TODO - Validate the input order number
    // - Validate the input using the validator crate by enumerating the rules
    // - i.e. [MY] + [0-9]{5} + [V]?
    // - e.g. MY12345V

    let order_number: &String = &input.order_number;

    // Retrieve the API endpoint from the environment variable
    let order_endpoint: String = env::var("ORDER_API_ENDPOINT").expect("BROKER_API_ENDPOINT must be set");

    // Build the URL to make the request to
    let order_url = format!("{}?order_number={}", order_endpoint, order_number);

    let mut order_response = client
        .post(order_url)
        .send()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response_payload: ResponsePayload = order_response
        .json()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Parse the date string into a NaiveDate
    let date = NaiveDate::parse_from_str(&response_payload.delivery_date, "%d-%m-%y")
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid date format"))?;

        let week_number = date.iso_week().week();
        let month_number = date.month();

    let weekname: Week = week_number.into(); // Conversion from u32 to Week

    let month: Month = month_number.into(); // Conversion from u32 to Month

    let prompt = format!(
        "I need to send an SMS to a customer who has purchased furniture from Nick Scali. \
        The typical delivery takes around 10-12 weeks, and we give an update at the 6-8 week mark. \
        Their delivery is tentatively set for the {} week of {}. \
        The sms will be no longer than 2-3 sentences. \
        Can you create a concise yet friendly SMS that informs them of the tentative delivery week, \
        reassures them about the update, and thanks them for their purchase? \
        I want to make sure they know we'll notify them with the exact date as soon as possible.",
        weekname.as_str(), month.as_str()
    ); 

    let message = if env::var("GPT").unwrap_or_default() == "true" {
        generate_message(&prompt).await?
    } else {
        format!(
            "Dear Customer, we at Nick Scali Castle Hill are pleased to inform you that your delivery is tentatively booked for the {} week of {}. We will notify you with the exact date as soon as possible. Thank you for choosing our service!",
            weekname.as_str(), month.as_str() 
        )
    };

    // TODO
    // - Send message to Internal API Broker that proxies request to SMS/and or SMTP providers

    Ok(HttpResponse::Ok().body(message))
}

#[get("/health")]
async fn healthy() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy!")
    
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    env_logger::init();

    let environment = env::var("ENV").unwrap_or_else(|_| "local".to_string());
    let host_var = format!("HOST_{}", environment.to_uppercase());
    let port_var = format!("PORT_{}", environment.to_uppercase());

    // Retrieve the host from the environment variable
    let host = env::var(&host_var)
        .unwrap_or_else(|_| "127.0.0.1".to_string()); // Default to localhost

    // Retrieve the port from the environment variable
    let port: u16 = env::var(&port_var)
        .unwrap_or_else(|_| "8080".to_string()) // Default to port 8080
        .parse()
        .expect("PORT must be a valid number");

    info!("Starting server at http://{}:{}", host, port);

    // Start the Actix Web server
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(
            // prefixes all resources and routes attached to it...
                web::scope("/api")
                    // handle all routes that start with "/api"
                    .service(healthy)
                    .service(send_message),
        )
    })
    .bind((host, port)) // Bind to the host and port determined by the environment
    .map_err(|e| {
        error!("Failed to bind server: {}", e);
        e
    })?
    .run()
    .await

}