//! Implements the [Todoist Sync API full-day dates].
//!
//! ## Example:
//! ```should_panic
//! use todoist_core::types::dates::DueDate;
//!
//! // Make a due date for tomorrow
//! let mut builder = DueDate::builder();
//!
//! // Make the a due date recuring
//! builder.is_recurring(true);
//!
//! // Set the date as tomorrow
//! builder.from_string("Tomorrow");
//!
//! // Build the due date
//! let due_date = builder.build();
//! ```
//!
//! [Todoist Sync API full-day dates]: https://developer.todoist.com/sync/v8/#due-dates
use serde::{Deserialize, Serialize};
use tracing;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DueDate {
    /// Due date in the format of YYYY-MM-DD. For recuring dates, the date of the current
    /// iteration. Default: `No date`.
    date: String,
    /// Always set to `null`
    timezone: String,
    /// Human-readable representation of due date. Default: `No date`.
    string: String,
    /// Language which is used to parse the content of the `string` attribute. Default: `en`.
    lang: String,
    /// Set to `true` if the due date is recurring. Default: `false`.
    is_recurring: bool,

    /// Set to `true` if the `DueDate` has no due date. This field must be `true` if `date`,
    /// `string`, and `is_recurring` are all default values.
    no_date: bool,
}

impl DueDate {
    /// [DueDate]s should only be built using the [DueDateBuilder]
    pub fn builder() -> DueDateBuilder {
        DueDateBuilder::default()
    }
}

impl Default for DueDate {
    fn default() -> Self {
        Self::builder().no_date(true).build().unwrap()
    }
}

/// Checker for supported languages as listed
/// [here](https://developer.todoist.com/sync/v8/#due-dates).
struct SupportedLang;
impl SupportedLang {
    fn supported_lang() -> Vec<&'static str> {
        vec![
            "en", "da", "pl", "zh", "ko", "de", "pt", "ja", "it", "fr", "sv", "ru", "es", "nl",
        ]
    }

    pub fn check_lang(value: &str) -> Result<(), ()> {
        if !Self::supported_lang().contains(&value) {
            return Err(());
        }
        Ok(())
    }

    pub fn default() -> String {
        String::from("en")
    }
}

#[derive(Clone, Default, Debug)]
pub struct DueDateBuilder {
    date: Option<String>,
    string: Option<String>,
    lang: Option<String>,
    is_recurring: bool,
    no_date: bool,
}

impl DueDateBuilder {
    /// Initialize relevant fields from a parsable string. The list of supported formats can be
    /// found [here](https://todoist.com/help/articles/due-dates-and-times).
    #[tracing::instrument]
    pub fn from_string(&mut self, date_string: &str) -> &mut Self {
        panic!("Not implemented");
    }

    /// Set the [DueDate] to have no date, this sets the `date` and `string` fields to "No date"
    /// and marks the object's `is_recurring` as `false`.
    #[tracing::instrument]
    pub fn no_date(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.no_date = true;
        new.date = None;
        new.string = None;
        if new.is_recurring {
            tracing::event!(tracing::Level::WARN, "Even with no date can't be recurring");
            new.is_recurring = false;
        }

        new
    }

    /// Set the [DueDate] to be recurring. If `no_date` is set, this function is a no-op.
    #[tracing::instrument]
    pub fn is_recurring(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        if new.no_date {
            tracing::event!(
                tracing::Level::WARN,
                "'no_date' is set, not marking DueDate as recurring"
            );
            return new;
        }
        new.is_recurring = value;
        new
    }

    /// Set the language used to interpret and parse the string date. Supported languages are
    /// listed [here](https://developer.todoist.com/sync/v8/#due-dates).
    /// Default's to "en" if an unsupported language is given
    #[tracing::instrument]
    pub fn lang(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        match SupportedLang::check_lang(value) {
            Ok(_) => new.lang = Some(value.to_string()),
            Err(_) => {
                tracing::event!(
                    tracing::Level::WARN,
                    "Unsupported language {:?}, defaulting to 'en'",
                    value
                );
                new.lang = Some(SupportedLang::default());
            }
        }
        new
    }

    #[tracing::instrument]
    pub fn build(&self) -> Result<DueDate, &'static str> {
        Ok(DueDate {
            date: match self.date {
                Some(ref value) => Clone::clone(value),
                None => match self.no_date {
                    true => "No date".to_string(),
                    false => return Err("No date given but no_date field is unset"),
                },
            },
            timezone: "null".to_string(),
            string: match self.string {
                Some(ref value) => Clone::clone(value),
                None => match self.no_date {
                    true => "No date".to_string(),
                    false => return Err("No date given but no_date field is unset"),
                },
            },
            lang: match self.lang {
                Some(ref value) => Clone::clone(value),
                None => "en".to_string(),
            },
            is_recurring: self.is_recurring,
            no_date: self.no_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::dates::{DueDate, DueDateBuilder, SupportedLang};

    #[test]
    fn builder_from_object_test() {
        let expected = DueDateBuilder::default().no_date(true).build().unwrap();

        match DueDate::builder().no_date(true).build() {
            Ok(value) => assert_eq!(value, expected),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn empty_test() {
        match DueDateBuilder::default().build() {
            Ok(_) => panic!("Empty builder should fail"),
            Err(value) => assert_eq!(value, "No date given but no_date field is unset"),
        };
    }

    #[test]
    fn no_date_test() {
        let expected = DueDate {
            date: "No date".to_string(),
            timezone: "null".to_string(),
            string: "No date".to_string(),
            lang: "en".to_string(),
            is_recurring: false,
            no_date: true,
        };
        match DueDateBuilder::default().no_date(true).build() {
            Ok(value) => assert_eq!(value, expected),
            Err(e) => panic!("{}", e),
        }

        match DueDateBuilder::default()
            .is_recurring(true)
            .no_date(true)
            .build()
        {
            Ok(value) => assert_eq!(value, expected),
            Err(e) => panic!("{}", e),
        }

        match DueDateBuilder::default()
            .no_date(true)
            .is_recurring(true)
            .build()
        {
            Ok(value) => assert_eq!(value, expected),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn supported_lang_test() {
        for lang in SupportedLang::supported_lang() {
            let expected = DueDate {
                date: "No date".to_string(),
                timezone: "null".to_string(),
                string: "No date".to_string(),
                lang: String::from(lang),
                is_recurring: false,
                no_date: true,
            };

            match DueDateBuilder::default().no_date(true).lang(lang).build() {
                Ok(value) => assert_eq!(value, expected),
                Err(e) => panic!("{}", e),
            }
        }
    }

    #[test]
    fn unsupported_lang_test() {
        let expected = DueDate {
            date: "No date".to_string(),
            timezone: "null".to_string(),
            string: "No date".to_string(),
            lang: String::from("en"),
            is_recurring: false,
            no_date: true,
        };
        for lang in vec!["bla", "foo", "bar"] {
            match DueDateBuilder::default().no_date(true).lang(lang).build() {
                Ok(value) => assert_eq!(value, expected),
                Err(e) => panic!("{}", e),
            }
        }
    }
}
