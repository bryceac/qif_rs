use std::fmt;

/// errors for creating transactions
#[derive(Debug, PartialEq)]
pub enum TransactionBuildingError {
    NoDate,
    NoVendor,
    NoAmount
}

impl TransactionBuildingError {
    pub fn to_string(&self) -> String {
        match self {
            Self::NoDate => "Date could not be found or parsed.".to_string(),
            Self::NoVendor => "Vendor not found.".to_string(),
            Self::NoAmount => "No Amount value found.".to_string(),
        }
    }
}

impl fmt::Display for TransactionBuildingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}