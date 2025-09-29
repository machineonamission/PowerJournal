use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
        // - Qt Qml requires linking Qt Network on macOS
        // .crate_include_root(Some("include/".to_owned()))
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "me.machineonamission.powerjournal",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["qml/main.qml", "qml/Entry.qml", "qml/Entries.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            cc.file("src/path.cpp").include("include");
        })
        .build();
}