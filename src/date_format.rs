/// a conveniece enumeration to deal with chrono based input and output.
pub enum DateFormat {
    MonthDayFullYear,
    MonthDayShortYear,
    FullYearMonthDay
}

impl DateFormat {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "mm/dd/yyyy" | "%m/%d/%Y" => Some(Self::MonthDayFullYear),
            "mm/dd/yy" | "%m/%d/%y" => Some(Self::MonthDayShortYear),
            "yyyy-mm-dd" | "%Y-%m-%d" => Some(Self::FullYearMonthDay),
            _ => None
        }
    }

    pub fn human_str(&self) -> &str {
        match self {
            Self::MonthDayFullYear => "mm/dd/yyyy",
            Self::MonthDayShortYear => "mm/dd/yy",
            Self::FullYearMonthDay => "yyyy-mm-dd"
        }
    }

    pub fn chrono_str(&self) -> &str {
        match self {
            Self::MonthDayFullYear => "%m/%d/%Y",
            Self::MonthDayShortYear => "%m/%d/%y",
            Self::FullYearMonthDay => "%Y-%m-%d"
        }
    }
}