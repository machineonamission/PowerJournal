use crate::components::pieces::Piece;
use crate::database::entity::prelude::*;
use crate::Route;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use sea_orm::compound::EntityLoaderPaginator;
use sea_orm::{DatabaseConnection, EntityLoaderTrait, ModelTrait, QueryOrder};

#[component]
fn Page(num: i64) -> Element {
    rsx! {
        for i in 0..100 {
            div {
                key:"{num} {i}",
                "page {num} elem {i}"
            }
        }
    }
}

#[component]
pub fn Paginate(// loader: C,
    // component: Component,
) -> Element {
    // let db_signal = use_context::<Signal<Option<DatabaseConnection>>>();

    // 2. Signals for your paginator state and page state
    // let mut paginator = use_signal(|| None::<EntityLoaderPaginator<DatabaseConnection, E, C>>);

    // // 3. Initialize the paginator EXACTLY ONCE when the DB loads
    // use_effect(move || {
    //     // Reading db_signal() subscribes this effect to DB changes
    //     if let Some(db) = db_signal() {
    //         // Use `.peek()` so we just check the value without subscribing
    //         // the effect to paginator changes, avoiding infinite loops.
    //         if paginator.peek().is_none() {
    //             // Init your paginator and store it in the signal!
    //             // (If this init needs to be async, wrap this block in a spawn(async move { ... }))
    //             paginator.set(Some(loader.paginate(&db, 10))); // or whatever page size you want
    //         }
    //     }
    // });
    //
    // // Fetch data asynchronously when the database becomes available
    // let entries: Resource<Vec<E>> = use_resource(move || async move {
    //     let page = current_page();
    //
    //     // If either the DB or Paginator isn't ready yet, abort and return None
    //     let Some(pag) = paginator() else { return None };
    //
    //     if let Some(users) = pag.fetch_page(page).await.unwrap() {
    //         return users
    //         // for user in users {
    //         //     dbg!(&user);
    //         // }
    //     }
    // });

    // let mut items = use_signal(|| vec![]);
    //
    // let mut add_item = move |name| {
    //     let id = generation();
    //     generation.set(id + 1);
    //     items.write().push(Item { id, name })
    // };
    let mut current_page = use_signal(|| 0_i64);


    rsx! {
        for page in (current_page - 2)..=(current_page + 2) {
            div {
                key: "{page}",
                style: "min-height: 200vh",
                onvisible: move |_| {
                    if (page - current_page()).abs() >= 2 && page >= 0 {
                        *current_page.write() = page;
                    }
                },
                Page { num: page }
            }
        }
        // ...
    }
}
