use regex::Regex;
use validator::ValidationError;

pub fn validate_state(state: &str) -> Result<(), ValidationError> {
    match state {
        "NSW" | "VIC" | "QLD" | "SA" | "WA" | "TAS" | "NT" | "ACT" => Ok(()),
        _ => {
            let mut error = ValidationError::new("invalid_state");
            error.message = Some(format!("Invalid state: {}", state).into());
            Err(error)
        }
    }
}

pub fn validate_australian_mobile_number(phone_number: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^\+?61\d{9}$").unwrap();
    if re.is_match(phone_number) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_australian_mobile_number");
        error.message = Some(format!("Invalid Australian mobile number {}", phone_number).into());
        Err(error)
    }
}

pub fn validate_order_number(order_number: &str) -> Result<(), ValidationError> {
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
