use chrono::{DateTime, Duration, Utc};



// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
	let giga_seconds = Duration::seconds(1_000_000_000);
    start + giga_seconds
}
