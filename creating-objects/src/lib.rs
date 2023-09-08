use neon::prelude::*;

fn get_user(mut cx: FunctionContext) -> JsResult<JsObject> {
    // Declare an empty object
    let obj = cx.empty_object();

    // Declare a string value
    let name = cx.string("Chigozie");

    // Declare a number value
    let age = cx.number(19);

    // Set the `name` field in `obj`
    obj.set(&mut cx, "name", name)?;

    // Set the `age` field in `obj`
    obj.set(&mut cx, "age", age)?;
    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getUser", get_user)?;
    Ok(())
}
