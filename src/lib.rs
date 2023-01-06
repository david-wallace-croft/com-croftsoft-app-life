use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlDivElement;
use wee_alloc::WeeAlloc;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  let document: Document = window().unwrap().document().unwrap();
  hello_canvas(&document);
  hello_console();
  hello_div(&document);
  Ok(())
}

fn hello_canvas(document: &Document) {
  let element: Element = document.get_element_by_id("canvas").unwrap();
  let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
  let object: Object = html_canvas_element.get_context("2d").unwrap().unwrap();
  let canvas_context: CanvasRenderingContext2d = object.dyn_into().unwrap();
  canvas_context.set_font("normal 14px serif");
  canvas_context.stroke_text("Hello, Canvas!", 0.0, 14.0).unwrap();
}

fn hello_console() {
  console::log_1(&JsValue::from_str("Hello, Console!"));
}

fn hello_div(document: &Document) {
  let element: Element = document.get_element_by_id("div").unwrap();
  let html_div_element: HtmlDivElement = element.dyn_into().unwrap();
  html_div_element.insert_adjacent_text("afterbegin", "Hello, Div!").unwrap();
}
