/**
 * enumeration for specifying qif type.
 * Only noninvoice and noninvestment times are currently present.
 */
#[derive(Clone, Debug, PartialEq)]
pub enum QIFType {
    Cash,
    Bank,
    CreditCard,
    Liability,
    Asset
}

impl QIFType {
    pub fn from(s: &str) -> Option<Self> {
        match s.to_string() {
            content if content == "Cash" => Some(Self::Cash),
            content if content == "Bank" => Some(Self::Bank),
            content if content == "CCard" => Some(Self::CreditCard),
            content if content == "Oth L" => Some(Self::Liability),
            content if content == "Oth A" => Some(Self::Asset),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Cash => "Cash",
            Self::Bank => "Bank",
            Self::CreditCard => "CCard",
            Self::Liability => "Oth L",
            Self::Asset => "Oth A"
        }
    }
}