//use crate::components::Hero;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let contents = "<h2>Heading</h2><p><strong>bold text</strong></p><h3>sub heading</h3><p>The exponential is defined by</p><p>. I guess the formula doesn't render.</p>";
    rsx! {
        div { dangerous_inner_html: "{contents}" }

    }
}
