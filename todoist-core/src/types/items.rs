//! Implements the [Todoist Sync API items or tasks]
//!
//! Currently, the type does not support collaborative projects and items.
//!
//! ## Example
//!
//! [Todoist Sync API items or tasks]: https://developer.todoist.com/sync/v8/#items
use serde::{Deserialize, Serialize};
use tracing;

use crate::types::dates::DueDate;
use crate::types::priority::Priority;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    id: Option<u64>,
    user_id: u64,
    project_id: u64,
    content: String,
    description: Option<String>,
    due: DueDate,
    priority: Priority,
    parent_id: Option<u64>,
    child_order: u32,
    section_id: Option<u64>,
    day_order: u32,
    collapsed: bool,
    labels: Vec<u64>,
    checked: bool,
    is_deleted: bool,
    date_completed: Option<String>,
    date_added: String,
}

impl Item {
    /// Returns a default [ItemBuilder].
    pub fn builder() -> ItemBuilder {
        ItemBuilder::default()
    }

    /// Returns an [ItemBuilder] that can be used to modify an existing [Item].
    pub fn to_builder(&self) -> Result<ItemBuilder, &'static str> {
        Ok(ItemBuilder {
            id: match self.id {
                None => return Err("Builder from Item with no ID not allowed."),
                Some(value) => Some(value),
            },
            user_id: match self.user_id {
                0 => None,
                value => Some(value),
            },
            project_id: match self.project_id {
                0 => None,
                value => Some(value),
            },
            content: Some(Clone::clone(&self.content)),
            description: Clone::clone(&self.description),
            due: Some(Clone::clone(&self.due)),
            priority: Some(Clone::clone(&self.priority)),
            parent_id: self.parent_id,
            child_order: Some(self.child_order),
            section_id: self.section_id,
            day_order: Some(self.day_order),
            collapsed: self.collapsed,
            labels: Some(Clone::clone(&self.labels)),
            checked: self.checked,
            is_deleted: self.is_deleted,
            date_completed: Clone::clone(&self.date_completed),
            date_added: Some(Clone::clone(&self.date_added)),
        })
    }
}

#[derive(Clone, Default, Debug)]
pub struct ItemBuilder {
    id: Option<u64>,
    user_id: Option<u64>,
    project_id: Option<u64>,
    content: Option<String>,
    description: Option<String>,
    due: Option<DueDate>,
    priority: Option<Priority>,
    parent_id: Option<u64>,
    child_order: Option<u32>,
    section_id: Option<u64>,
    day_order: Option<u32>,
    collapsed: bool,
    labels: Option<Vec<u64>>,
    checked: bool,
    is_deleted: bool,
    date_completed: Option<String>,
    date_added: Option<String>,
}

