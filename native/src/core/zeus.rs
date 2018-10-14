use neon::prelude::*;
use core::cpu::CPU;
use core::memory::{RAM, Memory};

pub struct Zeus {
    pub cpu: CPU
}

impl Zeus {
    pub fn new() -> Zeus {
        Zeus {
            cpu: CPU::new()
        }
    }

    pub fn step(&mut self) {
    }

    pub fn get_memory<'a>(&mut self, mut cx: FunctionContext<'a>) -> JsResult<'a, JsArray> {
        let arr = cx.empty_array();

        for (i, byte) in self.cpu.ram.mem.iter().enumerate() {
            let num = cx.number(*byte as u32);
            arr.set(&mut cx, i as u32, num);
        }
        
        Ok(arr)
    }

    pub fn get_screen<'a>(&mut self, mut cx: FunctionContext<'a>) -> JsResult<'a, JsArray> {
        let arr = cx.empty_array();
        let screen_mem = self.cpu.ram.mem.iter().take(40);
        for (i, byte) in screen_mem.enumerate() {
            let values = [
                cx.boolean(byte & 0x80 != 0),
                cx.boolean(byte & 0x40 != 0),
                cx.boolean(byte & 0x20 != 0),
                cx.boolean(byte & 0x10 != 0),
                cx.boolean(byte & 0x08 != 0),
                cx.boolean(byte & 0x04 != 0),
                cx.boolean(byte & 0x02 != 0),
                cx.boolean(byte & 0x01 != 0)
            ];

            for value in values.iter() {
                arr.set(&mut cx, i as u32, *value);
            } 
        }

        Ok(arr)
    }
}
