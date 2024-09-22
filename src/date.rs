pub struct Date {
    year: Option<isize>,
    month: Option<u8>,
    day: Option<u8>,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(y) = self.year {
            if let Some(m) = self.month {
                let m = match m {
                    1 => "January",
                    2 => "February",
                    3 => "March",
                    4 => "April",
                    5 => "May",
                    6 => "June",
                    7 => "July",
                    8 => "August",
                    9 => "September",
                    10 => "October",
                    11 => "November",
                    12 => "December",
                    _ => "Invalid",
                };
                if let Some(d) = self.day {
                    write!(f, "{d} {m} {y}")
                } else {
                    write!(f, "{m} {y}")
                }
            } else {
                write!(f, "{y}")
            }
        } else {
            write!(f, "Unknown")
        }
    }
}
