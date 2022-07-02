//! Implements the [Todoist Sync API section].
//!
//! ## Example
//! ```
//! use todoist_core::types::sections::Section;
//!
//! // Make a builder
//! let mut builder = Section::builder();
//! // not needed for new sections, but required to use `to_builder` to edit an existing label
//! builder.id(1);
//! builder.project_id(1);
//! builder.name("Foo bar");
//! builder.date_added("1999-01-01");
//!
//! // Make the section
//! let section = builder.build().unwrap();
//!
//! // ...
//!
//! // Mark the existing section as favorite
//! let mut builder = section.to_builder().unwrap();
//! builder.date_archived("2000-01-01");
//!
//! let section = builder.build().unwrap();
//! ```
//! [Todoist Sync API section]: https://developer.todoist.com/sync/v8/#sections
use serde::{Deserialize, Serialize};
use tracing;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Section {
    id: Option<u64>,
    name: String,
    project_id: u64,
    section_order: u32,
    collapsed: bool,
    is_deleted: bool,
    is_archived: bool,
    date_archived: Option<String>,
    date_added: String,
}

impl Section {
    pub fn builder() -> SectionBuilder {
        SectionBuilder::default()
    }

    pub fn to_builder(&self) -> Result<SectionBuilder, &'static str> {
        Ok(SectionBuilder {
            id: match self.id {
                None => return Err("Builder from section with no ID not allowed."),
                Some(value) => Some(value),
            },
            name: Some(Clone::clone(&self.name)),
            project_id: Some(self.project_id),
            section_order: self.section_order,
            collapsed: self.collapsed,
            is_deleted: self.is_deleted,
            is_archived: self.is_archived,
            date_archived: Clone::clone(&self.date_archived),
            date_added: Some(Clone::clone(&self.date_added)),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct SectionBuilder {
    id: Option<u64>,
    name: Option<String>,
    project_id: Option<u64>,
    section_order: u32,
    collapsed: bool,
    is_deleted: bool,
    is_archived: bool,
    date_archived: Option<String>,
    date_added: Option<String>,
}

impl SectionBuilder {
    /// The ID of the section. Not required for new projects.
    #[tracing::instrument]
    pub fn id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.id = Some(value);
        new
    }

    /// The name of the section.
    #[tracing::instrument]
    pub fn name(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.name = Some(String::from(value));
        new
    }

    /// The ID of the project that the section resides in.
    #[tracing::instrument]
    pub fn project_id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.project_id = Some(value);
        new
    }

    /// The order of the section in the list of sections in the same [Project]. The smallest value
    /// is placed at the top of the list.
    #[tracing::instrument]
    pub fn section_order(&mut self, value: u32) -> &mut Self {
        let mut new = self;
        new.section_order = value;
        new
    }

    /// Whether the section's tasks are collapsed. Default: `false`.
    #[tracing::instrument]
    pub fn collapsed(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.collapsed = value;
        new
    }

    /// Whether the section is marked as deleted.
    #[tracing::instrument]
    pub fn is_deleted(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_deleted = value;
        new
    }

    /// Whether the section is marked as archived.
    #[tracing::instrument]
    pub fn is_archived(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        if new.date_archived.is_none() {
            tracing::event!(
                tracing::Level::WARN,
                "Section marked as archived but there is no date"
            );
        }
        new.is_archived = value;
        if !new.is_archived {
            new.date_archived = None;
        }
        new
    }

    /// The date when the section was archived.
    #[tracing::instrument]
    pub fn date_archived(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        if !new.is_archived {
            tracing::event!(
                tracing::Level::WARN,
                "Section is not archived with archive date, marking as archived"
            );
            new.is_archived = true;
        }
        new.date_archived = Some(String::from(value));
        new
    }

    /// The date when the section was created.
    #[tracing::instrument]
    pub fn date_added(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.date_added = Some(String::from(value));
        new
    }

    pub fn build(&self) -> Result<Section, &'static str> {
        Ok(Section {
            id: Clone::clone(&self.id),
            name: match self.name {
                Some(ref value) => Clone::clone(value),
                None => return Err("Section does not have a name."),
            },
            project_id: match self.project_id {
                Some(value) => value,
                None => return Err("Section does not have a project ID."),
            },
            section_order: self.section_order,
            collapsed: self.collapsed,
            is_deleted: self.is_deleted,
            is_archived: match self.is_archived {
                true => {
                    if self.date_archived.is_none() {
                        return Err("Section marked as archived with no date.");
                    }
                    true
                }
                _ => self.is_archived,
            },
            date_archived: match self.date_archived {
                Some(ref value) => {
                    if !self.is_archived {
                        return Err("Section has archive date but not marked as archived.");
                    }
                    Some(Clone::clone(value))
                }
                None => None,
            },
            date_added: match self.date_added {
                Some(ref value) => Clone::clone(value),
                None => return Err("Section does not have an added date."),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::sections::{Section, SectionBuilder};

    #[test]
    fn error_test() {
        match SectionBuilder::default().build() {
            Ok(_) => panic!("Section with no name should fail."),
            Err(value) => assert_eq!(value, "Section does not have a name."),
        }

        match SectionBuilder::default().name("Foo").build() {
            Ok(_) => panic!("Section with no project ID should fail."),
            Err(value) => assert_eq!(value, "Section does not have a project ID."),
        }

        match SectionBuilder::default().name("Foo").project_id(1).build() {
            Ok(_) => panic!("Section with no creation date should fail."),
            Err(value) => assert_eq!(value, "Section does not have an added date."),
        }

        SectionBuilder::default()
            .name("Foo")
            .project_id(1)
            .date_added("1999-01-01")
            .build()
            .unwrap();
    }

    #[test]
    fn section_create_test() {
        let expected = Section {
            id: None,
            name: String::from("Foo"),
            project_id: 1,
            section_order: 0,
            collapsed: false,
            is_deleted: false,
            is_archived: false,
            date_archived: None,
            date_added: String::from("1999-01-01"),
        };

        let actual = Section::builder()
            .name("Foo")
            .project_id(1)
            .date_added("1999-01-01")
            .build()
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn section_update_test() {
        let mut expected = Section {
            id: None,
            name: String::from("Foo"),
            project_id: 1,
            section_order: 0,
            collapsed: false,
            is_deleted: false,
            is_archived: false,
            date_archived: None,
            date_added: String::from("1999-01-01"),
        };

        match expected.to_builder() {
            Ok(_) => panic!("Section with no ID should fail when trying to create builder"),
            Err(value) => assert_eq!(value, "Builder from section with no ID not allowed."),
        };

        expected.id = Some(1);

        let mut builder = expected.to_builder().unwrap();
        builder.is_deleted(true);

        let actual = builder.build().unwrap();
        assert_ne!(actual, expected);

        expected.is_deleted = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn section_archive_test() {
        let mut builder = Section::builder();
        builder
            .name("Foo")
            .project_id(1)
            .date_added("1999-01-01")
            .is_archived(true);

        match builder.build() {
            Ok(_) => panic!("Building archived section with no archive date should fail."),
            Err(value) => assert_eq!(value, "Section marked as archived with no date."),
        }

        builder.date_archived("2000-01-01");
        builder.build().unwrap();

        builder.is_archived(false);
        builder.build().unwrap();

        builder.date_archived("2000-02-01");
        builder.build().unwrap();
    }
}
