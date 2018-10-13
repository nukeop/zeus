use neon::prelude::*;
use core::cpu::CPU;
use core::memory::RAM;

pub struct Zeus {
    pub cpu: CPU
}

impl Zeus {
    pub fn new() -> Zeus {
        Zeus {
            cpu: CPU::new()
        }
    }

    pub fn get_memory<'a>(&mut self, mut cx: FunctionContext<'a>) -> JsResult<'a, JsArray> {
        let arr = cx.empty_array();

        for (i, byte) in self.cpu.ram.mem.iter().enumerate() {
            let num = cx.number(*byte as u32);
            arr.set(&mut cx, i as u32, num);
        }
        
        Ok(arr)
    }
}
