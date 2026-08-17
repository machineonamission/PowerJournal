use crate::database::entity::piece;
use dioxus::core::Element;
use dioxus::core_macro::component;
use dioxus::prelude::*;
use crate::components::editor::piece_0_text::Piece0TextEditor;
use crate::components::editor::piece_1_mood::Piece1MoodEditor;
use crate::components::editor::piece_2_blob::Piece2BlobEditor;
use crate::components::editor::piece_3_location::Piece3LocationEditor;
use crate::components::editor::piece_4_activities::Piece4ActivityEditor;

pub mod piece_0_text;
pub mod piece_1_mood;
pub mod piece_2_blob;
pub mod piece_3_location;
pub mod piece_4_activities;

#[component]
pub fn PieceEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    rsx! {
        match piece().piece_type.unwrap() {
            0 => rsx! { Piece0TextEditor { piece: piece} },
            1 => rsx! { Piece1MoodEditor {piece:  piece } },
            2 => rsx! { Piece2BlobEditor {piece:  piece } },
            3 => rsx! { Piece3LocationEditor { piece: piece } },
            4 => rsx! { Piece4ActivityEditor {piece:  piece } },
            _ => rsx! {},
        }
    }
}