pub mod home;
pub mod about;
pub mod nix;
pub mod rust;

use dioxus::prelude::*;
use crate::routes::{home::Home, nix::Nix, rust::Rust ,about::About};
use crate::components::NavBar;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/nix")]
        Nix {},
        #[route("/rust")]
        Rust{},
        #[route("/about")]
        About {},
}
