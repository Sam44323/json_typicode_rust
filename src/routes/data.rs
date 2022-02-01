use serde::*;

#[derive(Serialize)] // converting object to stream of bytes and then converting to actual data using deserialize
pub struct ToDoList {
  pub items: Vec<ToDoItem>,
}

#[derive(Serialize)]
pub struct ToDoItem {
  pub id: i64,
  pub item: String,
}

#[derive(Serialize)]
pub struct StatusMessage {
  pub message: String,
}

#[derive(Serialize)]
pub struct JSONData {
  userId: i32,
  id: i32,
  title: String,
  completed: bool,
}
