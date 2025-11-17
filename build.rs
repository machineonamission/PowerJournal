fn main() {
    println!("cargo:rustc-env=SLINT_BACKEND=winit-skia");
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
