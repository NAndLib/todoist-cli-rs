//! Implements the [Todoist Sync API labels]
//!
//! ## Example:
//! ```
//! use todoist_core::types::labels::Label;
//!
//! // Make a builder
//! let mut builder = Label::builder();
//! // not needed for new labels, but required to use `to_builder` to edit an existing label
//! builder.id(1);
//! builder.name("Foo bar");
//!
//! // Make the label
//! let label = builder.build().unwrap();
//!
//! // ...
//!
//! // Mark the existing label as favorite
//! let mut builder = label.to_builder().unwrap();
//! builder.is_favorite(true);
//!
//! let label = builder.build().unwrap();
//! ```
//!
//! [Todoist Sync API labels]: https://developer.todoist.com/sync/v8/#labels
use serde::{Deserialize, Serialize};
use tracing;

use crate::types::colors::Colors;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Label {
    id: Option<u64>,
    name: String,
    color: Colors,
    item_order: u32,
    is_deleted: bool,
    is_favorite: bool,
}

impl Label {
    pub fn builder() -> LabelBuilder {
        LabelBuilder::default()
    }

    pub fn to_builder(&self) -> Result<LabelBuilder, &'static str> {
        Ok(LabelBuilder {
            id: match self.id {
                Some(value) => Some(value),
                None => return Err("Builder from label with no ID not allowed"),
            },
            name: Some(Clone::clone(&self.name)),
            color: Some(Clone::clone(&self.color)),
            item_order: self.item_order,
            is_deleted: self.is_deleted,
            is_favorite: self.is_favorite,
        })
    }
}

#[derive(Clone, Default, Debug)]
pub struct LabelBuilder {
    id: Option<u64>,
    name: Option<String>,
    color: Option<Colors>,
    item_order: u32,
    is_deleted: bool,
    is_favorite: bool,
}

impl LabelBuilder {
    /// The ID of the label. Not required for new labels.
    #[tracing::instrument]
    pub fn id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.id = Some(value);
        new
    }

    /// The name of the label.
    #[tracing::instrument]
    pub fn name(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.name = Some(String::from(value));
        new
    }

    /// The color for the label. See [Colors] for list of supported color.
    /// Default: [Colors::default()]
    #[tracing::instrument]
    pub fn color(&mut self, value: &Colors) -> &mut Self {
        let mut new = self;
        new.color = Some(Clone::clone(value));
        new
    }

    /// The order of the label in the list. The smallest value is placed at the top of the list.
    #[tracing::instrument]
    pub fn item_order(&mut self, value: u32) -> &mut Self {
        let mut new = self;
        new.item_order = value;
        new
    }

    /// Whether the label is marked as deleted.
    #[tracing::instrument]
    pub fn is_deleted(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_deleted = value;
        new
    }

    /// Whether the label is marked as favorite.
    #[tracing::instrument]
    pub fn is_favorite(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_favorite = value;
        new
    }

    pub fn build(&self) -> Result<Label, &'static str> {
        Ok(Label {
            id: Clone::clone(&self.id),
            name: match self.name {
                Some(ref value) => Clone::clone(value),
                None => return Err("Label must have a name."),
            },
            color: match self.color {
                Some(ref value) => Clone::clone(value),
                None => Colors::default(),
            },
            item_order: self.item_order,
            is_deleted: self.is_deleted,
            is_favorite: self.is_favorite,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::colors::Colors;
    use crate::types::labels::{Label, LabelBuilder};

    #[test]
    fn error_test() {
        match LabelBuilder::default().build() {
            Ok(_) => panic!("Label with no name should fail"),
            Err(value) => assert_eq!(value, "Label must have a name."),
        }

        LabelBuilder::default().name("Foo bar").build().unwrap();
    }

    #[test]
    fn label_create_test() {
        let expected = Label {
            id: None,
            name: String::from("Foo bar"),
            color: Colors::default(),
            item_order: 0,
            is_deleted: false,
            is_favorite: false,
        };

        let actual = Label::builder().name("Foo bar").build().unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn label_update_test() {
        let mut expected = Label {
            id: Some(1),
            name: String::from("Foo bar"),
            color: Colors::default(),
            item_order: 0,
            is_deleted: false,
            is_favorite: false,
        };

        let actual = Label::builder().id(1).name("Foo bar").build().unwrap();

        assert_eq!(actual, expected);

        let mut builder = expected.to_builder().unwrap();
        builder.color(&Colors::BerryRed);

        let actual = builder.build().unwrap();
        assert_ne!(actual, expected);

        expected.color = Colors::BerryRed;
        assert_eq!(actual, expected);
    }
}
