fn main() {
    println!("cargo:rerun-if-changed=ui");
    slint_build::compile("ui/android.slint").expect("Slint build failed");
}