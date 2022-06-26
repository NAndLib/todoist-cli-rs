//! Implements the [Todoist Sync API projects].
//!
//! ## Example:
//! ```
//! use todoist_core::types::projects::Project;
//! use todoist_core::types::colors::Colors;
//!
//! // Make a builder.
//! let mut builder = Project::builder();
//!
//! // ID is not required for new projects, but is needed to use `to_builder`.
//! builder.id(1);
//! builder.name("Some project");
//!
//! // Make the project.
//! let project = builder.build().unwrap();
//!
//! // ...
//!
//! // Get a builder from an existing project to edit.
//! let mut builder = project.to_builder().unwrap();
//! builder.color(Colors::BerryRed);
//!
//! // Make the now updated project
//! let project = builder.build().unwrap();
//! ```
//!
//! [Todoist Sync API projects]: https://developer.todoist.com/sync/v8/#projects
use serde::{Deserialize, Serialize};
use tracing;

use crate::types::colors::Colors;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    id: Option<u64>,
    name: String,
    color: Colors,
    parent_id: Option<u64>,
    child_order: u32,
    collapsed: bool,
    shared: bool,
    sync_id: Option<u64>,
    is_deleted: bool,
    is_archived: bool,
    is_favorite: bool,
    inbox_project: bool,
}

impl Project {
    pub fn builder() -> ProjectBuilder {
        ProjectBuilder::default()
    }

    pub fn to_builder(&self) -> Result<ProjectBuilder, &'static str> {
        Ok(ProjectBuilder {
            id: match self.id {
                Some(value) => Some(value),
                None => return Err("Builder from project with no ID not allowed."),
            },
            name: Some(Clone::clone(&self.name)),
            color: Some(Clone::clone(&self.color)),
            parent_id: Clone::clone(&self.parent_id),
            child_order: self.child_order,
            collapsed: self.collapsed,
            shared: self.shared,
            sync_id: Clone::clone(&self.sync_id),
            is_deleted: self.is_deleted,
            is_archived: self.is_archived,
            is_favorite: self.is_favorite,
            inbox_project: self.inbox_project,
        })
    }
}

#[derive(Clone, Default, Debug)]
pub struct ProjectBuilder {
    id: Option<u64>,
    name: Option<String>,
    color: Option<Colors>,
    parent_id: Option<u64>,
    child_order: u32,
    collapsed: bool,
    shared: bool,
    sync_id: Option<u64>,
    is_deleted: bool,
    is_archived: bool,
    is_favorite: bool,
    inbox_project: bool,
}

