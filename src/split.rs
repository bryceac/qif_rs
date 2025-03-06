use std::fmt;

/// structure that represents a split in a transaction
#[derive(Clone, Debug)]
pub struct Split {
    pub category: Option<String>,
    pub memo: String,
    pub amount: f64
}

impl Split {
    pub fn from(category: Option<String>, memo: String, amount: f64) -> Self {
        Split { 
            category, 
            memo, 
            amount
        }
    }

    pub fn builder() -> SplitBuilder {
        SplitBuilder::new()
    }

    pub fn to_string(&self) -> String {
        format!("S{}\r\nE{}\r\n${:.2}", 
        self.category.clone().unwrap_or(String::default()), 
        self.memo, 
        self.amount)
    }
}

impl fmt::Display for Split {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Split {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category &&
        self.memo == other.memo &&
        self.amount == other.amount
    }
}

pub struct SplitBuilder {
    pub category: Option<String>,
    pub memo: Option<String>,
    pub amount: Option<f64>
}

impl SplitBuilder {
    pub fn new() -> Self {
        SplitBuilder { 
            category: None, 
            memo: None, 
            amount: None 
        }
    }

    pub fn set_category(&mut self, category: &str) -> &mut Self {
        self.category = if category.is_empty() {
            None
        } else {
            Some(category.to_string())
        };
        self
    }

    pub fn set_memo(&mut self, memo: &str) -> &mut Self {
        self.memo = if memo.is_empty() {
            None
        } else {
            Some(memo.to_string())
        };
        self
    }

    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    pub fn set_amount_via_percentage(&mut self, amount: f64, percentage: f64) -> &mut Self {
        let percentage_as_decimal = percentage/100.0;

        self.amount = Some(amount * percentage_as_decimal);
        self
    }

    pub fn build(&self) -> Option<Split> {
        if let Some(amount) = self.amount {
            Some(Split::from(
                self.category.clone(), 
                self.memo.clone().unwrap_or(String::default()), 
                amount))
        } else {
            None
        }
    }
}