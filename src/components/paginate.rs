use crate::components::pieces::Piece;
use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::logger::tracing::log::log;
use dioxus::prelude::*;
use sea_orm::compound::EntityLoaderPaginator;
use sea_orm::{DatabaseConnection, EntityLoaderTrait, EntityTrait, ModelTrait, QueryOrder};

#[component]
fn Page<E: EntityTrait, C: EntityLoaderTrait<E> + 'static>(
    num: i64,
    loader: Signal<C>,
    render: Callback<C::ModelEx, Element>,
) -> Element {
    let db_signal = use_context::<Resource<DatabaseConnection>>();
    let entries: Resource<Vec<C::ModelEx>> = use_resource(move || async move {
        // If either the DB or Paginator isn't ready yet, abort and return None
        debug!("loading {num}");
        let Some(db) = db_signal() else { return vec![] };

        let r = loader
            .peek()
            .cloned()
            .paginate(&db, 10)
            .fetch_page(num as u64)
            .await
            .unwrap();

        r
    });

    rsx! {
        for entry in entries.read().cloned().unwrap_or(vec![]) {
            {render.call(entry)}
        }
    }
}

#[component]
pub fn Paginate<E: EntityTrait, C: EntityLoaderTrait<E> + 'static>(
    loader: Signal<C>,
    render: Callback<C::ModelEx, Element>,
) -> Element {
    let mut current_page = use_signal(|| 0_i64);
    // after literal hours of figuring out a mechanism to get this to work, the sentinels that
    // turn on with the signal was ultimately Claude's idea, but using onvisible elements and
    // toggling on mount were my contributions, i tried lots and this works best
    let mut sentinels_active = use_signal(|| false);

    let db_signal = use_context::<Resource<DatabaseConnection>>();

    // fires once when db becomes Some, stays cached after — total_pages() reads the cache
    let total_pages: Resource<i64> = use_resource(move || async move {
        let Some(db) = db_signal() else { return 0 };
        loader.peek().clone()
            .paginate(&db, 10)
            .num_pages()
            .await
            .unwrap_or(0) as i64
    });


    rsx! {
        if sentinels_active() && current_page() > 0 {
            div {
                onvisible: move |_| {
                    if current_page() > 0 { debug!("page down"); current_page -= 1; }
                },
                "Loading..."
            }
        }
        for page in (current_page() - 1).max(0)..=(current_page() + 1) {
            div {
                key: "{page}",
                style: "min-height: 100vh",
                onmounted: move |_| {
                    // last of the initial batch to mount flips this on
                    debug!("MOUNTED!");
                    sentinels_active.set(true);
                },
                // onvisible: move |_| {
                //     debug!("page {page} visible");
                //     if *sentinels_active.peek() && *current_page.peek() != page {
                //         sentinels_active.set(false);
                //         // current_page.set(page);
                //     }
                // },
                Page {
                    num: page,
                    loader: loader.clone(),
                    render: render.clone(),
                }
            }
        }
        if current_page() + 2 < total_pages().unwrap_or(0) as i64 {
            div {
                onvisible: move |_| { debug!("page up"); current_page += 1; },
            },
            "Loading..."
        }
    }
}
