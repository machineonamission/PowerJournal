use bytes::Bytes;
use dioxus::prelude::Signal;
use sea_orm::DatabaseConnection;

pub struct ImporterArgs<'a> {
    pub file: Bytes,
    pub db: &'a DatabaseConnection,
    pub log_signal: Signal<Vec<String>>,
    pub current_prog_signal: Signal<i64>,
    pub max_prog_signal: Signal<i64>,
}