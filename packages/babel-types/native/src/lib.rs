#[macro_use]
extern crate neon;

use neon::prelude::*;

fn is_type(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let node_type = cx.argument::<JsString>(0)?.value();
    let target_type = cx.argument::<JsString>(1)?.value();

    if node_type == target_type {
        return Ok(cx.boolean(true));
    }
    // TODO:
    // // This is a fast-path. If the test above failed, but an alias key is found, then the
    // // targetType was a primary node type, so there's no need to check the aliases.
    // if (ALIAS_KEYS[targetType]) return false;
    //
    // const aliases: ?Array<string> = FLIPPED_ALIAS_KEYS[targetType];
    // if (aliases) {
    //   if (aliases[0] === nodeType) return true;
    //
    //   for (const alias of aliases) {
    //     if (nodeType === alias) return true;
    //   }
    // }
    Ok(cx.boolean(false))
}

register_module!(mut cx, { cx.export_function("isType", is_type) });
