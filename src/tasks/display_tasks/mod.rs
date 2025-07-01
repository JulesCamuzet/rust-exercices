use crate::utils::{self, get_tasks_data};

pub fn display_tasks() {
  let get_tasks_result = utils::get_tasks_data();

  if get_tasks_result.is_none() {
    println!("No tasks file created yet.");
    return
  }

  let data = get_tasks_data().expect("No tasks file.");

  let mut todos_output = String::from("Todos :\n");
  let mut done_output = String::from("Done :\n");

  let lines = data.split("\n");

  let mut todo_index = 1;
  let mut done_index = 1;

  for line in lines {
    let line_parts = line.trim().split("|");
    let mut category: Option<String> = None;
    let mut task: Option<String> = None;

    let mut line_part_index = 0;

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

    if !category.is_none() && !task.is_none() {
      match category.unwrap().as_str() {
        "TODO" => {
          todos_output.push_str(todo_index.to_string().as_str());
          todos_output.push_str(" - ");
          todos_output.push_str(task.unwrap().as_str());
          todos_output.push('\n');
          todo_index += 1;
        },
        "DONE" => {
          done_output.push_str(done_index.to_string().as_str());
          done_output.push_str(" - ");
          done_output.push_str(task.unwrap().as_str());
          done_output.push('\n');
          done_index += 1;
        },
        _ => break
      }
    }
  }

  println!("{}\n{}", todos_output, done_output);
}
