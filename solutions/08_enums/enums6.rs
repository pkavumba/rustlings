// Match guards let you add conditions to patterns. Order matters: the first
// matching arm wins.

#[derive(Debug, PartialEq)]
enum HttpStatus {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
    Unknown,
}

fn classify_status(code: u16) -> HttpStatus {
    match code {
        100..=199 => HttpStatus::Informational,
        200..=299 => HttpStatus::Success,
        300..=399 => HttpStatus::Redirection,
        400..=499 => HttpStatus::ClientError,
        500..=599 => HttpStatus::ServerError,
        _ => HttpStatus::Unknown,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_codes_with_range_patterns() {
        assert_eq!(classify_status(100), HttpStatus::Informational);
        assert_eq!(classify_status(204), HttpStatus::Success);
        assert_eq!(classify_status(301), HttpStatus::Redirection);
        assert_eq!(classify_status(404), HttpStatus::ClientError);
        assert_eq!(classify_status(503), HttpStatus::ServerError);
        assert_eq!(classify_status(42), HttpStatus::Unknown);
    }
}
