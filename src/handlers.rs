use serde_json::json;
use warp::reply::Json;

pub fn get_printer_list() -> Json {
    let list = printers::get_printers();
    let js_array: Vec<serde_json::Value> = list
        .iter()
        .map(|p| {
            json!({
                "name": p.name,
                "system_name": p.system_name,
                "driver_name": p.driver_name,
                "uri": p.uri,
                "port_name": p.port_name
            })
        })
        .collect();
    warp::reply::json(&js_array)
}

pub fn hello(param: String, agent: String) -> String {
    println!("Received request from {} with param {}", agent, param);
    format!("Hi, {}!", param)
}

pub async fn print_request(body: serde_json::Value) -> Result<impl warp::Reply, warp::Rejection> {
    if body.get("printerName").is_none() {
        return bad_request("Missing 'printerName'");
    }

    println!("Received request with body: {:?}", body);


    let gs_cmd = if cfg!(target_os = "windows") {
        "gswin64c"
    } else {
        "gs"
    };

    let output = std::process::Command::new(gs_cmd)
        .args([
            "-dBATCH",
            "-dNOPAUSE",
            "-sDEVICE=mswinpr2",
            "-sPAPERSIZE=custom",
            "-dFIXEDMEDIA",
            "-dDEVICEWIDTHPOINTS=165",
            "-dDEVICEHEIGHTPOINTS=600",
            "-sOutputFile=%printer%POS-58",
            "-dFitPage",
            "ec.pdf",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => println!("Printed successfully."),
        Ok(out) => println!("Error: {:?}", String::from_utf8_lossy(&out.stderr)),
        Err(e) => println!("Failed to execute command: {}", e),
    }

    ok("OK")
}

//? Ruta
pub async fn example_handler(body: serde_json::Value) -> Result<impl warp::Reply, warp::Rejection> {
    if body.get("someField").is_none() {
        return bad_request("Missing someField");
    }

    ok("OK")
}

//? Funciones de respuesta
fn bad_request(msg: &str) -> Result<warp::reply::WithStatus<String>, warp::Rejection> {
    Ok(warp::reply::with_status(msg.to_string(), warp::http::StatusCode::BAD_REQUEST))
}
fn ok(msg: &str) -> Result<warp::reply::WithStatus<String>, warp::Rejection> {
    Ok(warp::reply::with_status(msg.to_string(), warp::http::StatusCode::OK))
}