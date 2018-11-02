use std;
use neon::prelude::*;
use core::cpu::CPU;
use core::memory::{RAM, Memory};
use core::rom::Rom;
use core::seven_segment::SevenSegment;

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

    pub fn seven_segment_to_jsarray<'a>(cx: &mut FunctionContext<'a>,
        display: &SevenSegment) -> JsResult<'a, JsArray> {
        let result = cx.empty_array();

        let digits = display.digits.iter().enumerate();
            for (i, digit) in digits {
                let val = digit.val;
                let digit = cx.empty_array();
                let values = [
                    cx.boolean(val & 0x80 != 0),
                    cx.boolean(val & 0x40 != 0),
                    cx.boolean(val & 0x20 != 0),
                    cx.boolean(val & 0x10 != 0),
                    cx.boolean(val & 0x08 != 0),
                    cx.boolean(val & 0x04 != 0),
                    cx.boolean(val & 0x02 != 0),
                    cx.boolean(val & 0x01 != 0)
                ];
                for (j, value) in values.iter().enumerate() {
                    digit.set(cx, j as u32, *value);
                }

                result.set(cx, i as u32, digit);
            }

        Ok(result)
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
            let score = Zeus::seven_segment_to_jsarray(
                &mut cx,
                &self.cpu.score
            )?;
            
            let hi_score = Zeus::seven_segment_to_jsarray(
                &mut cx,
                &self.cpu.hi_score
            )?;

            let speed_lcd = Zeus::seven_segment_to_jsarray(
                &mut cx,
                &self.cpu.speed_lcd
            )?;

            let level_lcd = Zeus::seven_segment_to_jsarray(
                &mut cx,
                &self.cpu.level_lcd
            )?;

            seven_segment.set(&mut cx, "score", score);
            seven_segment.set(&mut cx, "hiscore", hi_score);
            seven_segment.set(&mut cx, "speed", speed_lcd);
            seven_segment.set(&mut cx, "level", level_lcd);
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
