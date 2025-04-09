mod handlers;
mod utils;

use warp::Filter;

#[tokio::main]
async fn main() {
    let port = 3005;

    println!("[Starting] v0.0.1-alpha (:{})",port);

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

    println!("[Running process]");

    let routes = hello_route
        .or(list_route)
        .or(print_route)
        .or(upload_route);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}