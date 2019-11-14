use std::fmt;

// compute x mod n since rust can't do negative modulo like other languages
fn modulo(x: i32, n: i32) -> i32 {
    (x % n + n) % n
}

#[derive(Debug, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = modulo(self.minutes, 60);
        let hours = modulo(
            (f64::from(self.minutes) / 60.0).floor() as i32 + self.hours,
            24,
        );
        write!(
            f,
            "{}{}{}",
            format!("{:0>2}", hours),
            ":",
            format!("{:0>2}", minutes)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
