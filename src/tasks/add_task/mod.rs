use std::fs;

use crate::utils;

pub fn add_task() {
  let task_to_add = utils::get_a_string(String::from("Enter the task to add"));
  let mut new_data = utils::get_tasks_data().expect("No tasks file.");

  new_data.push_str("\nTODO|");
  new_data.push_str(task_to_add.as_str());
  new_data.push('\n');

  fs::write("./src/tasks.txt", new_data.trim()).expect("Error while writing file.");
}
