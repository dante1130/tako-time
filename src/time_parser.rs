use chrono::Duration;

pub enum TimeParser {}

pub enum TimeParserError {
    InvalidNumberFormat,    
    InvalidTimeFormat,
}

impl TimeParser {
    pub fn parse(time_format_strings: &Vec<String>) -> Result<Duration, TimeParserError> {
        let mut duration = chrono::Duration::zero();

        for time_format in time_format_strings {
            let time_format_chars = time_format.chars();
            let mut number = String::new();
            let mut unit = String::new();

            for c in time_format_chars {
                if c.is_ascii_digit() {
                    number.push(c);
                } else {
                    unit.push(c);
                }
            }

            let number = match number.parse::<i64>() {
                Ok(number) => number,
                Err(_) => return Err(TimeParserError::InvalidNumberFormat),
            };

            match unit.as_str() {
                "m" => duration = duration + Duration::minutes(number),
                "h" => duration = duration + Duration::hours(number),
                "d" => duration = duration + Duration::days(number),
                "w" => duration = duration + Duration::weeks(number),
                _ => return Err(TimeParserError::InvalidTimeFormat),
            }
        }

        Ok(duration)
    }
}
