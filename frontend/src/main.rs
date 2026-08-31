// Copyright (c) 2026 Oscar Mora / SetaeSense. All rights reserved.

use leptos::*;

mod app;
mod components;
mod pages;

use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
