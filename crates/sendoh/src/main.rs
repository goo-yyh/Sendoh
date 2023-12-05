mod mako;
mod webpack;
mod utils;
mod lyy_parse;
mod lyy_visit;

use crate::webpack::Webpack;

fn main() {
    println!("Hello, world!");

    let webpack = Webpack::new("dist/webpack");
    webpack.get_files();
}