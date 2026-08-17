use crate::database::entity::piece;
use crate::database::entity::piece::ActiveModelExStoreExt;
use crate::store_lenses::ActiveHasOneStoreImplExt;
use dioxus::prelude::*;
use dioxus_leaflet::{LatLng, Map, MapPosition, Marker, Popup};
use sea_orm::Set;
use crate::database::entity::piece_3_location::ActiveModelExStoreExt as OtherActiveModelExStoreExt;

#[component]
pub fn Piece3LocationEditor(mut piece: Store<piece::ActiveModelEx>) -> Element {
    let mut store = piece.piece_3_location().model_or_default();

    // A signal to store the user's selected coordinates
    // let mut selected_pos = use_signal(|| None::<MapPosition>);
    store.lat();

    rsx! {
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            height: "500px",
            width: "100%",
            on_click: move |position: LatLng| {
                store.lat().set(Set(position.lat));
                store.lat().set(Set(position.lng));
            },
            if let Set(lat) = store.lat()() && let Set(lng) = store.lon()() {
                Marker {
                    coordinate: LatLng::new(lat, lng),
                    Popup {
                        b { "London" }
                        br { }
                        "Capital of England"
                    }
                }
            }
        }
    }
}