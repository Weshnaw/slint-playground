fn main() {
    println!("cargo:rerun-if-changed=ui");
    slint_build::compile("ui/windows.slint").expect("Slint build failed");
}