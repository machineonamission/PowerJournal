use sea_orm::QuerySelect;
use crate::database::entity::prelude::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use sea_orm::{
    DatabaseConnection, EntityLoaderTrait, EntityTrait, Iden, PaginatorTrait, QueryOrder,
};
use crate::components::entry::Entry;

#[component]
pub fn Journal(id: i32) -> Element {
    let db_signal = use_context::<Signal<Option<DatabaseConnection>>>();
    // Fetch data asynchronously when the database becomes available
    let entries: Resource<Vec<entries::ModelEx>> = use_resource(move || async move {
        if let Some(db) = db_signal.read().as_ref() {
            let mut cursor = entries::Entity::load()
                .with((piece::Entity, piece_0_text::Entity))
                .with((piece::Entity, piece_1_mood::Entity))
                .with((piece::Entity, piece_2_blob::Entity))
                .with((piece::Entity, piece_3_location::Entity))
                .with((piece::Entity, piece_4_activities::Entity))
                .order_by_desc(entries::Column::Datetime)
                .paginate(db, 10);
            // .paginate(db, 50);
            // note: needs to be changed to WHILE let to properly paginate, but we debug
            if let Some(users) = cursor.fetch_and_next().await.unwrap() {
                return users
                // for user in users {
                //     dbg!(&user);
                // }
            }
            vec![]
        } else {
            vec![]
        }
    });

    rsx! {
        ul {
            // this for loop will be replaced with a real paginator once i INVENT IT
            for entry in entries.read().as_ref().unwrap_or(&vec![]) {
                Entry { entry: entry.clone() }
            }
        }
    }
}
