pub fn generate_random_pdf_name() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: u32 = rand::random();
    format!("pdf_{}_{}", now, rand)
}

pub fn bad_request(msg: &str) -> Result<warp::reply::WithStatus<String>, warp::Rejection> {
    Ok(warp::reply::with_status(
        msg.to_string(),
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

pub fn ok(msg: &str) -> Result<warp::reply::WithStatus<String>, warp::Rejection> {
    Ok(warp::reply::with_status(
        msg.to_string(),
        warp::http::StatusCode::OK,
    ))
}