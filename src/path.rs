

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    unsafe extern "C++" {
        include!("path.h");
        // Treat StandardLocation as int
        pub fn writableLocation() -> QString;

        // #[Self = "QStandardPaths"]
        // pub fn writableLocation(location: i32) -> QString;
    }
}

pub fn path() -> cxx_qt_lib::QString {
    ffi::writableLocation()
}

// https://doc.qt.io/qt-6/qstandardpaths.html#StandardLocation-enum