use std::fmt::{self, Display};

pub struct Instant {
    hours: u8,
    minutes: u8,
    seconds: u8
}

impl Instant {
    pub fn new(hours: u8, minutes: u8, seconds: u8) -> Self {
        assert!(hours < 24);
        assert!(minutes < 60);
        assert!(seconds < 60);

        Instant {
            hours,
            minutes,
            seconds
        }
    }
}

fn divide_sub(src: &mut u8, rhs: u8) -> u8 {
    let value = *src / rhs;
    *src = *src % rhs;
    value
}

pub struct Clock {
    even_seconds: bool,
    five_hours: u8,
    one_hours: u8,
    five_minutes: u8,
    one_minutes: u8
}

impl From<Instant> for Clock {
    fn from(mut instant: Instant) -> Self {
        Clock {
            even_seconds: instant.seconds % 2 == 0,
            five_hours: divide_sub(&mut instant.hours, 5),
            one_hours: instant.hours,
            five_minutes: divide_sub(&mut instant.minutes, 5),
            one_minutes: instant.minutes
        }
    }
}

fn colorize(condition: bool, color: char) -> char {
    if condition { color } else { '0' }
}

fn range(max: u8, length: u8, color: char) -> String {
    (0..length).map(|idx| colorize(idx < max, color)).collect()
}

impl Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}\n{}\n{}\n{}\n{}",
            colorize(self.even_seconds, 'R'),
            range(self.five_hours, 4, 'R'),
            range(self.one_hours, 4, 'R'),
            (0..11)
                .map(|idx| colorize(idx < self.five_minutes, if idx % 3 == 2 { 'R' } else { 'Y' }))
                .collect::<String>(),
            range(self.one_minutes, 4, 'Y')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_format() {
        assert_eq!(format!("{}", Clock::from(Instant::new(11, 15, 20))), "R\nRR00\nR000\nYYR00000000\n0000");
        assert_eq!(format!("{}", Clock::from(Instant::new(0, 0, 1))), "0\n0000\n0000\n00000000000\n0000");
        assert_eq!(format!("{}", Clock::from(Instant::new(23, 59, 0))), "R\nRRRR\nRRR0\nYYRYYRYYRYY\nYYYY");
    }
}