mod config;
mod mako;
mod webpack;
mod utils;
mod lyy_parse;
mod lyy_webpack_visit;
mod lyy_mako_visit;
mod lyy_config_visit;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serde_json;

use crate::webpack::Webpack;
use crate::mako::Mako;
use crate::config::Config;
use crate::utils::{compare_module, compare_code, CodeDiff};

fn main() {
    let mut Config = Config::new("/Users/yuyuehui/umi-max-test/.umirc.ts");

    Config.edit_to_webpack();
}

fn _main() {
    let mut webpack = Webpack::new("dist/webpack");
    webpack.get_files();

    let mut mako = Mako::new("dist/mako");
    mako.get_files();

    // 将 HashSet 转换为 Vec，并对 Vec 进行排序
    let mut webpack_sorted_vec: Vec<String> = webpack.module_ids.into_iter().collect();
    webpack_sorted_vec.sort_unstable();

    let mut mako_sorted_vec: Vec<String> = mako.module_ids.into_iter().collect();
    mako_sorted_vec.sort_unstable();
    // println!("webpack module_ids: {:#?}\n", mako_sorted_vec);
    // println!("webpack_len: {}\nmako_len: {}", webpack.module_ids.len(), mako.module_ids.len());

    let (webpack_more, mako_more, connect_map) = compare_module(&mako_sorted_vec, &webpack_sorted_vec);

    let code_diff_arr = compare_code(mako.code_map, webpack.code_map, connect_map);

    let mut more_map: HashMap<String, Vec<String>> = HashMap::new();
    more_map.insert("webpack".to_string(), webpack_more);
    more_map.insert("mako".to_string(), mako_more);
    more_map.insert("mako_len".to_string(), mako_sorted_vec);
    more_map.insert("webpack_len".to_string(), webpack_sorted_vec);


    let mut diff_map: HashMap<String, Vec<CodeDiff>> = HashMap::new();
    diff_map.insert("diff".to_string(), code_diff_arr);

    let more_json_str = serde_json::to_string_pretty(&more_map).unwrap();
    let diff_json_str = serde_json::to_string_pretty(&diff_map).unwrap();

    let mut file = File::create("app/src/data/more.json").unwrap();
    file.write(more_json_str.as_bytes()).unwrap();

    let mut file = File::create("app/src/data/diff.json").unwrap();
    file.write(diff_json_str.as_bytes()).unwrap();

    // println!("more_str: {}\ndiff_str: {}", more_json_str, diff_json_str);
    // println!("mako_more: {:#?}\nwebpack_more: {:#?}", mako_more, webpack_more);
    // println!("code_diff_arr: {:#?}", code_diff_arr);
}