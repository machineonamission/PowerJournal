pub mod cxxqt_object;

mod database;
mod path;
mod models;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use rusqlite::Connection;
use std::sync::{Mutex, OnceLock};


fn main() {
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    let db = database::get_db();
    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/me/machineonamission/powerjournal/qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        // Listen to a signal from the QML Engine
        engine
            .as_qqmlengine()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}