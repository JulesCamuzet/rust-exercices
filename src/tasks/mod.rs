use crate::utils;

mod display_tasks;
mod mark_as_done;
mod mark_as_undone;

pub fn tasks_manager() {
  let mut keep_going = true;

  while keep_going {
    let choice: i32 = utils::get_a_number(String::from("What do you want to do : \n1 : Display your tasks\n2 : Mark a task as done\n3 : Mark a task as undone\n4 : Delete a task\n5 : Update a task"));
  
    let valid_choices = 1..5;
    if !valid_choices.contains(&choice) {
      println!("Wrong choice.");
      return;
    }
  
    match choice {
      1 => display_tasks::display_tasks(),
      2 => mark_as_done::mark_as_done(),
      3 => mark_as_undone::mark_as_undone(),
      4 => println!("Coming soon"),
      5 => println!("Coming soon"),
      6 => keep_going = false,
      _ => return
    }
  }
}
