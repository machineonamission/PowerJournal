pub mod piece_0_text;
mod piece_1_mood;
mod piece_2_blob;
mod piece_3_location;
mod piece_4_activities;

use crate::components::pieces::piece_0_text::Piece0Text;
use crate::components::pieces::piece_1_mood::Piece1Mood;
use crate::components::pieces::piece_2_blob::Piece2Blob;
use crate::components::pieces::piece_3_location::Piece3Location;
use crate::components::pieces::piece_4_activities::Piece4Activities;
use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use sea_orm::prelude::HasMany::Loaded;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Piece(piece: piece::ModelEx) -> Element {
    // dbg!(&piece);
    let node = match piece.piece_type {
        0 => {
            rsx!(Piece0Text {
                piece: piece.piece_0_text.unwrap()
            })
        }
        1 => {
            rsx!(Piece1Mood {
                piece: piece.piece_1_mood.unwrap()
            })
        }
        2 => {
            rsx!(Piece2Blob {
                id: piece.id
            })
        }
        3 => {
            rsx!(Piece3Location {
                piece: piece.piece_3_location.unwrap()
            })
        }
        4 => {
            // for some reason, if there's a vec inside, i cant unwrap the object
            // so we just pull out of the enum manally
            if let Loaded(activities) = piece.piece_4_activity {
                rsx!(Piece4Activities { piece: activities })
            } else {
                unreachable!()
            }
        }
        _ => {
            rsx!(p {"invalid piece of type {piece.piece_type}"})
        }
    };
    rsx! {
        {node}
    }
}
