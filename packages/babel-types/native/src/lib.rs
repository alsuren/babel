use neon::prelude::*;
use neon::register_module;

mod generated_constants;

use self::generated_constants::{ALIAS_KEYS, FLIPPED_ALIAS_KEYS};

fn is_type(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let node_type = match cx.argument::<JsString>(0) {
        Ok(s) => s.value(),
        // TODO: work out why this is sometimes undefined?
        Err(_) => {
            // println!("Not a string: {:?}", cx.argument_opt(0));
            return Ok(cx.boolean(false));
        }
    };
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
