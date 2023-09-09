use neon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn write_file(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    if let Ok(mut file) = File::create("foo.txt") {
        if let Ok(()) = file.write_all(b"Hello, world!") {
            Ok(cx.boolean(true))
        } else {
            Ok(cx.boolean(false))
        }
    } else {
        Ok(cx.boolean(false))
    }
}

fn read_file(mut cx: FunctionContext) -> JsResult<JsString> {
    match File::open("foo.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                Ok(cx.string(contents))
            } else {
                Ok(cx.string(""))
            }
        }
        Err(_) => Ok(cx.string("")),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("readFile", read_file)?;
    cx.export_function("writeFile", write_file)?;
    Ok(())
}
