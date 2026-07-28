
fn main() {
    // Install required packages

    // Compile TailwindCSS .css file
    std::process::Command::new("deno")
        .args([
            "run",
            "-A",
            "npm:@tailwindcss/cli",
            "-i",
            "src/tailwindinput.css",
            "-o",
            "assets/tailwind.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}
