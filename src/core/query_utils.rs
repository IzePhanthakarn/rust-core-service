use chrono::{DateTime, TimeZone, Utc};

/// Trims a filter value and treats an empty string the same as `None`.
pub fn normalize_filter(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}

/// Resolves a `month`/`year` filter pair into a `[start, end)` UTC range covering that month.
/// Returns `None` if either part is missing or does not parse into a valid month/year.
pub fn month_year_range(month: Option<&str>, year: Option<&str>) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let month = month?.parse::<u32>().ok()?;
    let year = year?.parse::<i32>().ok()?;

    if !(1..=12).contains(&month) {
        return None;
    }

    let (next_year, next_month) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };

    let start_date = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single()?;
    let end_date = Utc.with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0).single()?;

    Some((start_date, end_date))
}
