mod handlers;
mod utils;

use warp::Filter;
use std::net::TcpListener;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn log_and_print(message: &str) {
    // Get the current time
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Convert to a human-readable format (YYYY-MM-DD HH:MM:SS)
    let seconds = since_the_epoch.as_secs();
    let days = seconds / 86400;
    let remaining_seconds = seconds % 86400;
    let hours = remaining_seconds / 3600;
    let minutes = (remaining_seconds % 3600) / 60;
    let seconds = remaining_seconds % 60;
    let human_readable_date = format!("{}-{}-{} {}:{}:{}", 1970 + days / 365, (days % 365) / 30 + 1, days % 30 + 1, hours, minutes, seconds);

    // Print the message
    println!("{}", message);

    // Append the message and date to a log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Unable to open file");

    writeln!(file, "[{}] {}", human_readable_date, message).expect("Unable to write to file");
}

#[tokio::main]
async fn main() {
    let port = 3005;

    // Check if the port is available
    if TcpListener::bind(("127.0.0.1", port)).is_err() {
        log_and_print(&format!("[Error] Port {} is already in use.", port));
        return;
    }

    log_and_print(&format!("[Starting] v0.0.1-alpha (:{})", port));

    let hello_route = warp::path!("hello" / String)
        .and(warp::header::<String>("user-agent"))
        .map(handlers::hello);

    let list_route = warp::path("printer-list").map(handlers::get_printer_list);

    let print_route = warp::path("print")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::print_request);
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form())
        .and_then(handlers::upload_file);

    log_and_print(&format!("[Running process]"));

    let routes = hello_route
        .or(list_route)
        .or(print_route)
        .or(upload_route);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}