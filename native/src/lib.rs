#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate neon;
extern crate zeus_core;

use std::sync::Mutex;
use neon::prelude::*;

mod zeus;

lazy_static! {
    static ref zeusInst: Mutex<zeus::Zeus> = Mutex::new(zeus::Zeus::new());
}

fn get_memory(mut cx: FunctionContext) -> JsResult<JsArray> {
    zeusInst.lock().unwrap().get_memory(cx)
}

fn get_screen(mut cx: FunctionContext) -> JsResult<JsArray> {
    zeusInst.lock().unwrap().screen_to_jsarray(&mut cx)
}

fn step(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    zeusInst.lock().unwrap().step();

    Ok(cx.undefined())
}

fn run_frame(mut cx: FunctionContext) -> JsResult<JsObject> {
    zeusInst.lock().unwrap().run_frame(cx)
}

fn load_rom(mut cx: FunctionContext) -> JsResult<JsObject> {
    zeusInst.lock().unwrap().load_rom(cx)
}


register_module!(mut cx, {
    cx.export_function("getMemory", get_memory);
    cx.export_function("getScreen", get_screen);
    cx.export_function("step", step);
    cx.export_function("runFrame", run_frame);
    cx.export_function("loadRom", load_rom);
    
    Ok(())
});
