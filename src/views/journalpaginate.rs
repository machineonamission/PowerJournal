use sea_orm::QuerySelect;
use crate::database::entity::prelude::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use sea_orm::{
    DatabaseConnection, EntityLoaderTrait, EntityTrait, Iden, PaginatorTrait, QueryOrder,
};
use crate::components::entry::Entry;
use crate::components::paginate::Paginate;

#[component]
pub fn JournalPaginate(id: i32) -> Element {
    let mut cursor = use_signal(|| entries::Entity::load()
        .with((piece::Entity, piece_0_text::Entity))
        .with((piece::Entity, piece_1_mood::Entity))
        // .with((piece::Entity, piece_2_blob::Entity))
        .with((piece::Entity, piece_3_location::Entity))
        .with((piece::Entity, piece_4_activities::Entity))
        .order_by_desc(entries::Column::Datetime));

    rsx! {
        Paginate {
            loader: cursor,
            render: |model| rsx! { Entry { entry: model } }
        }
    }
}
