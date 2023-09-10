use neon::prelude::*;

fn get_user(mut cx: FunctionContext) -> JsResult<JsObject> {
    // Create an empty object
    let obj = cx.empty_object();

    // Create values to store in the object
    let name = cx.string("Chigozie");
    let age = cx.number(19);

    // Store these values in the object
    obj.set(&mut cx, "name", name)?;
    obj.set(&mut cx, "age", age)?;
    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getUser", get_user)?;
    Ok(())
}
