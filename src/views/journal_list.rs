use crate::Route;
use crate::components::journal_list::JournalListEntry;
use crate::components::paginate::Paginate;
use crate::database::entity::{
    entries, journal, piece, piece_0_text, piece_1_mood, piece_2_blob, piece_3_location,
    piece_4_activities,
};
use dioxus::prelude::*;
use sea_orm::QueryOrder;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn JournalList() -> Element {
    let cursor = use_signal(|| journal::Entity::load());

    rsx! {
        Paginate {
            loader: cursor,
            render: |model| rsx! { JournalListEntry { journal: model } }
        }
    }
}
