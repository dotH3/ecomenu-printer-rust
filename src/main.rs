mod handlers;
mod utils;

use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    net::TcpListener,
    time::{SystemTime, UNIX_EPOCH},
};

use warp::http::Method;
use warp::Filter;

const VERSION: &str = "v0.0.4-alpha";

fn log_and_print(message: &str) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let seconds = since_the_epoch.as_secs();
    let days = seconds / 86400;
    let remaining_seconds = seconds % 86400;
    let hours = remaining_seconds / 3600;
    let minutes = (remaining_seconds % 3600) / 60;
    let seconds = remaining_seconds % 60;
    let human_readable_date = format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        1970 + days / 365,
        (days % 365) / 30 + 1,
        days % 30 + 1,
        hours,
        minutes,
        seconds
    );

    println!("{}", message);

    let log_path = dirs::data_local_dir()
        .unwrap()
        .join("EcomenuPrinter")
        .join("log.txt");

    create_dir_all(log_path.parent().unwrap()).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Unable to open file");

    writeln!(file, "[{}] {}", human_readable_date, message).expect("Unable to write to file");
}

#[tokio::main]
async fn main() {
    let port = 3005;

    if TcpListener::bind(("127.0.0.1", port)).is_err() {
        log_and_print(&format!("[Error] Port {} is already in use.", port));
        return;
    }

    log_and_print(&format!("[Starting!3] {} (:{})", VERSION, port));

    let hello_route = warp::path!("hello" / String)
        .and(warp::header::<String>("user-agent"))
        .map(|name: String, user_agent: String| {
            log_and_print(&format!("[Request] /hello/{} from {}", name, user_agent));
            handlers::hello(name, user_agent)
        });

    let list_route = warp::path("printer-list").map(|| {
        log_and_print("[Request] /printer-list");
        handlers::get_printer_list()
    });

    let print_route = warp::path("print")
        .and(warp::post())
        .and(warp::multipart::form())
        .and_then(|form| {
            log_and_print("[Request] /print");
            handlers::print_request(form)
        });

    let print_options = warp::path("print").and(warp::options()).map(|| {
        log_and_print("[Request] OPTIONS /print");
        warp::reply()
    });

    log_and_print("[Running process]");

    let cors = warp::cors()
        .allow_origin("https://test.ecomenuapp.com")
        .allow_methods(&[Method::POST, Method::OPTIONS])
        .allow_headers(vec!["content-type", "token"]);

    let routes = hello_route.or(list_route).or(print_route).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
