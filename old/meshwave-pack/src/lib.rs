#![feature(array_map)]
mod utils;

// inspired by https://www.reddit.com/r/proceduralgeneration/comments/o88ual/magnets_generatively_warped_line/
use game_loop::game_loop;
use noise::{ NoiseFn, SuperSimplex };

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use derivative::Derivative;

use rand::prelude::{ thread_rng, ThreadRng };
use rand::Rng;

use js_sys::Date;

use std::rc::Rc;
use std::cell::RefCell;

const UPDATE_RATE: u32 = 20; // updates per second

const NOISE_RANGE: f64 = 100.;
const NOISE_SCALE: f64 = 300.;
const CHANGE_SPEED: f64 = 0.13;
const RESOLUTION: f64 = 0.04; // points per pixel
const MOUSE_RADIUS: f64 = 2.5;

 // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
 // allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Derivative)]
#[derivative(Debug)]
struct Lines {
    size_w: f64,
    size_h: f64,
    mouse_x: f64,
    mouse_y: f64,
    #[derivative(Debug="ignore")]
    canvas: Rc<web_sys::HtmlCanvasElement>,
    #[derivative(Debug="ignore")]
    noise: SuperSimplex,
    #[derivative(Debug="ignore")]
    rng: ThreadRng,
}
impl Lines {
    fn new(canvas: Rc<web_sys::HtmlCanvasElement>) -> Rc<RefCell<Lines>> {
        let (size_w, size_h) = (canvas.client_width() as f64, canvas.client_height() as f64);
        let rng = thread_rng();
        let noise = SuperSimplex::new();

        let ret = Rc::new(RefCell::new(Lines { mouse_x: size_w/2., mouse_y: size_h/2., canvas: canvas.clone(), size_w, size_h, noise, rng }));

        {   // mousemove listener
            let ret = ret.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                ret.borrow_mut().mouse_x = event.client_x() as f64;
                ret.borrow_mut().mouse_y = event.client_y() as f64;
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .expect("failed to add mousemove event listener");
            closure.forget();
        }

        ret
    }

    fn render(&mut self) {

        let pos = Date::now() / 1e3 * CHANGE_SPEED;
        
        // TODO: move this canvas size getting and ctx refreshing to onresize handler
        let (size_w, size_h) = (self.canvas.client_width() as f64, self.canvas.client_height() as f64);
        self.canvas.set_width(size_w as u32);
        self.canvas.set_height(size_h as u32);

        let ctx = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        ctx.set_fill_style(  &JsValue::from_str(&format!("#3d86ff")));

        ctx.clear_rect(0., 0., size_w, size_h);

        for x in (-NOISE_RANGE*RESOLUTION) as i32..((size_w + NOISE_RANGE)*RESOLUTION) as i32 {
            for y in (-NOISE_RANGE*RESOLUTION) as i32..((size_h + NOISE_RANGE)*RESOLUTION) as i32 {
                let actual_x = x as f64 / RESOLUTION;
                let actual_y = y as f64 / RESOLUTION;
                let dist = ((actual_x-self.mouse_x).powf(2.) + (actual_y-self.mouse_y).powf(2.)).sqrt();
                let radius = size_w.max(size_h) / MOUSE_RADIUS;
                if self.rng.gen_bool(1./((dist/radius).powf(4.)+1.)) {
                    let x = x as f64 / RESOLUTION;
                    let y = y as f64 / RESOLUTION;

                    let noise_x = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE,  pos*CHANGE_SPEED])
                        as f64 * NOISE_RANGE;
                    let noise_y = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE, -pos*CHANGE_SPEED])
                        as f64 * NOISE_RANGE;

                    ctx.fill_rect(x + noise_x, y + noise_y, 3., 3.);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn greet() -> i32 {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("meshwave-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    let sim = Lines::new(Rc::new(canvas));
    game_loop(sim, UPDATE_RATE, 0.1, |_| {
        // update fn
    }, |g| {
        g.game.borrow_mut().render();
    });

    0
}
