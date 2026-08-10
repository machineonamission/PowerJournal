#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LogoSize {
    pub name: &'static str,
    pub size: usize,
}

fn resize_logo() {
    let IMPORTERS: [LogoSize; 1] = [LogoSize { name: "", size: 0 }];
}

fn main() {
    resize_logo()
}
