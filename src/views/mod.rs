//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`DebugMenu`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod debugmenu;
pub mod importers;
pub mod journal_list;
mod journalpaginate;
mod navbar;
mod testpaginate;

pub use debugmenu::DebugMenu;
pub use importers::ImportersView;
pub use journal_list::JournalList;
pub use journalpaginate::JournalPaginate;
pub use navbar::Navbar;
pub use testpaginate::TestPaginate;
