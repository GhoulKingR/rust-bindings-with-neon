use neon::prelude::*;

fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let num1 = cx
        .argument::<JsNumber>(0)? // Access the first argument
        .value(&mut cx);
    let num2 = cx
        .argument::<JsNumber>(1)? // Access the second argument
        .value(&mut cx);

    Ok(cx.number(num1 + num2))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("add", add)?;
    Ok(())
}
