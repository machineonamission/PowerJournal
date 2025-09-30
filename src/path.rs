use std::path;
use std::fs;
use anyhow::Result;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    unsafe extern "C++" {
        include!("path.h");
        pub fn writableLocation() -> QString;

        // try as i might, you cant pass static class enums through cxx, so we have to use a wrapper file
        // #[Self = "QStandardPaths"]
        // pub fn writableLocation(location: i32) -> QString;
    }
}

pub fn data_dir() -> Result<path::PathBuf> {
    let qpath = ffi::writableLocation();
    let s = qpath.to_string();
    if s.is_empty() {
        return Err(anyhow::anyhow!("Qt returned empty data directory"));
    }
    let p = path::PathBuf::from(s);
    fs::create_dir_all(&p)?;
    Ok(p)
}

// https://doc.qt.io/qt-6/qstandardpaths.html#StandardLocation-enum