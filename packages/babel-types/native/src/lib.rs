use lazy_static::lazy_static;
use neon::prelude::*;
use neon::register_module;
use std::collections::HashMap;

fn to_hashmap(arr: Vec<(&str, Vec<&str>)>) -> HashMap<String, Vec<String>> {
    arr.iter()
        .map(|(k, v)| (k.to_string(), v.iter().map(|i| i.to_string()).collect()))
        .collect()
}

lazy_static!{
    // TODO: generate these, and consider storing them precomputed using a PHF.
    pub static ref ALIAS_KEYS: HashMap<String, Vec<String>> = to_hashmap(vec![
        ("ArrayExpression", vec!["Expression"])
    ]);

    pub static ref FLIPPED_ALIAS_KEYS: HashMap<String, Vec<String>> =  to_hashmap(vec![
        ("Expression", vec!["ArrayExpression"])
    ]);
}

fn is_type(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let node_type = cx.argument::<JsString>(0)?.value();
    let target_type = cx.argument::<JsString>(1)?.value();

    if node_type == target_type {
        return Ok(cx.boolean(true));
    }
    // This is a fast-path. If the test above failed, but an alias key is found, then the
    // targetType was a primary node type, so there's no need to check the aliases.
    if ALIAS_KEYS.contains_key(&target_type) {
        return Ok(cx.boolean(false));
    };

    if let Some(aliases) = FLIPPED_ALIAS_KEYS.get(&target_type) {
        for alias in aliases {
            if &node_type == alias {
                return Ok(cx.boolean(true));
            }
        }
    }
    Ok(cx.boolean(false))
}

register_module!(mut cx, { cx.export_function("isType", is_type) });
