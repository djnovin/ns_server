use std::env;

use actix_web::{web, HttpResponse, Error, post, Responder, get};
use awc::Client;
use chrono::{NaiveDate, Datelike};
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::helpers::gpt::generate_message;
use crate::validations::validations::{validate_state, validate_australian_mobile_number, validate_order_number};


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

// Struct that represents the JSON payload that is sent to the API
#[derive(Serialize, Deserialize, Validate)]
pub struct Input {
    #[validate(custom = "validate_order_number")]
     order_number: String,
    // + any other request fields that are required
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


#[post("/message")]
pub async fn send_message(input: web::Json<Input>) -> Result<HttpResponse, Error> {
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
        generate_message(prompt).await?
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::App;

    #[actix_web::test]
async fn test_send_message() {
    // Set the environment variable needed by your test via env::set_var
    std::env::set_var("ORDER_API_ENDPOINT", "http://example.com/order_api");

    // Create an Actix Web test server
    let srv = actix_web::test::init_service(
        App::new().service(send_message)
    ).await;

    // Define your Input
    let input = Input {
        order_number: "MY12345V".to_string(),
        // Add other fields as needed
    };

    // Make a request to the endpoint
    let request = actix_web::test::TestRequest::post()
        .uri("/message")
        .set_json(&input)
        .to_request();

    // Call the service and get the response
    let response = actix_web::test::call_service(&srv, request).await;

    // Check that the response status code is 200 OK
    assert_eq!(response.status(), actix_web::http::StatusCode::OK);

    // You can further assert the response body if needed
}
}

#[get("/health")]
pub async fn healthy() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy!")
}