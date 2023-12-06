use std::collections::HashMap;

use serde::{Serialize, Deserialize};

pub fn compare_module(mako_modules: &Vec<String>, webpack_modules: &Vec<String>) -> (Vec<String>, Vec<String>, HashMap<String, String>) {
    let mut webpack_more: Vec<String> = vec![];
    let mut mako_more: Vec<String> = vec![];
    let mut connect_map: HashMap<String, String> = HashMap::new();

    for w in webpack_modules {
        let mut more = true;
        for m in mako_modules {
            if w.ends_with(m) {
                connect_map.insert(w.clone(), m.clone());
                more = false;
                break;
            }
        }

        if more {
            webpack_more.push(w.clone());
        }
    }

    for m in mako_modules {
        let mut more = true;
        for w in webpack_modules {
            if w.ends_with(m) {
                more = false;
                break;
            }
        }
        if more {
            mako_more.push(m.clone());
        }
    }

    (webpack_more, mako_more, connect_map)
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeDiff {
    mako: String,
    webpack: String,
    mako_code: String,
    webpack_code: String,
    mako_line: u16,
    webpack_line: u16,
    diff_line: u16
}

fn get_line_count(code: String) -> u16 {
    let line_count = code.matches('\n').count() + 1 ;
    line_count as u16
}

pub fn compare_code(
    mako_code_map: HashMap<String, String>,
    webpack_code_map: HashMap<String, String>,
    connect_map: HashMap<String, String>
) -> Vec<CodeDiff> {
    let mut code_diff_arr: Vec<CodeDiff> = vec![];
    for (id, code) in webpack_code_map.iter() {
        if connect_map.contains_key(id) {
            let mako_id = connect_map.get(id).unwrap();
            let mako_code = mako_code_map.get(mako_id).unwrap();

            let w_len = get_line_count(code.clone());
            let m_len = get_line_count(mako_code.clone());

            code_diff_arr.push(CodeDiff {
                mako: mako_id.clone(),
                webpack: id.clone(),
                mako_code: mako_code.clone(),
                webpack_code: code.clone(),
                mako_line: m_len,
                webpack_line: w_len,
                diff_line: w_len.abs_diff(m_len)
            })
        }
    }

    code_diff_arr.sort_by(|a, b| b.diff_line.cmp(&a.diff_line));

    code_diff_arr
}

