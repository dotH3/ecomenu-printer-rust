use warp::Filter;

#[tokio::main]
async fn main() {

    let port = 3005;

    // Router
    let hello_route = warp::path!("hello" / String)
        .and(warp::header::<String>("user-agent"))
        .map(|param: String, agent: String| {
            println!("Received request from {} with param {}", agent, param);
            "Hi".to_string()+&param
        });

    let list_route = warp::path("printer-list").map(|| {
        format!("{:?}", printers::get_printers())
    });

    let print_route = warp::path("print")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            format!("Received POST request with body: {}", body)
        });

    println!("[Running process]");

    let routes = hello_route.or(list_route).or(print_route);
    warp::serve(routes)
    .run(([127, 0, 0, 1], port))
    .await;
}
