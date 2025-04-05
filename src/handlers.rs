use serde_json::json;
use warp::reply::Json;

pub fn get_printer_list() -> Json {
    let list = printers::get_printers();
    let js_array: Vec<serde_json::Value> = list.iter().map(|p| {
        json!({
            "name": p.name,
            "system_name": p.system_name,
            "driver_name": p.driver_name,
            "uri": p.uri,
            "port_name": p.port_name
        })
    }).collect();
    warp::reply::json(&js_array)
}

pub fn hello(param: String, agent: String) -> String {
    println!("Received request from {} with param {}", agent, param);
    format!("Hi, {}!", param)
}

pub fn print_request(body: serde_json::Value) -> String {
    println!("Received POST request with body: {:?}", body);

    let output = std::process::Command::new("gswin64c")
        .args([
            "-dBATCH", "-dNOPAUSE",
            "-sDEVICE=mswinpr2",
            "-sPAPERSIZE=custom",
            "-dFIXEDMEDIA",
            "-dDEVICEWIDTHPOINTS=165",
            "-dDEVICEHEIGHTPOINTS=600",
            "-sOutputFile=%printer%POS-58",
            "-dFitPage",
            "ticket.pdf",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => println!("Printed successfully."),
        Ok(out) => println!("Error: {:?}", String::from_utf8_lossy(&out.stderr)),
        Err(e) => println!("Failed to execute command: {}", e),
    }

    format!("Received POST request with body: {}", body)
}


// pub fn print_request(body: serde_json::Value) -> String {
//     println!("Received POST request with body: {:?}", body);
    
//     let printer = printers::get_default_printer();

//     if printer.is_some() {
//         let printer = printer.unwrap();
//         println!("Printer: {:?}", printer);
//         // printer.print("Hola Martina".as_bytes(), None).unwrap();
//         match printer.print_file("a.pdf", Some("pdff")) {
//             Ok(_) => println!("File printed successfully."),
//             Err(e) => println!("Failed to print file: {}", e),
//         }
//     } else {
//         println!("No default printer found");
//     }
//     format!("Received POST request with body: {}", body)


// }