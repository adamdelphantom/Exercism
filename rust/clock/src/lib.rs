use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut actual_hours = hours;
        let mut actual_minutes = minutes;

        if actual_minutes > 60 {
            actual_hours += actual_minutes / 60;
            actual_minutes %= 60;
        } else if actual_minutes == 60 {
            actual_hours += 1;
            actual_minutes = 0;
        } else if actual_minutes == -60 {
            actual_hours -= 1;
            actual_minutes = 0;
        } else if actual_minutes < 0 {
            actual_hours += (actual_minutes / 60) + 23;
            actual_minutes = (actual_minutes % 60) + 60;
        }

        if actual_hours > 24 {
            actual_hours %= 24;
        } else if actual_hours == 24 {
            actual_hours = 0;
        } else if actual_hours < 0 {
            actual_hours = (actual_hours % 24) + 24;
        }

        Self {
            hours: actual_hours,
            minutes: actual_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = self.minutes + minutes;
        let mut new_hours = 0;

        if total_minutes >= 60 {
            new_hours = total_minutes / 60;
            total_minutes %= 60;
        } else if total_minutes < 0 {
            new_hours += (total_minutes / 60) + 23;
            total_minutes = (total_minutes % 60) + 60;
        }

        new_hours += self.hours;

        if new_hours >= 24 || new_hours == -24 {
            new_hours %= 24;
        }

        Self {
            hours: new_hours,
            minutes: total_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
