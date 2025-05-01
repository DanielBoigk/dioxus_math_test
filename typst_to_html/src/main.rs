use typst_html::html_fragment;
use typst_library::engine::{Engine, SystemWorld};
use typst_library::introspection::Locator;
use typst_library::visualize::StyleChain;

fn main() {
    let source = "the exponential is defined by $e^x = sum_(n=0)^infinity (x^n)/(n!)$.".to_string();


    // Create the engine and compile the document
    let mut engine = Engine::new();
    let html = html_fragment(engine, source, locator, styles)
    println!("{}", html);
}
