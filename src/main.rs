#[macro_use]
extern crate js;

use js::jsapi::CompartmentOptions;
use js::jsapi::JS_NewGlobalObject;
use js::jsapi::OnNewGlobalHookOption;
use js::jsval::UndefinedValue;
use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS};

use std::ptr;

fn main() {
    let rt = Runtime::new().unwrap();
    let cx = rt.cx();

    unsafe {

        rooted!(in(cx) let global =
            JS_NewGlobalObject(cx, &SIMPLE_GLOBAL_CLASS, ptr::null_mut(),
                               OnNewGlobalHookOption::FireOnNewGlobalHook,
                               &CompartmentOptions::default())
        );
        rooted!(in(cx) let mut rval = UndefinedValue());
        let result = rt.evaluate_script(
                global.handle(),
                "console.log(\"hello world\")",
                "test.js",
                1,
                rval.handle_mut());
        let thing = match rval {
            is_object() => "object",
            is_symbol() => "symbol",
            _ => "no idea"
        };

        println!(
            // "evaluate: {:?} result: {:?}",
            "evaluate: {:?}\nthing: {:?}",            
            result.is_ok(),
            thing
        );

    }
}

