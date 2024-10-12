use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Home" }
        Link { to: Route::Nix {}, "Nix" }
        Link { to: Route::Rust {}, "Rust" }
        Link { to: Route::About {}, "About" }
        Outlet::<Route> {}
    }
}
