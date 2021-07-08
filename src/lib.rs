use noise::{ NoiseFn, SuperSimplex };

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{ console, window, Node };

use derivative::Derivative;

use rand::prelude::{ thread_rng, ThreadRng };
use rand::distributions::{ Distribution, uniform::{ Uniform, UniformFloat } };

use std::iter::repeat;
use std::mem;

const NUM_POINTS: usize = 6; 
const NOISE_SCALE: f64 = 1.;
const RESOLUTION: f64 = 0.2; // points per pixel


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Derivative)]
#[derivative(Debug)]
struct Line((f64, f64), (f64, f64)); // assert self.0.0 < self.1.0
impl Line {
    fn new(mut a: (f64, f64), mut b: (f64, f64)) -> Line {
        if a.0 > b.0 { mem::swap(&mut a, &mut b) }     // assert self.0.0 < self.1.0
        Line { 0: a, 1: b }
    }
    //fn from_distributions(udist_w: Uniform<f64>, udist_h: Uniform<f64>, rng: &mut ThreadRng) -> Line {
    //    let a = (udist_w.sample(&mut rng), udist_h.sample(&mut rng));
    //    let b = (udist_w.sample(&mut rng), udist_h.sample(&mut rng));
    //    if a.0 > b.0 { mem::swap(a, b) }     // assert self.0.0 < self.1.0
    //    Line { a, b }
    //}
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Lines {
    ctx: web_sys::CanvasRenderingContext2d,
    lines: Vec<Line>,
    size_w: f64,
    size_h: f64,
    #[derivative(Debug="ignore")]
    noise: SuperSimplex,
}
impl Lines {
    fn new(canvas: web_sys::HtmlCanvasElement, points: usize) -> Lines {
    //fn new(canvas: web_sys::HtmlCanvasElement, points: usize) {
    //fn new(canvas: web_sys::HtmlCanvasElement, ctx: web_sys::CanvasRenderingContext2d, points: usize) -> Lines {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let (size_w, size_h) = (canvas.client_width() as f64, canvas.client_height() as f64);
        
        let mut rng = thread_rng();
        let udist_w = Uniform::new(0., size_w);
        let udist_h = Uniform::new(0., size_h);
        
        let points = repeat(()).take(points)
            .map(|_| (udist_w.sample(&mut rng), udist_h.sample(&mut rng)))
            .collect::<Vec<(f64, f64)>>();

        let mut lines = Vec::<Line>::new();
        lines.reserve(points.len().pow(2));
        for a in &points {
            for b in &points {
                lines.push(Line::new(*a, *b));
            }
        }

        let noise = SuperSimplex::new();

        Lines { ctx, size_w, size_h, lines, noise }
    }

    fn draw_line(&self, line: &Line, pos: f64) {
        let f = |x: f64| (line.0.1-line.1.0) / (line.0.0-line.1.0) * (x - line.0.0) + line.0.1;

        self.ctx.move_to(line.0.0, line.0.1);
        let dist = ((line.1.0 - line.0.0).powf(2.)
                   +(line.1.1 - line.0.1).powf(2.)).sqrt();
        let num  = (dist * RESOLUTION) as i32;
        for i in 0..num {
            let x = line.0.0 + (i as f64/num as f64) * (line.1.0 - line.0.0);
            let y = f(x);

            //let noise_x = self.simplex.noise_3d(x as f32, y as f32,  pos as f32) as f64 * NOISE_SCALE;
            //let noise_y = self.simplex.noise_3d(x as f32, y as f32, -pos as f32) as f64 * NOISE_SCALE;
            let noise_x = self.noise.get([x, y,  pos]) as f64 * NOISE_SCALE;
            let noise_y = self.noise.get([x, y, -pos]) as f64 * NOISE_SCALE;

            self.ctx.line_to(x + noise_x, y + noise_y);
        }
        //for x_int in 0..((line.1.0 - line.0.0) as f64 *RESOLUTION) as i32 {
        //    let x = line.0.0 + x_int as f64 / RESOLUTION;
        //    self.ctx.line_to(line.0.0 + x, line.0.1 + f(x)); // TODO: add noise
        //}
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
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let (size_w, size_h) = (canvas.client_width(), canvas.client_height());
    let sim = Lines::new(canvas, NUM_POINTS);
    //let sim = Lines::new(canvas, ctx, NUM_POINTS);

    console::log_1(&JsValue::from_str(&format!("client size: {}, {}", size_w, size_h)));

    Ok(())
}