impl ProjectBuilder {
    /// The ID of the project. Not required for new projects.
    #[tracing::instrument]
    pub fn id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.id = Some(value);
        new
    }

    /// The name of the project.
    #[tracing::instrument]
    pub fn name(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.name = Some(String::from(value));
        new
    }

    /// The color for the project icon. Refer to [Colors] for list of supported colors. Default:
    /// [Colors::default()]
    #[tracing::instrument]
    pub fn color(&mut self, value: Colors) -> &mut Self {
        let mut new = self;
        new.color = Some(Clone::clone(&value));
        new
    }

    /// The ID of the parent project. Default: `None`.
    #[tracing::instrument]
    pub fn parent_id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.parent_id = Some(value);
        new
    }

    /// The order of the project in the list of projects with the same [`parent_id`]. The smallest
    /// value is placed at the top of the list.
    #[tracing::instrument]
    pub fn child_order(&mut self, value: u32) -> &mut Self {
        let mut new = self;
        new.child_order = value;
        new
    }

    /// Whether the project's sub-projects are collapsed. Default: `false`.
    #[tracing::instrument]
    pub fn collapsed(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.collapsed = value;
        new
    }

    /// Whether the project is shared. Currently unsupported and is always `false`.
    #[tracing::instrument]
    pub fn shared(&mut self, _value: bool) -> &mut Self {
        panic!("Shared projects not supported.")
    }

    /// Identifier to find the match between different copies of shared projects. Currently
    /// unsupported and is always `None`.
    #[tracing::instrument]
    pub fn sync_id(&mut self, _value: u64) -> &mut Self {
        panic!("Shared projects not supported.")
    }

    /// Whether the project is marked as deleted.
    #[tracing::instrument]
    pub fn is_deleted(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_deleted = value;
        new
    }

    /// Whether the project is marked as archived.
    #[tracing::instrument]
    pub fn is_archived(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_archived = value;
        new
    }

    /// Whether the project is marked as favorite.
    #[tracing::instrument]
    pub fn is_favorite(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_favorite = value;
        new
    }

    /// Whether the project is `Inbox`. If this is set to `true`, the project's name will be
    /// changed to `Inbox`.
    #[tracing::instrument]
    pub fn inbox_project(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        match &new.name {
            Some(value) => {
                if *value != "Inbox" {
                    tracing::event!(
                        tracing::Level::WARN,
                        "Project marked as inbox project, changing name to `Inbox`"
                    );
                    new.name = Some(String::from("Inbox"));
                }
            }
            None => {
                new.name = Some(String::from("Inbox"));
            }
        }
        new.inbox_project = value;
        new
    }

    pub fn build(&self) -> Result<Project, &'static str> {
        Ok(Project {
            id: Clone::clone(&self.id),
            name: match &self.name {
                Some(value) => {
                    if self.inbox_project && *value != "Inbox" {
                        return Err("Project is not named 'Inbox' but is marked as inbox project");
                    }
                    Clone::clone(value)
                }
                None => return Err("Project has no name."),
            },
            color: match self.color {
                Some(ref value) => Clone::clone(value),
                None => Colors::default(),
            },
            parent_id: Clone::clone(&self.parent_id),
            child_order: self.child_order,
            collapsed: self.collapsed,
            shared: self.shared,
            sync_id: Clone::clone(&self.sync_id),
            is_deleted: self.is_deleted,
            is_archived: self.is_archived,
            is_favorite: self.is_favorite,
            inbox_project: self.inbox_project,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::colors::Colors;
    use crate::types::projects::{Project, ProjectBuilder};

    #[test]
    fn error_test() {
        match ProjectBuilder::default().build() {
            Ok(_) => panic!("Project with no name should fail."),
            Err(value) => assert_eq!(value, "Project has no name."),
        }

        ProjectBuilder::default().name("Foo").build().unwrap();
    }

    #[test]
    fn project_create_test() {
        let expected = Project {
            id: Some(1),
            name: String::from("Foo"),
            color: Colors::default(),
            parent_id: None,
            child_order: 0,
            collapsed: false,
            shared: false,
            sync_id: None,
            is_deleted: false,
            is_archived: false,
            is_favorite: false,
            inbox_project: false,
        };

        let actual = Project::builder().id(1).name("Foo").build().unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn project_update_test() {
        let mut expected = Project {
            id: None,
            name: String::from("Foo"),
            color: Colors::default(),
            parent_id: None,
            child_order: 0,
            collapsed: false,
            shared: false,
            sync_id: None,
            is_deleted: false,
            is_archived: false,
            is_favorite: false,
            inbox_project: false,
        };

        let actual = Project::builder().name("Foo").build().unwrap();

        assert_eq!(actual, expected);

        match expected.to_builder() {
            Ok(_) => panic!("`to_builder` with no ID should fail."),
            Err(value) => assert_eq!(value, "Builder from project with no ID not allowed."),
        };

        expected.id = Some(1);

        let mut builder = expected.to_builder().unwrap();
        builder.name("Bar");

        let actual = builder.build().unwrap();
        assert_ne!(actual, expected);

        expected.name = String::from("Bar");
        assert_eq!(actual, expected);
    }

    #[test]
    fn inbox_project_test() {
        ProjectBuilder::default()
            .name("Not inbox")
            .inbox_project(true)
            .build()
            .unwrap();

        match ProjectBuilder::default()
            .inbox_project(true)
            .name("Not inbox")
            .build()
        {
            Ok(_) => panic!("Project not named 'Inbox' should fail when marked as inbox project"),
            Err(value) => assert_eq!(
                value,
                "Project is not named 'Inbox' but is marked as inbox project"
            ),
        }
    }
}
