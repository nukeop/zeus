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

    pub fn screen_to_jsarray<'a>(&mut self, cx: &mut
                                 FunctionContext<'a>) -> JsResult<'a, JsArray> {
        let arr = cx.empty_array();
        let screen_pixels = self.cpu.screen.pixels.iter();
        let mut j = 0;

        for byte in screen_pixels {
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
                arr.set(&mut *cx, j as u32, *value);
                j+=1;
            }
        }
        
        Ok(arr)
    }

    pub fn run_frame<'a>(&mut self, mut cx: FunctionContext<'a>) ->
        JsResult<'a, JsObject> {
            let result = cx.empty_object();
            self.cpu.run_frame();
            let screen = self.screen_to_jsarray(&mut cx)?;
            result.set(&mut cx, "screen", screen);

            let mem = cx.empty_array();

            for (i, byte) in self.cpu.ram.mem.iter().enumerate() {
                let num = cx.number(*byte as u32);
                mem.set(&mut cx, i as u32, num);
            }

            result.set(&mut cx, "memory", mem);

            let seven_segment = cx.empty_object();
            let score = cx.empty_array();
            let hi_score = cx.empty_array();

            let score_mem = self.cpu.ram.mem.iter().skip(41).take(5).enumerate();
            for (i, byte) in score_mem {
                let num = cx.number(*byte as u32);
                score.set(&mut cx, i as u32, num);
            }

            let hi_score_mem = self.cpu.ram.mem.iter().skip(46).take(5).enumerate();
            for (i, byte) in hi_score_mem {
                let num = cx.number(*byte as u32);
                hi_score.set(&mut cx, i as u32, num);
            }
            
            seven_segment.set(&mut cx, "score", score);
            seven_segment.set(&mut cx, "hiscore", hi_score);
            result.set(&mut cx, "sevenSegment", seven_segment);
            
            Ok(result)
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
