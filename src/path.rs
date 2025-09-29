
#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    #[qenum]
    #[namespace = "QStandardPaths"]
    enum StandardLocation {
        DesktopLocation,
        DocumentsLocation,
        FontsLocation,
        ApplicationsLocation,
        MusicLocation,
        MoviesLocation,
        PicturesLocation,
        TempLocation,
        HomeLocation,
        AppLocalDataLocation,
        CacheLocation,
        GenericDataLocation,
        RuntimeLocation,
        ConfigLocation,
        DownloadLocation,
        GenericCacheLocation,
        GenericConfigLocation,
        AppDataLocation,
        AppConfigLocation,
        PublicShareLocation,
        TemplatesLocation,
        StateLocation,
        GenericStateLocation,
    }

    unsafe extern "C++Qt" {
        include!("<QStandardPaths>");
        #[qobject]
        type QStandardPaths;
        // pub fn writableLocation(location: StandardLocation) -> QString;
    }
}

// pub fn path() -> cxx_qt_lib::QString {
//     ffi::writableLocation(ffi::StandardLocation::AppDataLocation)
// }

// https://doc.qt.io/qt-6/qstandardpaths.html#StandardLocation-enum