use game_loop::game_loop;
use noise::{ NoiseFn, SuperSimplex };

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{ console, EventListener, MouseEvent };

use derivative::Derivative;

use rand::prelude::{ thread_rng, ThreadRng };
use rand::distributions::{ Distribution, uniform::{ Uniform, UniformFloat } };
use rand::Rng;

use itertools::Itertools;

use std::iter::repeat;
use std::mem;

const UPDATE_RATE: u32 = 30; // updates per second

const NUM_POINTS: usize = 10;
const NOISE_RANGE: f64 = 100.;
const NOISE_SCALE: f64 = 300.;
const CHANGE_SPEED: f64 = 0.1;
const RESOLUTION: f64 = 0.12; // points per pixel


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//#[derive(Derivative)]
//#[derivative(Debug)]
//struct Line((f64, f64), (f64, f64)); // assert self.0.0 < self.1.0
//impl Line {
//    fn new(mut a: (f64, f64), mut b: (f64, f64)) -> Line {
//        if a.0 > b.0 { mem::swap(&mut a, &mut b) }     // assert self.0.0 < self.1.0
//        Line { 0: a, 1: b }
//    }
//    //fn from_distributions(udist_w: Uniform<f64>, udist_h: Uniform<f64>, rng: &mut ThreadRng) -> Line {
//    //    let a = (udist_w.sample(&mut rng), udist_h.sample(&mut rng));
//    //    let b = (udist_w.sample(&mut rng), udist_h.sample(&mut rng));
//    //    if a.0 > b.0 { mem::swap(a, b) }     // assert self.0.0 < self.1.0
//    //    Line { a, b }
//    //}
//}

