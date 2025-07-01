use std::fs;
use crate::utils;

pub fn delete_task() {
  let index_to_delete = utils::get_a_number(String::from("Enter the index you want to delete (you can only delete done tasks)"));

  let data = utils::get_tasks_data().expect("No tasks file.");
  let mut new_data = String::new();

  let mut current_done_index = 1;

  let lines = data.split("\n");

  for line in lines {
    let mut category: Option<String> = None;
    let mut task: Option<String> = None;

    let mut line_part_index = 0;

    let line_parts = line.split("|");

    for part in line_parts {
      let cleaned_part = part.trim();
      let current_part_is_category = line_part_index == 0;
      
      if current_part_is_category && (cleaned_part.eq("TODO") || cleaned_part.eq("DONE")) {
        let _ = category.insert(String::from(cleaned_part));
      } else if !current_part_is_category && !cleaned_part.eq("") {
        let _ = task.insert(String::from(cleaned_part));
      }

      line_part_index += 1;
    }

    if !task.is_none() && !category.is_none() {
      let unwrapped_category = category.unwrap();

      let is_done = unwrapped_category.eq("DONE");
      
      if !is_done || current_done_index != index_to_delete {
        new_data.push_str(unwrapped_category.as_str());
        new_data.push('|');
        new_data.push_str(task.unwrap().as_str());
        new_data.push('\n');
      }

      if is_done {
        current_done_index += 1;
      }
    }
  }

  fs::write("./src/tasks.txt", new_data.trim()).expect("Error while writing file");
}
