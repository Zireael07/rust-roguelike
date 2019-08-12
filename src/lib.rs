extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//the websys Canvas bindings uses it
use wasm_bindgen::JsCast; // for dyn_into
use std::f64;

//for getting vals to JS side
use std::os::raw::{c_double, c_int};

//for requestAnimationFrame
use std::cell::RefCell;
use std::rc::Rc;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

//use raw_module because we need a relative path
#[wasm_bindgen(raw_module = "../www/js/js-renderer.js")]
extern "C" {
    fn renderPlayer(x: c_double, y: c_double);
}

// Auto-starts on page load
//#[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    //log!("We have a context {:?}", context);


    //clear
    context.set_fill_style(&wasm_bindgen::JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, 800.0, 600.0);

    //renderPlayer(player_x as c_double,player_y as c_double);

    run(context);
}


//based on wasm tut
fn get_window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}



fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    get_window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn run(ctx: web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
    // Here we want to call `requestAnimationFrame` in a loop
    // After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    
    log!("Running the loop...");

    log!("We have a context {:?}", ctx);


    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        // Display how many times this requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);

        //clear
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("black"));
        ctx.fill_rect(0.0, 0.0, 800.0, 600.0);

        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("rgb(255, 255, 255"));
        ctx.fill_text(&text, 2.0, 20.0);
        renderPlayer(1.0, 1.0);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}