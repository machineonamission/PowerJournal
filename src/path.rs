
#[cxx_qt::bridge]
mod ffi {
    // unsafe extern "C++" {
    //     include!("cxx-qt-lib/qstring.h");
    //     type QString = cxx_qt_lib::QString;
    // }
    unsafe extern "C++" {
        include!("path.h");
        // Treat StandardLocation as int
        pub fn writableLocation(loc: u8) -> String;

        // #[Self = "QStandardPaths"]
        // pub fn writableLocation(location: i32) -> QString;
    }
}

pub fn path() -> String {
    ffi::writableLocation(1)
}

// https://doc.qt.io/qt-6/qstandardpaths.html#StandardLocation-enum