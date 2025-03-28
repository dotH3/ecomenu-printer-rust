use warp::Filter;
mod get_list;

#[tokio::main]
async fn main() {
    let hi = warp::path!("hello" / String)
        .and(warp::header::<String>("user-agent"))
        .map(|param: String, agent: String| {
            let list = get_list::get_list();
            format!("Hello {}. {} Lista: {:?}", param, agent, list)
        });

    println!("[Running]");

    warp::serve(hi)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
