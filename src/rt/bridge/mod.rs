use std::sync::{Arc, Mutex};

use wasmer::{TypedFunction, Store, Instance, MemoryType, Pages, WasmPtr, imports, Memory, Function, Module, FunctionEnv, MemoryView, FunctionEnvMut};

use crate::{W4Process, w4::{self, PALETTE_ADDR, FRAMEBUFFER_ADDR}};

pub (crate) mod fb;

pub struct WasmerW4Process {
    store: Store,
    instance: Instance,
    mem: Memory,
    start_fn: Option<TypedFunction<(), ()>>,
    update_fn: Option<TypedFunction<(), ()>>,
}

pub (crate) struct WasmerW4State {
    mem: Memory,
}

type WasmerW4StateRef = WasmerW4State;

impl WasmerW4Process {
    pub fn new(wa_module_bytes: Vec<u8>) -> Self {   
        let mut store = Store::default();
        let module = Module::new(&store, wa_module_bytes)
        .unwrap();
        let w4memory_type = MemoryType{minimum:Pages(1), maximum:Some(Pages(1)), shared:false};
        let mem = Memory::new(&mut store, w4memory_type).unwrap();

        let state = WasmerW4State {mem: mem.clone()};

        let fenv = FunctionEnv::new(&mut store, state);
        let imports = imports!{
            "env" => {
                "memory" => mem.clone(),
                "blit" => Function::new_typed_with_env(&mut store, &fenv, fb::blit),
                "textUtf8" => Function::new_typed_with_env(&mut store, &fenv, fb::text_len)
            }
        };
        
        let instance = Instance::new(&mut store, &module, &imports).unwrap();    
            
        
        let start_fn: Option<TypedFunction<(), ()>> = instance.exports.get_typed_function(&store, "start").ok();
        let update_fn: Option<TypedFunction<(), ()>> = instance.exports.get_typed_function(&store, "update").ok();
        
        let ww4 = Self {store, mem, instance, start_fn, update_fn};
        ww4
    }
}

impl W4Process for WasmerW4Process {
    fn start(&mut self) {
        self.start_fn.as_mut()
        .map(|f|f.call(&mut self.store));
    }

    fn update(&mut self) {
        self.update_fn.as_mut()
        .map(|f|f.call(&mut self.store));
    }

    fn read_raw_palette(&self, buf: &mut [u8; 3*4]) {
        let view = self.mem.view(&self.store);
        view.read(PALETTE_ADDR as u64, buf).unwrap();
    }

    fn write_raw_palette(&self, buf: &[u8; 3*4]) {
        let view = self.mem.view(&self.store);
        view.write(PALETTE_ADDR as u64, buf).unwrap();
    }
    
    fn read_fb(&self, buf: &mut [u8]) {
        let view = self.mem.view(&self.store);
        view.read(FRAMEBUFFER_ADDR as u64, buf).unwrap();
    }
}
