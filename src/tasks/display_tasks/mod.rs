use crate::utils;

pub fn display_tasks() {
  let get_tasks_result = utils::get_tasks_data();

  if get_tasks_result.is_none() {
    println!("No tasks file created yet.");
    return
  }

  let data = get_tasks_result.unwrap();

  let global_split = str::split(&data, "|");
  let mut index = 0;

  for part in global_split {
    if index == 0 {
      println!("TO DO :")
    } else if index == 1 {
      println!("\nDONE :")
    }

    let tasks_split = part.trim().split('\n');

    let mut j = 0;
    for task in tasks_split {
      if task.trim() != "" {
        j += 1;
        println!("{}-{}", j, task.trim());
      }
    }

    index += 1;
  }
}
