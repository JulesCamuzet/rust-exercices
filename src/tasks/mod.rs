use crate::utils;

mod display_tasks;
mod mark_as_done;
mod mark_as_undone;
mod add_task;
mod delete_task;

pub fn tasks_manager() {
  let mut keep_going = true;

  while keep_going {
    let choice: i32 = utils::get_a_number(String::from("What do you want to do : \n1 : Display your tasks\n2 : Mark a task as done\n3 : Mark a task as undone\n4 : Add a task\n5 : Delete a task\n6 : Exit"));
  
    let valid_choices = 1..7;
    if !valid_choices.contains(&choice) {
      println!("Wrong choice.");
      return;
    }
  
    match choice {
      1 => display_tasks::display_tasks(),
      2 => mark_as_done::mark_as_done(),
      3 => mark_as_undone::mark_as_undone(),
      4 => add_task::add_task(),
      5 => delete_task::delete_task(),
      6 => keep_going = false,
      _ => return
    }
  }
}
