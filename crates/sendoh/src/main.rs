mod mako;
mod webpack;
mod utils;
mod lyy_parse;
mod lyy_webpack_visit;
mod lyy_mako_visit;

use crate::webpack::Webpack;
use crate::mako::Mako;

fn main() {
    let mut webpack = Webpack::new("dist/webpack");
    webpack.get_files();

    let mut mako = Mako::new("dist/mako");
    mako.get_files();

    // 将 HashSet 转换为 Vec，并对 Vec 进行排序
    let mut webpack_sorted_vec: Vec<String> = webpack.module_ids.into_iter().collect();
    webpack_sorted_vec.sort_unstable();

    let mut mako_sorted_vec: Vec<String> = mako.module_ids.into_iter().collect();
    mako_sorted_vec.sort_unstable();
    println!("webpack module_ids: {:#?}\n", mako_sorted_vec);

    // println!("webpack_len: {}\nmako_len: {}", webpack.module_ids.len(), mako.module_ids.len());
}