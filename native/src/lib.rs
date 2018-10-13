#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate neon;

pub mod core;

use std::sync::Mutex;
use neon::prelude::*;
use core::zeus::Zeus;

lazy_static! {
    static ref zeus: Mutex<Zeus> = Mutex::new(Zeus::new());
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn get_memory(mut cx: FunctionContext) -> JsResult<JsArray> {
    zeus.lock().unwrap().get_memory(cx)
}


register_module!(mut cx, {
    cx.export_function("hello", hello);
    cx.export_function("getMemory", get_memory);
    
    Ok(())
        
});
