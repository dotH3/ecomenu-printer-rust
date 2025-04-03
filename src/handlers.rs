pub fn get_printer_list() -> String {
    format!("{:?}", printers::get_printers())
}

pub fn hello(param: String, agent: String) -> String {
    println!("Received request from {} with param {}", agent, param);
    format!("Hi{}", param)
}

pub fn print_request(body: serde_json::Value) -> String {
    format!("Received POST request with body: {}", body)
}
