use chrono::prelude::*;

use crate::{TransactionStatus, Split, DateFormat, errors::{ TransactionBuildingError}, split::SplitBuilder};

use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

/// structure that represents a regular transaction in a QIF file.
#[derive(Clone, Debug)]
pub struct Transaction {
    pub date: DateTime<Local>,
    pub check_number: Option<u32>,
    pub vendor: String,
    pub address: String,
    pub amount: f64,
    pub category: Option<String>,
    pub memo: String,
    pub status: Option<TransactionStatus>,
    pub splits: Vec<Split>
}

impl Transaction {
    /**
     * create a Tranaction object.
     * The field are self explanatory, in what is expected.
     * 
     * However, this function is mainly used as a convenice initializer, 
     * though it can be used directly.
    */
    pub fn from(date: DateTime<Local>, check_number: Option<u32>, vendor: String, address: String, amount: f64, category: Option<String>, memo: String, status: Option<TransactionStatus>, splits: Vec<Split>) -> Self {
        Transaction { 
            date, 
            check_number, 
            vendor, 
            address, 
            amount, 
            category, 
            memo, 
            status, 
            splits 
        }
    }

    /**
     * This method creates a builder that will help faciliate
     * in a creating transaction.
     * 
     * Transactions can then be made like this:
     * 
     * let sam_hill = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build().unwrap();
     */
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder::new()
    }

    pub fn to_string(&self) -> String {
        if self.splits.is_empty() {
            format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^", 
                self.date.format("%m/%d/%Y"),
                self.amount,
                if let Some(status) = &self.status {
                    status.to_str()
                } else {
                    ""
                },
                if let Some(check_number) = self.check_number {
                    check_number.to_string()
                } else {
                    String::default()
                },
                self.vendor,
                self.memo,
                self.address,
                if let Some(category) = &self.category {
                    category.to_owned()
                } else {
                    String::default()
                }
            )
        } else {
            let mut initial_string = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n", 
                self.date.format("%m/%d/%Y"),
                self.amount,
                if let Some(status) = &self.status {
                    status.to_str()
                } else {
                    ""
                },
                if let Some(check_number) = self.check_number {
                    check_number.to_string()
                } else {
                    String::default()
                },
                self.vendor,
                self.memo,
                self.address,
                if let Some(category) = &self.category {
                    category.to_owned()
                } else {
                    String::default()
                }
            );

            for split in self.splits.clone() {
                if let Some(most_recent_split) = self.splits.last() {
                    if split == most_recent_split.clone() {
                        let last_entry = format!("{}\r\n^", split);
                        initial_string.push_str(&last_entry);
                    } else {
                        let split_entry = format!("{}\r\n", split);
                        initial_string.push_str(&split_entry);
                    }
                }
            }

            initial_string
        }
    }

    pub fn from_str(s: &str) -> Result<Self, TransactionBuildingError> {
        let lines = s.lines();
        let mut builder = Transaction::builder();
        let mut split_builders: Vec<SplitBuilder> = vec![];

        for line in lines {
            match line {
                content if content.starts_with("D") => {
                    builder.set_date(&drop_first_character_from(content), &DateFormat::MonthDayFullYear);
                },
                content if content.starts_with("T") || content.starts_with("U") => if let Ok(amount) = drop_first_character_from(content).parse::<f64>() {
                    builder.set_amount(amount);
                },
                content if content.starts_with("N") => if let Ok(check_number) = drop_first_character_from(content).parse::<u32>() {
                    builder.set_check_number(check_number);
                },
                content if content.starts_with("P") => {
                    builder.set_vendor(&drop_first_character_from(content));
                },
                content if content.starts_with("A") => {
                    builder.set_address(&drop_first_character_from(content));
                },
                content if content.starts_with("L") => {
                    builder.set_category(&drop_first_character_from(content));
                },
                content if content.starts_with("M") => {
                    builder.set_memo(&drop_first_character_from(content));
                },
                content if content.starts_with("C") => {
                    builder.set_status(&drop_first_character_from(content));
                },
                content if content.starts_with("S") => {
                    let mut split_builder = Split::builder();

                    split_builder.set_category(&drop_first_character_from(content));

                    split_builders.push(split_builder);
                },
                content if content.starts_with("E") => {
                    if split_builders.is_empty() {
                        let mut split_builder = Split::builder();

                        split_builder.set_memo(&drop_first_character_from(content));

                        split_builders.push(split_builder);
                    } else {
                        if let Some(current_split_builder) = split_builders.last_mut() {
                            current_split_builder.set_memo(&drop_first_character_from(content));
                        }
                    }
                },
                content if content.starts_with("$") => {
                    if split_builders.is_empty() {
                        let mut split_builder = Split::builder();

                        if let Ok(amount) = drop_first_character_from(content).parse::<f64>() {
                            split_builder.set_amount(amount);
                        }

                        split_builders.push(split_builder);
                    } else {
                        if let Some(current_split_builder) = split_builders.last_mut() {
                            if let Ok(amount) = drop_first_character_from(content).parse::<f64>() {
                                current_split_builder.set_amount(amount);
                            }
                        }
                    }
                },
                content if content.starts_with("%") => {
                    if split_builders.is_empty() {
                        let mut split_builder = Split::builder();

                        if let Ok(percentage) = drop_first_character_from(content).parse::<f64>() {
                            if let Some(amount) = builder.amount {
                                split_builder.set_amount_via_percentage(amount, percentage);
                            }
                        }

                        split_builders.push(split_builder);
                    } else {
                        if let Some(current_split_builder) = split_builders.last_mut() {
                            if let Ok(percentage) = drop_first_character_from(content).parse::<f64>() {
                                if let Some(amount) = builder.amount {
                                    current_split_builder.set_amount_via_percentage(amount, percentage);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        for split_builder in split_builders {
            if let Some(split) = split_builder.build() {
                builder.add_split(split);
            }
        }

        builder.build()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date &&
        self.check_number == other.check_number &&
        self.vendor == other.vendor &&
        self.address == other.address &&
        self.amount == other.amount &&
        self.category == other.category &&
        self.memo == other.memo &&
        self.status == other.status &&
        self.splits == other.splits
    }
}

fn parse_date(s: &str, format: &DateFormat) -> Option<DateTime<Local>> {
    if let Ok(date_input) = NaiveDate::parse_from_str(s, format.chrono_str()) {
        if let Some(datetime) = date_input.and_hms_opt(0, 0, 0) {
            Some(Local.from_local_datetime(&datetime).unwrap())
        } else {
            None
        }
   } else {
        None
   }
}

fn drop_first_character_from(s: &str) -> String {
    let characters: Vec<String> = s.graphemes(true).map(|s| s.to_owned()).collect();

    let content: String = characters[1..].iter().map(|s| s.to_owned()).collect();

    content
}

pub struct TransactionBuilder {
    pub date: Option<DateTime<Local>>,
    pub check_number: Option<u32>,
    pub vendor: Option<String>,
    pub address: Option<String>,
    pub amount: Option<f64>,
    pub category: Option<String>,
    pub memo: Option<String>,
    pub status: Option<TransactionStatus>,
    pub splits: Vec<Split>
}

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder { 
            date: None, 
            check_number: None, 
            vendor: None, 
            address: None, 
            amount: None, 
            category: None, 
            memo: None, 
            status: None, 
            splits: vec![] 
        }
    }

    // builder functions to set various fields.
    pub fn set_date(&mut self, date: &str, format: &DateFormat) -> &mut Self {
        self.date = parse_date(date, &format);
        self
    }

    pub fn set_check_number(&mut self, check_number: u32) -> &mut Self {
        self.check_number = if check_number > 0 {
            Some(check_number)
        } else {
            None
        };
        self
    }

    pub fn set_vendor(&mut self, vendor: &str) -> &mut Self {
        self.vendor = if vendor.is_empty() {
            None
        } else {
            Some(vendor.to_string())
        };
        self
    }

    pub fn set_address(&mut self, address: &str) -> &mut Self {
        self.address = if address.is_empty() {
            self.vendor.clone()
        } else {
            Some(String::from(address))
        };

        self
    }

    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    pub fn set_category(&mut self, category: &str) -> &mut Self {
        self.category = if category.is_empty() {
            None
        } else {
            Some(String::from(category))
        };

        self
    }

    pub fn set_memo(&mut self, memo: &str) -> &mut Self {
        self.memo = if memo.is_empty() {
            None
        } else {
            Some(String::from(memo))
        };

        self
    }

    pub fn set_status(&mut self, status: &str) -> &mut Self {
        self.status = TransactionStatus::from(status);
        self
    }

    pub fn add_split(&mut self, split: Split) -> &mut Self  {
        self.splits.push(split);
        self
    }

    /**
     * this function is used to actually create the transaction.
     * It will return an error if there is no date, vendor, or amount
     * provided, which are all set with the respective setter methods.
     * 
     * Usage will look like this:
     * 
     * let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .add_split(initial_split)
        .add_split(bonus_split)
        .build().unwrap();

     * The address setter is optional because vendor 
     * can also be listed the address.
     */
    pub fn build(&self) -> Result<Transaction, TransactionBuildingError> {
        if let Some(date) = self.date {
            if let Some(vendor) = self.vendor.clone() {
                if let Some(amount) = self.amount {
                    Ok(Transaction::from(
                        date, 
                        self.check_number, 
                        vendor.clone(), 
                        self.address.clone().unwrap_or(vendor.clone()), 
                        amount, 
                        self.category.clone(), 
                        self.memo.clone().unwrap_or(String::default()), 
                        self.status.clone(), 
                        self.splits.clone()))
                } else {
                    Err(TransactionBuildingError::NoAmount)
                }
            } else {
                Err(TransactionBuildingError::NoVendor)
            }
        } else {
            Err(TransactionBuildingError::NoDate)
        }
    }
}