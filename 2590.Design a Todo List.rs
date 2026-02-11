use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Task {
    task_id: i32,
    description: String,
    tags: HashSet<String>,
    due_date: i32,
}

struct TodoList {
    next_id: i32,
    user_tasks: HashMap<i32, Vec<Task>>,
}

impl TodoList {
    /// A TodoList that supports adding, retrieving, filtering, and completing tasks per user.
    ///
    /// # Intuition
    /// Each user maintains a list of tasks. Tasks are globally identified by an
    /// auto-incrementing ID. Completing a task removes it from the user's list.
    ///
    /// # Approach
    /// - Store tasks per user in a HashMap<i32, Vec<Task>>
    /// - Auto-increment task IDs globally
    /// - Sort by due_date when retrieving tasks
    /// - Filter by tag using HashSet membership
    ///
    /// # Complexity
    /// - add_task: O(1)
    /// - get_all_tasks / get_tasks_for_tag: O(n log n) due to sorting
    /// - complete_task: O(n) linear scan and removal
    /// - Space: O(total tasks)
    fn new() -> Self {
        Self {
            next_id: 1,
            user_tasks: HashMap::new(),
        }
    }

    fn add_task(
        &mut self,
        user_id: i32,
        task_description: String,
        due_date: i32,
        tags: Vec<String>,
    ) -> i32 {
        let task = Task {
            task_id: self.next_id,
            description: task_description,
            tags: tags.into_iter().collect(),
            due_date,
        };
        self.user_tasks
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(task);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn get_all_tasks(&self, user_id: i32) -> Vec<String> {
        self.user_tasks
            .get(&user_id)
            .map(|tasks| {
                let mut sorted = tasks.clone();
                sorted.sort_unstable_by_key(|t| t.due_date);
                sorted.into_iter().map(|t| t.description).collect()
            })
            .unwrap_or_default()
    }

    fn get_tasks_for_tag(&self, user_id: i32, tag: String) -> Vec<String> {
        self.user_tasks
            .get(&user_id)
            .map(|tasks| {
                let mut sorted: Vec<_> = tasks
                    .iter()
                    .filter(|t| t.tags.contains(&tag))
                    .cloned()
                    .collect();
                sorted.sort_unstable_by_key(|t| t.due_date);
                sorted.into_iter().map(|t| t.description).collect()
            })
            .unwrap_or_default()
    }

    fn complete_task(&mut self, user_id: i32, task_id: i32) {
        if let Some(tasks) = self.user_tasks.get_mut(&user_id) {
            tasks.retain(|t| t.task_id != task_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_list_workflow() {
        let mut todo = TodoList::new();
        let id1 = todo.add_task(1, "Task1".into(), 50, vec!["work".into()]);
        let id2 = todo.add_task(1, "Task2".into(), 30, vec!["home".into()]);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(todo.get_all_tasks(1), vec!["Task2", "Task1"]);

        todo.complete_task(1, id2);
        assert_eq!(todo.get_all_tasks(1), vec!["Task1"]);
    }

    #[test]
    fn test_filter_by_tag() {
        let mut todo = TodoList::new();
        todo.add_task(1, "A".into(), 10, vec!["x".into()]);
        todo.add_task(1, "B".into(), 20, vec!["y".into()]);
        todo.add_task(1, "C".into(), 5, vec!["x".into()]);

        assert_eq!(todo.get_tasks_for_tag(1, "x".into()), vec!["C", "A"]);
    }

    #[test]
    fn test_nonexistent_user() {
        let todo = TodoList::new();
        assert!(todo.get_all_tasks(999).is_empty());
    }
}
