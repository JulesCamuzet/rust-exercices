use crate::utils;
use std::fs;

pub fn mark_as_done() {
  let index_to_mark = utils::get_a_number(String::from("Enter the task index that you want to mark as done :"));

  let get_tasks_result = utils::get_tasks_data();

  if get_tasks_result.is_none() {
    println!("No tasks file created yet.");
    return
  }

  let data = get_tasks_result.unwrap();
  let mut new_data = String::new();

  let mut task_to_mark = String::new();

  let global_split = data.split('|');
  let mut global_index = 0;

  for part in global_split {
    let we_are_in_todos = global_index == 0;
    let tasks_split = part.split('\n');

    if we_are_in_todos {
      let mut task_index = 0;

      for task in tasks_split {
        task_index += 1;

        if task_index != index_to_mark {
          let mut line_to_push = task.to_string();
          line_to_push.push_str("\n");
          new_data.push_str(line_to_push.as_str());
        } else {
          task_to_mark = task.to_string();
        }

        task_index += 1;
      }

      new_data.push_str("|\n");
    } else {
      for task in tasks_split {
        let mut line_to_push = task.to_string();
        // line_to_push.push_str("\n");
        new_data.push_str(line_to_push.as_str());
      }

      let mut line_to_push = task_to_mark.to_string();
      line_to_push.push_str("\n");
      new_data.push_str(line_to_push.as_str());
    }

    global_index += 1;
  }

  fs::write("./src/tasks.txt", new_data).expect("Error while writing file.");
}