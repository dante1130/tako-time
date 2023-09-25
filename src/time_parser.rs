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

    pub fn duration_to_string(duration: &Duration) -> String {
        let mut duration_string = String::new();

        const DAYS_IN_WEEK: i64 = 7;
        const HOURS_IN_DAY: i64 = 24;
        const MINUTES_IN_HOUR: i64 = 60;

        let weeks = duration.num_weeks();
        let days = duration.num_days() - weeks * DAYS_IN_WEEK;

        let hours =
            duration.num_hours() - weeks * DAYS_IN_WEEK * HOURS_IN_DAY - days * HOURS_IN_DAY;

        let minutes = duration.num_minutes()
            - weeks * DAYS_IN_WEEK * HOURS_IN_DAY * MINUTES_IN_HOUR
            - days * HOURS_IN_DAY * MINUTES_IN_HOUR
            - hours * MINUTES_IN_HOUR;

        if weeks > 0 {
            duration_string.push_str(&format!("{}w ", weeks));
        }

        if days > 0 {
            duration_string.push_str(&format!("{}d ", days));
        }

        if hours > 0 {
            duration_string.push_str(&format!("{}h ", hours));
        }

        if minutes > 0 {
            duration_string.push_str(&format!("{}m", minutes));
        }

        duration_string.trim().into()
    }
}
