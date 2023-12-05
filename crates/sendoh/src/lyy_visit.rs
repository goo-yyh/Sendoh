use std::collections::HashMap;
use swc_ecma_ast::{
    KeyValueProp, PropName, Str
};

use swc_ecma_visit::Visit;

pub struct WebpackAsyncVisit {
    module_ids: Vec<String>,
    module_map: HashMap<String, String>
}

impl Visit for WebpackAsyncVisit {
    fn visit_key_value_prop(&mut self, n: &KeyValueProp) {
        if let KeyValueProp {
            key: PropName::Str(Str {
                value,
                ..
            }),
            ..
        } = n {
            println!("value: {}", value);
        }
    }
}

