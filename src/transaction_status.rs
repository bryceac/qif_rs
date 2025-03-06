/// enumeration that houses the possible statuses of a transaction.
#[derive(Clone, Debug, PartialEq)]
pub enum TransactionStatus {
    Cleared,
    Reconciled
}

impl TransactionStatus {
    pub fn from(s: &str) -> Option<Self> {
        match s.to_string() {
            value if value == "X" => Some(Self::Cleared),
            value if value == "*" => Some(Self::Reconciled),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Cleared => "X",
            Self::Reconciled => "*"
        }
    }
}