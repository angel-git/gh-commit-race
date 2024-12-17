use chrono::{DateTime, Datelike};

pub fn convert_timestamp_to_month_and_year(timestamp: &u32) -> String {
    let naive_datetime = DateTime::from_timestamp(timestamp.clone() as i64, 0).unwrap();
    format!("{} {}", naive_datetime.format("%b"), naive_datetime.year())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_convert_week_timestamp_to_month() {
        let timestamp = 1361059200;
        let month = convert_timestamp_to_month_and_year(&timestamp);
        assert_eq!(month, "Feb 2013");
    }
}