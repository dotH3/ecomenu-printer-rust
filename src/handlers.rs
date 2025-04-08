use crate::utils::{bad_request, generate_random_pdf_name, ok};
use printers::{get_printer_by_name, get_printers};
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

pub async fn upload_file(
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Received upload request");
    use futures::TryStreamExt;
    use std::{
        fs::File,
        io::Write,
        sync::{Arc, Mutex},
    };
    use warp::Buf;

    let pdf_name = Arc::new(Mutex::new(None));
    let printer_name = Arc::new(Mutex::new(None));
    let pdf_name_clone = pdf_name.clone();
    let printer_name_clone = printer_name.clone();

    form.try_for_each(move |mut part| {
        let pdf_name_clone = pdf_name_clone.clone();
        let printer_name_clone = printer_name_clone.clone();
        async move {
            let name = part.name().to_string();

            if name == "pdf" {
                let filename = generate_random_pdf_name();
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = part.data().await {
                    data.extend_from_slice(chunk.chunk());
                }

                let final_name = format!("{}.pdf", filename);
                if let Ok(mut file) = File::create(&final_name) {
                    let _ = file.write_all(&data);
                }

                *pdf_name_clone.lock().unwrap() = Some(final_name.clone());
            }

            if name == "printer_name" {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = part.data().await {
                    data.extend_from_slice(chunk.chunk());
                }

                let value = String::from_utf8_lossy(&data).to_string();
                *printer_name_clone.lock().unwrap() = Some(value);
            }

            Ok(())
        }
    })
    .await
    .ok();

    println!("!");
    println!("pdf name {pdf_name}");

    let final_pdf_name = pdf_name.lock().unwrap().clone();
    let final_printer_name = printer_name.lock().unwrap().clone();


    if final_printer_name.is_none() {
        return bad_request("Missing 'printer_name'");
    }

    let printer = get_printer_by_name(final_printer_name.as_ref().unwrap());

    if printer.is_none() {
        return bad_request("Printer not found");
    }


    println!("Pdf Guardado: {:?}", final_pdf_name);
    println!("Printer Name: {:?}", final_printer_name);

    //? A este punto del codigo, ya deberiamos de tener el nombre del archivo e impresora
    //? Lo que significa que podemos realizar la impresion

    println!("Iniciando la impresion...");

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
            final_pdf_name.as_ref().unwrap()
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => println!("Printed successfully."),
        Ok(out) => println!("Error: {:?}", String::from_utf8_lossy(&out.stderr)),
        Err(e) => println!("Failed to execute command: {}", e),
    }

    ok("OK")
}
