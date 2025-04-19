use crate::{
    log_and_print,
    utils::{bad_request, generate_random_pdf_name, ok},
};
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

pub async fn print_request(
    form: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    log_and_print("Received upload request");

    use dirs;
    use futures::TryStreamExt;
    use std::{
        fs::File,
        io::Write,
        sync::{Arc, Mutex},
    };
    use warp::Buf;

    let app_data_dir = dirs::data_local_dir().unwrap().join("EcomenuPrinter");
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

    let print_cmd_command = if cfg!(target_os = "windows") {
        "./SumatraPDF-3.4.6-32.exe"
    } else {
        "gs"
    };

    let argumentos = [
        "-print-to",
        final_printer_name.as_ref().unwrap(),
        final_pdf_name.as_ref().unwrap(),
    ];

    let output = std::process::Command::new(print_cmd_command).args(argumentos).output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if out.status.success() && !stderr.contains("Error") && !stderr.contains("invalid") {
                ok("Printed successfully")
            } else {
                log_and_print(&format!(
                    "[Error] print failed:\nSTDOUT: {}\nSTDERR: {}",
                    stdout, stderr
                ));
                bad_request(&format!(
                    "print error:\nSTDOUT: {}\nSTDERR: {}",
                    stdout, stderr
                ))
            }
        }
        Err(e) => bad_request(&format!("Failed to execute command: {}", e)),
    }
}
