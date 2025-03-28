
pub fn get_list() {
    let x = printers::get_printers();
    drop(x);
}
