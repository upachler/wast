use std::sync::{Arc, Mutex};

use wasmer::{TypedFunction, Store, Instance, MemoryType, Pages, WasmPtr, imports, Memory, Function, Module, FunctionEnv, MemoryView, FunctionEnvMut};

use crate::{W4Process, w4};

pub (crate) mod fb;

pub struct WasmerW4Process {
    state: Arc<Mutex<WasmerW4State>>,
    instance: Instance,
    start_fn: Option<TypedFunction<(), ()>>,
    update_fn: Option<TypedFunction<(), ()>>,
}

pub (crate) struct WasmerW4State {
    mem: Memory,
    store: Store,
}

type WasmerW4StateRef = Arc<Mutex<WasmerW4State>>;

impl WasmerW4Process {
    pub fn new(wa_module_bytes: Vec<u8>) -> Self {   
        let mut store = Store::default();
        let module = Module::new(&store, wa_module_bytes)
        .unwrap();
        let w4memory_type = MemoryType{minimum:Pages(1), maximum:Some(Pages(1)), shared:false};
        let mem = Memory::new(&mut store, w4memory_type).unwrap();

        let state = Arc::new(Mutex::new(WasmerW4State {mem: mem.clone(), store}));

        let store = &mut state.lock().unwrap().store;
        let fenv = FunctionEnv::new(store, state.clone());
        let imports = imports!{
            "env" => {
                "memory" => mem,
                "blit" => Function::new_typed_with_env(store, &fenv, fb::blit),
                "textUtf8" => Function::new_typed_with_env(store, &fenv, fb::text_len)
            }
        };
        
        let instance = Instance::new(store, &module, &imports).unwrap();    
            
        
        let start_fn: Option<TypedFunction<(), ()>> = instance.exports.get_typed_function(store, "start").ok();
        let update_fn: Option<TypedFunction<(), ()>> = instance.exports.get_typed_function(store, "update").ok();
        
        let ww4 = Self {state: state.clone(), instance, start_fn, update_fn};
        ww4
    }
}

impl WasmerW4Process {
    fn invoke_callback(&self, callback_opt: Option<&TypedFunction<(),()>>) {
        let mut state = self.state.lock().unwrap();
        callback_opt.map(|f|f.call(&mut state.store));
    }
}
impl W4Process for WasmerW4Process {
    fn start(&mut self) {
        self.invoke_callback(self.start_fn.as_ref());
    }

    fn update(&mut self) {
        self.invoke_callback(self.update_fn.as_ref());
    }
}
