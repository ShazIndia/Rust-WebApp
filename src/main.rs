#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task, read_tasks])
}

use std::{fs::{OpenOptions}, io::{Write}};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::io::{BufRead, BufReader};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    id: u8,
    item: &'r str
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");   
    let reader = BufReader::new(&tasks);
    let id = reader.lines().count();
    let task_item_string = format!("{},{}\n", id, task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("unable to write to tasks.txt");
    "Task added succesfully"
}


#[get("/readtasks")]
fn read_tasks() -> Json<Vec<String>> {
    let tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");  
    let reader = BufReader::new(tasks);
    Json(reader.lines()
            .map(|line| {
                let line_string: String = line.expect("could not read line");
                let line_pieces: Vec<&str> = line_string.split(",").collect();
                line_pieces[1].to_string()
            })
            .collect())
}

