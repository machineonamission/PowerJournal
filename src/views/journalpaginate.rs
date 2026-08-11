use crate::components::entry::Entry;
use crate::components::paginate::Paginate;
use crate::database::entity::prelude::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::{
    DatabaseConnection, EntityLoaderTrait, EntityTrait, Iden, PaginatorTrait, QueryOrder,
};

#[component]
fn JournalPaginateBase(id: Option<i64>) -> Element {
    let cursor = use_signal(|| {
        let mut c = entries::Entity::load();
        if let Some(id) = id {
            c = c.filter(entries::Column::JournalId.eq(id))
        }
        c.with((piece::Entity, piece_0_text::Entity))
            .with((piece::Entity, piece_1_mood::Entity))
            .with((piece::Entity, piece_2_blob::Entity))
            .with((piece::Entity, piece_3_location::Entity))
            .with((piece::Entity, piece_4_activities::Entity))
            .order_by_desc(entries::Column::Datetime)
    });

    rsx! {
        Paginate {
            loader: cursor,
            render: |model| rsx! { Entry { entry: model } }
        }
    }
}

#[component]
pub fn JournalPaginate(id: i64) -> Element {
    rsx! {
        JournalPaginateBase { id: Some(id)}
    }
}

#[component]
pub fn JournalPaginateAll() -> Element {
    rsx! {
        JournalPaginateBase { id: None}
    }
}
