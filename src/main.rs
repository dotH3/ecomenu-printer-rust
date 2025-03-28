use warp::Filter;
mod get_list;

#[tokio::main]
async fn main() {
    let hello_route = warp::path!("hello" / String)
        .and(warp::header::<String>("user-agent"))
        .map(|param: String, agent: String| {
            let list = get_list::get_list();
            format!("Hello {}. {} Lista: {:?}", param, agent, list)
        });

    let list_route = warp::path("list").map(|| {
        let list = get_list::get_list();
        format!("{:?}", list)
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
    .run(([127, 0, 0, 1], 3030))
    .await;
}
