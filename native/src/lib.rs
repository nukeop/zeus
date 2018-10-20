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

fn get_memory(mut cx: FunctionContext) -> JsResult<JsArray> {
    zeus.lock().unwrap().get_memory(cx)
}

fn get_screen(mut cx: FunctionContext) -> JsResult<JsArray> {
    zeus.lock().unwrap().screen_to_jsarray(&mut cx)
}

fn step(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    zeus.lock().unwrap().step();

    Ok(cx.undefined())
}

fn run_frame(mut cx: FunctionContext) -> JsResult<JsObject> {
    zeus.lock().unwrap().run_frame(cx)
}

fn load_rom(mut cx: FunctionContext) -> JsResult<JsObject> {
    zeus.lock().unwrap().load_rom(cx)
}


register_module!(mut cx, {
    cx.export_function("getMemory", get_memory);
    cx.export_function("getScreen", get_screen);
    cx.export_function("step", step);
    cx.export_function("runFrame", run_frame);
    cx.export_function("loadRom", load_rom);
    
    
    Ok(())
});
