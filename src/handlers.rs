pub fn get_printer_list() -> String {
    format!("{:?}", printers::get_printers())
}

pub fn hello(param: String, agent: String) -> String {
    println!("Received request from {} with param {}", agent, param);
    format!("Hi{}", param)
}

pub fn print_request(body: serde_json::Value) -> String {
    println!("Received POST request with body: {:?}", body);
    
    let printer = printers::get_default_printer();

    if printer.is_some() {
        let printer = printer.unwrap();
        println!("Printer: {:?}", printer);
        printer.print("Hola Martina".as_bytes(), None).unwrap();
    } else {
        println!("No default printer found");
    }
    format!("Received POST request with body: {}", body)


}
