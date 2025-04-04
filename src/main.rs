mod handlers;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = 3005;

    let hello_route = warp::path!("hello" / String)
        .and(warp::header::<String>("user-agent"))
        .map(handlers::hello);

    let list_route = warp::path("printer-list").map(handlers::get_printer_list);

    let print_route = warp::path("print")
        .and(warp::post())
        .and(warp::body::json())
        .map(handlers::print_request);

    println!("[Running process]");

    let routes = hello_route.or(list_route).or(print_route);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
