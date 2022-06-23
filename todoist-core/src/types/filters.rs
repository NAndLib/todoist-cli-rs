//! Implements the [Todoist Sync API filters]
//!
//! ## Example
//! ```
//! use todoist_core::types::filters::Filter;
//! use todoist_core::types::colors::Colors;
//!
//! // Make a filter
//! let mut builder = Filter::builder();
//! // Add an ID
//! builder.id(1);
//! // Add a name
//! builder.name("Foo");
//! // Add a query
//! builder.query("today | overdue");
//! // Change the color
//! builder.color(Colors::Red);
//!
//! let filter = builder.build();
//! ```
//!
//! [Todoist Sync API filters]: https://developer.todoist.com/sync/v8/#filters
use serde::{Deserialize, Serialize};
use tracing;

use crate::types::colors::Colors;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Filter {
    id: Option<u64>,
    name: String,
    query: String,
    color: Colors,
    item_order: u32,
    is_deleted: bool,
    is_favorite: bool,
}

impl Filter {
    /// [Filter]s should only be built using the [FilterBuilder]
    pub fn builder() -> FilterBuilder {
        FilterBuilder::default()
    }
}

#[derive(Clone, Default, Debug)]
pub struct FilterBuilder {
    id: Option<u64>,
    name: Option<String>,
    query: Option<String>,
    color: Colors,
    item_order: u32,
    is_deleted: bool,
    is_favorite: bool,
}

impl FilterBuilder {
    /// The ID of the filter.
    #[tracing::instrument]
    pub fn id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.id = Some(value);

        new
    }

    /// The name of the filter.
    #[tracing::instrument]
    pub fn name(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.name = Some(String::from(value));

        new
    }

    /// The query to search for. [Example of searches](https://todoist.com/help/articles/205248842)
    /// can be found in the Todoist help page.
    #[tracing::instrument]
    pub fn query(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.query = Some(String::from(value));

        new
    }

    /// The color of the filter, refer to [Colors] for list of supported colors
    #[tracing::instrument]
    pub fn color(&mut self, value: Colors) -> &mut Self {
        let mut new = self;
        new.color = value;

        new
    }

    /// The order of the filter in the filter list. Smallest value will place the filter at the top
    #[tracing::instrument]
    pub fn item_order(&mut self, value: u32) -> &mut Self {
        let mut new = self;
        new.item_order = value;

        new
    }

    /// Whether the filter is marked as deleted
    #[tracing::instrument]
    pub fn is_deleted(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_deleted = value;

        new
    }

    /// Whether the filter is marked as favorite
    #[tracing::instrument]
    pub fn is_favorite(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_favorite = value;

        new
    }

    pub fn build(&self) -> Result<Filter, &'static str> {
        Ok(Filter {
            id: self.id,
            name: match self.name {
                Some(ref value) => Clone::clone(value),
                None => return Err("No name set for filter"),
            },
            query: match self.query {
                Some(ref value) => Clone::clone(value),
                None => return Err("No query set for filter"),
            },
            color: Clone::clone(&self.color),
            item_order: self.item_order,
            is_deleted: self.is_deleted,
            is_favorite: self.is_favorite,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::colors::Colors;
    use crate::types::filters::{Filter, FilterBuilder};

    #[test]
    fn error_test() {
        match FilterBuilder::default().build() {
            Ok(_) => panic!("Builder with no ID, name, or query should fail"),
            Err(value) => assert_eq!(value, "No name set for filter"),
        }

        match FilterBuilder::default().id(1).build() {
            Ok(_) => panic!("Builder with no ID, name, or query should fail"),
            Err(value) => assert_eq!(value, "No name set for filter"),
        }

        match FilterBuilder::default().id(1).name("foo").build() {
            Ok(_) => panic!("Builder with no ID, name, or query should fail"),
            Err(value) => assert_eq!(value, "No query set for filter"),
        }

        let expected = Filter {
            id: Some(1),
            name: String::from("foo"),
            query: String::from("bar"),
            color: Colors::default(),
            item_order: 0,
            is_deleted: false,
            is_favorite: false,
        };

        let actual = FilterBuilder::default()
            .id(1)
            .name("foo")
            .query("bar")
            .build()
            .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn simple_test() {
        let expected = Filter {
            id: Some(1),
            name: String::from("foo"),
            query: String::from("bar"),
            color: Colors::Red,
            item_order: 0,
            is_deleted: false,
            is_favorite: false,
        };
        let actual = FilterBuilder::default()
            .id(1)
            .name("foo")
            .query("bar")
            .color(Colors::Red)
            .build()
            .unwrap();

        assert_eq!(actual, expected)
    }
}
