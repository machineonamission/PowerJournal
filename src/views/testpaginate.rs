use crate::components::paginate::Paginate;
use crate::database::entity::prelude::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use sea_orm::QuerySelect;
use sea_orm::{
    DatabaseConnection, EntityLoaderTrait, EntityTrait, Iden, PaginatorTrait, QueryOrder,
};

#[component]
pub fn TestPaginate() -> Element {
    rsx! {
        // Paginate {}
    }
}
