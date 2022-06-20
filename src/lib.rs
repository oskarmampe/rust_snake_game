mod random;
mod snake;

use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use snake::SnakeGame;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, UnwrapThrowExt,
};
use web_sys::{console, window};

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 20)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || game.borrow_mut().tick()
    }) as Box<dyn FnMut()>);
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting ...".into());

    TICK_CLOSURE.with(|closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw();
    });
}
