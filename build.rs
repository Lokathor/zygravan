use std::path::Path;

fn main() {
  let out_dir = std::env::var("OUT_DIR").unwrap();
  let path_buf = Path::new(out_dir.as_str()).join("rt0.o");
  let out_name = format!("{}", path_buf.display());
  //
  let profile = std::env::var("PROFILE").unwrap();
  let debug = if profile == "debug" { vec!["-g"] } else { Vec::new() };
  //
  let assembler_output = std::process::Command::new("arm-none-eabi-as")
    .args(&["-o", out_name.as_str()])
    .args(&debug)
    .arg("-mthumb-interwork")
    .arg("-mcpu=arm7tdmi")
    .arg("src/rt0.S")
    .output()
    .expect("failed to run: arm-none-eabi-as");
  if !assembler_output.status.success() {
    panic!("\n{}", String::from_utf8_lossy(&assembler_output.stderr));
  }
  //
  println!("cargo:rustc-link-search={}", out_dir);
}
