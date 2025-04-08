use printers::{get_printer_by_name,get_printers};
use serde_json::json;
use warp::reply::Json;

pub fn get_printer_list() -> Json {
    let list = get_printers();
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
    println!("Received request with body: {:?}", body);

    let body_printer_name = body.get("printerName");
    let _body_pdf = body.get("pdf");
    
    if body_printer_name.is_none() {
        return bad_request("Missing 'printerName'");
    }

    let printer = get_printer_by_name(body_printer_name.unwrap().as_str().unwrap());

    if printer.is_none() {
        return bad_request("Printer not found");
    }

    let gs_cmd = if cfg!(target_os = "windows") {
        "gswin64c"
    } else {
        "gs"
    };

    println!("-sOutputFile=%printer%{}", printer.unwrap().system_name);

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

pub async fn upload_file(form: warp::multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
    use futures::TryStreamExt;
    use warp::Buf;

    form.try_for_each(|mut part| async move {
        let name = part.name().to_string();
        let filename = part.filename().unwrap_or("-").to_string();
        let mut size = 0;
    
        while let Some(Ok(chunk)) = part.data().await {
            size += chunk.remaining();
        }
    
        println!("Field: {name}, Filename: {filename}, Bytes: {size}");
        Ok(())
    }).await.ok();
    

    ok("OK")
}




//? Funciones internas

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