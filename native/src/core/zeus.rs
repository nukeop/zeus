use std;
use neon::prelude::*;
use core::cpu::CPU;
use core::memory::{RAM, Memory};
use core::rom::Rom;

pub struct Zeus {
    pub cpu: CPU,
    pub rom: Option<Rom>
}

impl Zeus {
    pub fn new() -> Zeus {
        Zeus {
            cpu: CPU::new(),
            rom: None
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
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
        let mut j = 0;
        
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
                arr.set(&mut cx, j as u32, *value);
                j+=1;
            } 
        }

        Ok(arr)
    }

    pub fn load_rom<'a>(&mut self, mut cx: FunctionContext<'a>) ->
        JsResult<'a, JsObject> {
            let result = cx.empty_object();
            let filename: Handle<JsString> = cx.argument(0).unwrap();
            let filename_s = (*filename).value();

            let rom = Rom::load(&filename_s).unwrap();

            let mut mut_cx = &mut cx;
            let magic = mut_cx.empty_array();
            
            for (i, byte) in rom.header.magic.iter().enumerate() {
                let num = mut_cx.number(*byte);
                magic.set(mut_cx, i as u32, num);
            }
            result.set(mut_cx, "magic", magic);

            let version = mut_cx.empty_array();
            for (i, byte) in rom.header.version.iter().enumerate() {
                let num = mut_cx.number(*byte);
                version.set(mut_cx, i as u32, num);
            }
            result.set(mut_cx, "version", version);

            let mut rom_data = [0;0xE000];
            for (place, element) in rom_data.iter_mut().zip(rom.data.iter()) {
                *place = *element;
            }
            self.cpu.ram.rom_mem = rom_data;

            let rom_data = mut_cx.empty_array();
            for (i, byte) in rom.data.iter().enumerate() {
                let num = mut_cx.number(*byte);
                rom_data.set(mut_cx, i as u32, num);
            }
            result.set(mut_cx, "data", rom_data);
            
            self.rom = Some(rom);
            self.reset();
            Ok(result)
        }
}
