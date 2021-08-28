use rocket::serde::Serialize;
use rocket::serde::{json::json, json::Json, json::Value};
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use std::io;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world !"
}

#[get("/todo")]
fn todo() -> Json<Task> {
    Json(Task {
        name: "ruo".to_string(),
    })
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;

    format!("Waited for {} seconds !", seconds)
}

#[get("/")]
async fn task() -> io::Result<Vec<u8>> {
    // the file that is _data.proygon need create by you !
    let vec = spawn_blocking(|| std::fs::read("_data.proygon"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[catch(404)]
fn route_not_found() -> Value {
    json!({
        "status": "system_error_monitor",
        "info": {
            "message": "the route is not found !"
        }
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index, delay, todo])
        .mount("/api/task", routes![task])
        .register("/api", catchers![route_not_found])
}
