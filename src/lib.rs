use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct FibonacciEval {
    first: i32,
    second: i32,
}

#[wasm_bindgen]
impl FibonacciEval{
    /// Create new instance.
    pub fn new() -> FibonacciEval {
        FibonacciEval { first: 0, second: 1 }
    }

    pub fn advance(&mut self) -> i32 {
        let tmp = self.first + self.second;
        self.first = self.second;
        self.second = tmp;
        tmp
    }
}

/// Run entry point for the main thread.
#[wasm_bindgen]
pub fn startup() {

    // Here, we create our worker. In a larger app, multiple callbacks should be
    // able to interact with the code in the worker. Therefore, we wrap it in
    // `Rc<RefCell>` following the interior mutability pattern. Here, it would
    // not be needed but we include the wrapping anyway as example.
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within Wasm".into());

    // Setup message handler
    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {

        let value = event.data();
        let number_str = if let Some(n) = value.as_f64() {
            n.to_string()
        } else {
            "Invalid data".to_string()
        };

        console::log_1(&"GOT MESSAGE".into());
        console::log_1(&number_str.clone().into());

        let document = web_sys::window().unwrap().document().unwrap();

        let result_div = document
            .get_element_by_id("resultField")
            .expect("#resultField must exist")
            .dyn_into::<HtmlElement>()
            .unwrap();

        result_div.set_inner_text(&number_str);
    }) as Box<dyn FnMut(_)>);

    console::log_1(&"Setting the handler".into());
    worker_handle
        .borrow()
        .set_onmessage(Some(callback.as_ref().unchecked_ref()));

    callback.forget(); // Leak the closure to keep it alive
}