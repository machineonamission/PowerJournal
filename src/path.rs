#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("QStandardPaths");
        pub type QStandardPaths;
    }
}

// https://doc.qt.io/qt-6/qstandardpaths.html#StandardLocation-enum