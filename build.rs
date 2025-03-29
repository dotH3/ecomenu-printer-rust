extern crate winres;

fn main() {
  println!("cargo:rerun-if-changed=icon.ico");
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().unwrap();
  }
}