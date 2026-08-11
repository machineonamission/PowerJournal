//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`DebugMenu`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

pub mod debugmenu;
pub mod importers;
pub mod journal_list;
pub mod journalpaginate;
pub mod navbar;
pub mod new_entry;
pub mod settings;

pub use debugmenu::*;
pub use importers::*;
pub use journal_list::*;
pub use journalpaginate::*;
pub use navbar::*;
pub use new_entry::*;
pub use settings::*;