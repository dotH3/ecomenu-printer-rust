use crate::{log_and_print, utils::{bad_request, generate_random_pdf_name, ok}};
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

pub fn hello(param: String, _agent: String) -> String {
    format!("Hi, {}!", param)
}

// pub async fn print_request(body: serde_json::Value) -> Result<impl warp::Reply, warp::Rejection> {
//     let body_printer_name = body.get("printerName");
//     let _body_pdf = body.get("pdf");

//     if body_printer_name.is_none() {
//         return bad_request("Missing 'printerName'");
//     }

//     let printer = get_printer_by_name(body_printer_name.unwrap().as_str().unwrap());

//     if printer.is_none() {
//         return bad_request("Printer not found");
//     }

//     let gs_cmd = if cfg!(target_os = "windows") {
//         "gswin64c"
//     } else {
//         "gs"
//     };

//     log_and_print(&format!("-sOutputFile=%printer%{}", printer.unwrap().system_name));

//     let output = std::process::Command::new(gs_cmd)
//         .args([
//             "-dBATCH",
//             "-dNOPAUSE",
//             "-sDEVICE=mswinpr2",
//             "-sPAPERSIZE=custom",
//             "-dFIXEDMEDIA",
//             "-dDEVICEWIDTHPOINTS=165",
//             "-dDEVICEHEIGHTPOINTS=600",
//             "-sOutputFile=%printer%POS-58",
//             "-dFitPage",
//             "ec.pdf",
//         ])
//         .output();

//     match output {
//         Ok(out) if out.status.success() => log_and_print("Printed successfully."),
//         Ok(out) => log_and_print(&format!("Error: {:?}", String::from_utf8_lossy(&out.stderr))),
//         Err(e) => log_and_print(&format!("Failed to execute command: {}", e)),
//     }

//     ok("OK")
// }

pub async fn print_request(
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    log_and_print("Received upload request");

    use futures::TryStreamExt;
    use std::{
        fs::File,
        io::Write,
        sync::{Arc, Mutex},
    };
    use warp::Buf;
    use dirs;

    let app_data_dir = dirs::data_local_dir()
        .unwrap()
        .join("EcomenuPrinter");
    std::fs::create_dir_all(&app_data_dir).unwrap();

    let pdf_name = Arc::new(Mutex::new(None));
    let printer_name = Arc::new(Mutex::new(None));
    let pdf_name_clone = pdf_name.clone();
    let printer_name_clone = printer_name.clone();

    form.try_for_each(move |mut part| {
        let pdf_name_clone = pdf_name_clone.clone();
        let printer_name_clone = printer_name_clone.clone();
        let app_data_dir = app_data_dir.clone();
        async move {
            let name = part.name().to_string();

            if name == "pdf" {
                let filename = generate_random_pdf_name();
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = part.data().await {
                    data.extend_from_slice(chunk.chunk());
                }

                let final_name = app_data_dir.join(format!("{}.pdf", filename));
                if let Ok(mut file) = File::create(&final_name) {
                    let _ = file.write_all(&data);
                }

                *pdf_name_clone.lock().unwrap() = Some(final_name.to_string_lossy().to_string());
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

    let final_pdf_name = pdf_name.lock().unwrap().clone();
    let final_printer_name = printer_name.lock().unwrap().clone();

    if final_pdf_name.is_none() {
        return bad_request("Missing 'pdf' file");
    }

    if final_printer_name.is_none() {
        return bad_request("Missing 'printer_name'");
    }

    let printer = get_printer_by_name(final_printer_name.as_ref().unwrap());

    if printer.is_none() {
        return bad_request("Printer not found");
    }

    //? A este punto del codigo, ya deberiamos de tener el nombre del archivo e impresora
    //? Lo que significa que podemos realizar la impresion


    let gs_cmd = if cfg!(target_os = "windows") {
        "gswin64c"
    } else {
        "gs"
    };

    // println!("4");
    let output = std::process::Command::new(gs_cmd)
    .args([
        "-dBATCH",
        "-dNOPAUSE",
        "-sDEVICE=mswinpr2",
        "-dFIXEDMEDIA",
        "-dPDFFitPage=false",
        "-dDEVICEHEIGHTPOINTS=3276",
        "-dDEVICEWIDTHPOINTS=165",
        format!("-sOutputFile=%printer%{}", final_printer_name.unwrap()).as_str(),
        final_pdf_name.as_ref().unwrap(),
    ])
    .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if out.status.success() && !stderr.contains("Error") && !stderr.contains("invalid") {
                let _ = std::fs::remove_file(final_pdf_name.unwrap());
                ok("Printed successfully")
            } else {
                log_and_print(&format!("[Error] Ghostscript failed:\nSTDOUT: {}\nSTDERR: {}", stdout, stderr));
                bad_request(&format!("Ghostscript error:\nSTDOUT: {}\nSTDERR: {}", stdout, stderr))
            }
        }
        Err(e) => bad_request(&format!("Failed to execute command: {}", e)),
    }
}
