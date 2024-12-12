use crate::Todo;

pub enum FilteringMethod {
    Title(String),
    UserId(i64),
    Completed(bool),
}

pub fn filter_todos(todos: &Vec<Todo>, method: FilteringMethod) -> Vec<Todo> {
    match method {
        FilteringMethod::Title(title) => todos
            .iter()
            .filter(|todo| {
                todo.title
                    .contains(&title)
            })
            .cloned()
            .collect(),
        FilteringMethod::UserId(user_id) => todos
            .iter()
            .filter(|todo| todo.user_id == user_id)
            .cloned()
            .collect(),
        FilteringMethod::Completed(status) => todos
            .iter()
            .filter(|todo| todo.completed == status)
            .cloned()
            .collect(),
    }
}
