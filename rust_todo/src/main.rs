use std::io::{self, Write};

enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

struct Task {
    status: TaskStatus,
    title: String,
}

fn display_menu(task_list: &Vec<Task>) {
    let mut i: u32 = 0;

    println!("\n------- ToDo list -------\n");
    for task in task_list {
        print!("{} ", i);
        match task.status {
            TaskStatus::ToDo => println!("[ ] {}", task.title),
            TaskStatus::InProgress => println!("[*] {}", task.title),
            TaskStatus::Done => println!("[x] {}", task.title),
        }
        i += 1;
    }

    println!("\nChoice :\n- 1 : Add a note.\n- 2 : Update a note.\n- 3 : Delete a note.\n");
    println!("------------------------");
}

fn add_task(task_list: &mut Vec<Task>) {
    let mut input = String::new();

    print!("Title of the task : ");
    io::stdout().flush().expect("[-] Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("[-] Error getting title for new task");

    task_list.push(Task {
        status: TaskStatus::ToDo,
        title: input.trim().to_string(),
    });
}

fn update_task(task_list: &mut Vec<Task>) {
    let mut input = String::new();
    let task_choice: usize;
    let status_update: u32;

    print!("Task to update : ");
    io::stdout().flush().expect("[-] Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("[-] Error getting input for update task");

    task_choice = input.trim().parse().unwrap();

    println!("- 1 : To Do\n- 2 : In Progress\n- 3 : Done\n");
    input.clear();
    io::stdin().read_line(&mut input).expect("[-] Error getting input for update task");

    status_update = input.trim().parse().unwrap();

    if task_choice <= task_list.len() {
        match status_update {
            1 => task_list[task_choice].status = TaskStatus::ToDo,
            2 => task_list[task_choice].status = TaskStatus::InProgress,
            3 => task_list[task_choice].status = TaskStatus::Done,
            _ => { println!("Unknow status."); return; }
        };
    } else {
        println!("Task {} doesn\'t exist", task_choice);
    }
}

fn delete_task(task_list: &mut Vec<Task>) {
    let mut input = String::new();
    let task_choice: usize;

    print!("Task to delete : ");
    io::stdout().flush().expect("[-] Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("[-] Error getting input for delete task");

    task_choice = input.trim().parse().unwrap();

    if task_choice < task_list.len() {
        task_list.remove(task_choice);
    } else {
        println!("Task {} doesn\'t exist", task_choice);
    }
}

fn main() {
    let mut user_input = String::new();
    let mut task_list: Vec<Task> = Vec::new();

    while user_input.trim() != "exit" {
        display_menu(&task_list);

        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("[-] Error recieving user input");

        match user_input.trim() {
            "1" => add_task(&mut task_list),
            "2" => update_task(&mut task_list),
            "3" => delete_task(&mut task_list),
            "exit" => return,
            _ => println!("Error on input"),
        };
    }
}
