#[cfg(feature = "native")]
use neon::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn fibonacci(n: i32) -> i32 {
    return match n {
        n if n < 2 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    };
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn fibonacci_wasm(n: i32) -> i32 {
    fibonacci(n)
}

#[cfg(feature = "native")]
fn fibonacci_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0).unwrap();
    let res = fibonacci(handle.value(&mut cx) as i32);
    Ok(cx.number(res))
}

#[cfg(feature = "native")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fibonacci_rs", fibonacci_api)?;
    Ok(())
}