impl ItemBuilder {
    /// The ID of the task, unnecessary if creating a new task.
    #[tracing::instrument]
    pub fn id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.id = Some(value);
        new
    }

    /// The owner of the task. Required.
    #[tracing::instrument]
    pub fn user_id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.user_id = Some(value);
        new
    }

    /// The ID of the parent project.
    #[tracing::instrument]
    pub fn project_id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.project_id = Some(value);
        new
    }

    /// The text of the task. This value may contain markdown-formatted text and hyperlinks.
    /// Required.
    #[tracing::instrument]
    pub fn content(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.content = Some(String::from(value));
        new
    }

    /// The description of the task. This value may contain markdown-formatted text and hyperlinks.
    #[tracing::instrument]
    pub fn description(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.description = Some(String::from(value));
        new
    }

    /// The due date of the task. Default: ["No date"][DueDate::default()]
    #[tracing::instrument]
    pub fn due(&mut self, value: &DueDate) -> &mut Self {
        let mut new = self;
        new.due = Some(Clone::clone(value));
        new
    }

    /// The priority of the task. Default: [P4][Priority::P4].
    #[tracing::instrument]
    pub fn priority(&mut self, value: &Priority) -> &mut Self {
        let mut new = self;
        new.priority = Some(Clone::clone(value));
        new
    }

    /// The ID of the parent task. Default is `None` for root task.
    #[tracing::instrument]
    pub fn parent_id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.parent_id = Some(value);
        new
    }

    /// The order of the task among all the tasks with the same parent. The smallest number would
    /// be place at the top. Default: 0.
    #[tracing::instrument]
    pub fn child_order(&mut self, value: u32) -> &mut Self {
        let mut new = self;
        new.child_order = Some(value);
        new
    }

    /// The ID of the parent section. Default is `None` for tasks not belonging to a section.
    #[tracing::instrument]
    pub fn section_id(&mut self, value: u64) -> &mut Self {
        let mut new = self;
        new.section_id = Some(value);
        new
    }

    /// The order of the task inside the `Today` or `Next 7 days` view. The smallest number will be
    /// placed at the top. Default: 0.
    #[tracing::instrument]
    pub fn day_order(&mut self, value: u32) -> &mut Self {
        let mut new = self;
        new.day_order = Some(value);
        new
    }

    /// Whether the task's sub-tasks are collapsed. Default: `false`.
    #[tracing::instrument]
    pub fn collapsed(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.collapsed = value;
        new
    }

    /// Add a label by its ID. Will allocate a new [Vec] if there are no existing labels.
    #[tracing::instrument]
    pub fn label_add(&mut self, value: u64) -> &mut Self {
        let mut new = self;

        let mut labels: Vec<u64> = match &new.labels {
            Some(value) => Clone::clone(value),
            None => Vec::new(),
        };
        labels.push(value);

        new.labels = Some(labels);

        new
    }

    /// Remove a label by its ID. Will deallocate the internal [Vec] if no labels are left.
    #[tracing::instrument]
    pub fn label_remove(&mut self, value: u64) -> &mut Self {
        let mut new = self;

        let mut labels: Vec<u64> = match &new.labels {
            Some(value) => Clone::clone(value),
            None => return new,
        };

        if labels.contains(&value) {
            // Unwrap is safe here due to member check
            let pos = labels.iter().position(|&x| x == value).unwrap();
            labels.swap_remove(pos);
        }
        if labels.is_empty() {
            new.labels = None;
        } else {
            new.labels = Some(labels);
        }

        new
    }

    /// Whether the task is marked as completed.
    #[tracing::instrument]
    pub fn checked(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.checked = value;
        new
    }

    /// Whether the task is marked as deleted.
    #[tracing::instrument]
    pub fn is_deleted(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.is_deleted = value;
        new
    }

    /// The date when the task was completed. `None` if the task is not completed.
    #[tracing::instrument]
    pub fn date_completed(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.date_completed = Some(String::from(value));
        new
    }

    /// The date when the task was created.
    #[tracing::instrument]
    pub fn date_added(&mut self, value: &str) -> &mut Self {
        let mut new = self;
        new.date_added = Some(String::from(value));
        new
    }

    pub fn build(&self) -> Result<Item, &'static str> {
        Ok(Item {
            id: self.id,
            user_id: match self.user_id {
                Some(value) => value,
                None => return Err("Task has no user ID."),
            },
            project_id: match self.project_id {
                Some(value) => value,
                None => return Err("Task has no project ID."),
            },
            content: match self.content {
                Some(ref value) => Clone::clone(value),
                None => return Err("Task has no content."),
            },
            description: Clone::clone(&self.description),
            due: match self.due {
                Some(ref value) => Clone::clone(value),
                None => DueDate::default(),
            },
            priority: match self.priority {
                Some(ref value) => Clone::clone(value),
                None => Priority::default(),
            },
            parent_id: self.parent_id,
            child_order: self.child_order.unwrap_or(0),
            section_id: self.section_id,
            day_order: self.day_order.unwrap_or(0),
            collapsed: self.collapsed,
            labels: match self.labels {
                Some(ref value) => Clone::clone(value),
                None => Vec::new(),
            },
            checked: self.checked,
            is_deleted: self.is_deleted,
            date_completed: match self.date_completed {
                Some(ref value) => {
                    if !self.checked {
                        return Err("Uncompleted task can't have a completion date");
                    }
                    Some(Clone::clone(value))
                }
                None => {
                    if self.checked {
                        return Err("Completed task must have a completion date.");
                    }
                    None
                }
            },
            date_added: match self.date_added {
                Some(ref value) => Clone::clone(value),
                None => return Err("Task has no creation date."),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::dates::DueDate;
    use crate::types::items::{Item, ItemBuilder};
    use crate::types::priority::Priority;

    #[test]
    fn error_test() {
        match ItemBuilder::default().build() {
            Ok(_) => panic!("ItemBuilder with no fields should fail."),
            Err(e) => assert_eq!(e, "Task has no user ID."),
        };

        match ItemBuilder::default().user_id(1).build() {
            Ok(_) => panic!("ItemBuilder with no fields should fail."),
            Err(e) => assert_eq!(e, "Task has no project ID."),
        };

        match ItemBuilder::default().user_id(1).project_id(1).build() {
            Ok(_) => panic!("ItemBuilder with no fields should fail."),
            Err(e) => assert_eq!(e, "Task has no content."),
        };

        match ItemBuilder::default()
            .user_id(1)
            .project_id(1)
            .content("Lorem ipsum")
            .build()
        {
            Ok(_) => panic!("ItemBuilder with no fields should fail."),
            Err(e) => assert_eq!(e, "Task has no creation date."),
        };

        ItemBuilder::default()
            .user_id(1)
            .project_id(1)
            .content("Lorem ipsum")
            .date_added("1999-01-01")
            .build()
            .unwrap();
    }

    #[test]
    fn new_item_test() {
        let expected = Item {
            id: None,
            user_id: 1,
            project_id: 1,
            content: String::from("Lorem ipsum"),
            description: None,
            due: DueDate::default(),
            priority: Priority::default(),
            parent_id: None,
            child_order: 0,
            section_id: None,
            day_order: 0,
            collapsed: false,
            labels: Vec::new(),
            checked: false,
            is_deleted: false,
            date_completed: None,
            date_added: String::from("1999-01-01"),
        };
        let actual = Item::builder()
            .user_id(1)
            .project_id(1)
            .content("Lorem ipsum")
            .date_added("1999-01-01")
            .build()
            .unwrap();

        assert_eq!(expected, actual)
    }

    #[test]
    fn modified_item_test() {
        let mut base = Item {
            id: None,
            user_id: 1,
            project_id: 1,
            content: String::from("Lorem ipsum"),
            description: None,
            due: DueDate::default(),
            priority: Priority::default(),
            parent_id: None,
            child_order: 0,
            section_id: None,
            day_order: 0,
            collapsed: false,
            labels: Vec::new(),
            checked: false,
            is_deleted: false,
            date_completed: None,
            date_added: String::from("1999-01-01"),
        };

        match base.to_builder() {
            Ok(_) => panic!("Getting builder from Item with no ID should fail."),
            Err(value) => assert_eq!(value, "Builder from Item with no ID not allowed."),
        };

        base.id = Some(1);

        let mut builder = base.to_builder().unwrap();
        let actual = builder.build().unwrap();
        assert_eq!(base, actual);

        builder.content("Dolor sit met");

        let actual = builder.build().unwrap();
        assert_ne!(base, actual);

        base.content = String::from("Dolor sit met");
        assert_eq!(base, actual);
    }

    #[test]
    fn labels_test() {
        let mut base = Item {
            id: Some(1),
            user_id: 1,
            project_id: 1,
            content: String::from("Lorem ipsum"),
            description: None,
            due: DueDate::default(),
            priority: Priority::default(),
            parent_id: None,
            child_order: 0,
            section_id: None,
            day_order: 0,
            collapsed: false,
            labels: Vec::new(),
            checked: false,
            is_deleted: false,
            date_completed: None,
            date_added: String::from("1999-01-01"),
        };

        let mut builder = base.to_builder().unwrap();

        let mut labels: Vec<u64> = Vec::new();
        for i in 1..6 {
            builder.label_add(i);
            labels.push(i);
        }

        let actual = builder.build().unwrap();
        assert_ne!(base, actual);

        base.labels = Clone::clone(&labels);
        assert_eq!(base, actual);

        builder.label_remove(5);
        labels.pop();

        let actual = builder.build().unwrap();
        assert_ne!(base, actual);

        base.labels = Clone::clone(&labels);
        assert_eq!(base, actual);

        for i in 1..5 {
            builder.label_remove(i);
            labels.pop();
        }

        let actual = builder.build().unwrap();
        assert_ne!(base, actual);

        base.labels = Clone::clone(&labels);
        assert_eq!(base, actual);
    }

    #[test]
    fn checked_test() {
        let base = Item {
            id: Some(1),
            user_id: 1,
            project_id: 1,
            content: String::from("Lorem ipsum"),
            description: None,
            due: DueDate::default(),
            priority: Priority::default(),
            parent_id: None,
            child_order: 0,
            section_id: None,
            day_order: 0,
            collapsed: false,
            labels: Vec::new(),
            checked: false,
            is_deleted: false,
            date_completed: None,
            date_added: String::from("1999-01-01"),
        };

        let mut builder = base.to_builder().unwrap();
        builder.checked(true);

        match builder.build() {
            Ok(_) => panic!("Checked item without completed date should fail."),
            Err(value) => assert_eq!(value, "Completed task must have a completion date."),
        };

        builder.date_completed("2000-01-01");
        builder.build().unwrap();
    }
}
