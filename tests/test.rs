use rocket::local::blocking::Client;
use rocket::http::Status;
use std::io::{Write, BufRead, BufReader};
use std::fs::{File, remove_file};

#[test]
fn test_index() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}

#[test]
fn test_add_task_and_read_tasks() {
    // Setup: Create a temporary tasks.txt file
    let mut tasks_file = File::create("tasks.txt").expect("create tasks.txt");
    write!(tasks_file, "0,task1\n").expect("write to tasks.txt");
    let rocket_instance = rocket();

    // Test adding a task
    let client = Client::tracked(rocket_instance.clone()).expect("valid rocket instance");
    let response = client
        .post("/addtask")
        .header("Content-Type", "application/json")
        .body(r#"{ "id": 1, "item": "task2" }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Task added succesfully");

    // Test reading tasks
    let response = client.get("/readtasks").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body_str = response.into_string().unwrap();
    assert_eq!(body_str, r#"["task1", "task2"]"#);

    // Tear down: Remove the temporary tasks.txt file
    remove_file("tasks.txt").expect("remove tasks.txt");
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, add_task, read_tasks])
}

// Define your routes and handlers here
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    id: u8,
    item: &'r str
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    // Implementation of add_task function
    "Task added succesfully"
}

#[get("/readtasks")]
fn read_tasks() -> Json<Vec<String>> {
    // Implementation of read_tasks function
    Json(vec![])
}