#[derive(Derivative)]
#[derivative(Debug)]
struct Lines {
    //lines: Vec<Line>,
    //points: Vec<(f64, f64)>,
    size_w: f64,
    size_h: f64,
    #[derivative(Debug="ignore")]
    canvas: &'static web_sys::HtmlCanvasElement,
    #[derivative(Debug="ignore")]
    noise: SuperSimplex,
    #[derivative(Debug="ignore")]
    rng: ThreadRng,
}
impl Lines {
    fn new(canvas: &'static web_sys::HtmlCanvasElement, points: usize) -> Lines {

        let (size_w, size_h) = (canvas.client_width() as f64, canvas.client_height() as f64);
        
        let rng = thread_rng();


        ////let udist_w = Uniform::new(0., size_w);
        ////let udist_h = Uniform::new(0., size_h);
        ////let points = repeat(()).take(points)
        ////    .map(|_| (udist_w.sample(&mut rng), udist_h.sample(&mut rng)))
        ////    .collect::<Vec<(f64, f64)>>();
        //let points_per_side = (points as f64).sqrt() as i32;
        //let points = (0..points_per_side).cartesian_product(0..points_per_side)
        //    .map(|(x, y)| (size_w * (x as f64/points_per_side as f64),
        //                   size_h * (y as f64/points_per_side as f64)))
        //    .collect::<Vec<(f64, f64)>>();
        //
        //let mut lines = Vec::<Line>::new();
        //lines.reserve(points.len().pow(2));
        //for a in &points {
        //    for b in &points {
        //        lines.push(Line::new(*a, *b));
        //    }
        //}

        let noise = SuperSimplex::new();

        //Lines { canvas, size_w, size_h, lines, points, noise }
        let ret = Lines { canvas, size_w, size_h, noise, rng };

        {
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {

                //if pressed.get() {
                //    ctx.line_to(event.offset_x() as f64, event.offset_y() as f64);
                //    ctx.stroke();
                //    ctx.begin_path();
                //    ctx.move_to(event.offset_x() as f64, event.offset_y() as f64);
                //}
                // TODO
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .expect("failed to add mousemove event listener");
            closure.forget();
        }
        ret
    }

    //fn draw_line(&self, ctx: &web_sys::CanvasRenderingContext2d, line: &Line, pos: f64) {
    //    let f = |x: f64| (line.0.1-line.1.1) / (line.0.0-line.1.0) * (x - line.0.0) + line.0.1;
    //
    //    ctx.begin_path();
    //    {
    //        let (x, y) = line.0;
    //        let noise_x = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE,  pos*CHANGE_SPEED]) as f64 * NOISE_RANGE;
    //        let noise_y = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE, -pos*CHANGE_SPEED]) as f64 * NOISE_RANGE;
    //        ctx.move_to(x + noise_x, y + noise_y);
    //    }
    //    //self.ctx.move_to(
    //    //    line.0.0 + self.noise.get([line.0.0/NOISE_SCALE, line.0.1/NOISE_SCALE,  pos*CHANGE_SPEED]) as f64 * NOISE_RANGE,
    //    //    line.0.1 + self.noise.get([line.0.0/NOISE_SCALE, line.0.1/NOISE_SCALE, -pos*CHANGE_SPEED]) as f64 * NOISE_RANGE);
    //
    //    let dist = ((line.1.0 - line.0.0).powf(2.)
    //               +(line.1.1 - line.0.1).powf(2.)).sqrt();
    //    let num  = (dist * RESOLUTION) as i32;
    //
    //    let to_warped_point = |x: f64, y: f64| {
    //        let noise_x = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE,  pos*CHANGE_SPEED]) as f64 * NOISE_RANGE;
    //        let noise_y = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE, -pos*CHANGE_SPEED]) as f64 * NOISE_RANGE;
    //        ctx.line_to(x + noise_x, y + noise_y);
    //    };
    //    for i in 0..num {
    //        let x = line.0.0 + (i as f64/num as f64) * (line.1.0 - line.0.0);
    //        let y = f(x);
    //
    //        to_warped_point(y, x);
    //    }
    //    to_warped_point(line.1.0, line.1.1);
    //    ctx.stroke();
    //    //console::log_1(&JsValue::from_str(&format!("\n")));
    //}

    fn render(&mut self, pos: f64) {
        let (size_w, size_h) = (self.canvas.client_width() as f64, self.canvas.client_height() as f64);
        self.canvas.set_width(size_w as u32);
        self.canvas.set_height(size_h as u32);
        //self.canvas.set_width(self.canvas.client_width() as u32);
        //self.canvas.set_height(self.canvas.client_height() as u32);
        let ctx = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        ctx.set_stroke_style(&JsValue::from_str(&format!("blue")));
        ctx.set_fill_style(  &JsValue::from_str(&format!("green")));

        ctx.clear_rect(0., 0., size_w, size_h);

        for x in (-NOISE_RANGE*RESOLUTION) as i32..((size_w + NOISE_RANGE)*RESOLUTION) as i32 {
            for y in (-NOISE_RANGE*RESOLUTION) as i32..((size_h + NOISE_RANGE)*RESOLUTION) as i32 {
                if self.rng.gen_bool(0.8) {
                    let x = x as f64 / RESOLUTION;
                    let y = y as f64 / RESOLUTION;

                    let noise_x = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE,  pos*CHANGE_SPEED])
                        as f64 * NOISE_RANGE;
                    let noise_y = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE, -pos*CHANGE_SPEED])
                        as f64 * NOISE_RANGE;

                    //let (x, y) = to_warped_loc(x as f64 / RESOLUTION, y as f64 / RESOLUTION);
                    ctx.fill_rect(x + noise_x, y + noise_y, 1., 1.);
                }
            }
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);

    //let ctx = canvas
    //    .get_context("2d")
    //    .unwrap()
    //    .unwrap()
    //    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    //    .unwrap();


    let (size_w, size_h) = (canvas.client_width(), canvas.client_height());
    console::log_1(&JsValue::from_str(&format!("client size: {}, {}", size_w, size_h)));

    let sim = Lines::new(&canvas, NUM_POINTS);

    game_loop(sim, UPDATE_RATE, 0.1, |_| {
        // update fn
    }, |g| {
        g.game.render(g.number_of_updates() as f64 / UPDATE_RATE as f64);
    });

    Ok(())
}
