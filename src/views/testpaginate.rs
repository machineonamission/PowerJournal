use sea_orm::QuerySelect;
use crate::database::entity::prelude::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use sea_orm::{
    DatabaseConnection, EntityLoaderTrait, EntityTrait, Iden, PaginatorTrait, QueryOrder,
};
use crate::components::paginate::Paginate;

#[component]
pub fn TestPaginate() -> Element {
    rsx! {
        // Paginate {}
    }
}
