use game_loop::game_loop;
use noise::{ NoiseFn, SuperSimplex };

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{ console, window, Node };

use derivative::Derivative;

use rand::prelude::{ thread_rng, ThreadRng };
use rand::distributions::{ Distribution, uniform::{ Uniform, UniformFloat } };

use std::iter::repeat;
use std::mem;

const UPDATE_RATE: u32 = 12; // updates per second

const NUM_POINTS: usize = 6; 
const NOISE_RANGE: f64 = 100.;
const NOISE_SCALE: f64 = 300.;
const RESOLUTION: f64 = 0.05; // points per pixel


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
    points: Vec<(f64, f64)>,
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
        ctx.set_stroke_style(&JsValue::from_str(&format!("blue")));
        ctx.set_fill_style(  &JsValue::from_str(&format!("green")));

        let (size_w, size_h) = (canvas.client_width() as f64, canvas.client_height() as f64);
        //canvas.set_height(size_h as u32);
        
        let mut rng = thread_rng();
        let udist_w = Uniform::new(0., size_w);
        let udist_h = Uniform::new(0., size_h);
        
        let points = repeat(()).take(points)
            .map(|_| (udist_w.sample(&mut rng), udist_h.sample(&mut rng)))
            .collect::<Vec<(f64, f64)>>();

        //for point in &points {
        //    console::log_1(&JsValue::from_str(&format!("point at {}, {}", point.0, point.1)));
        //}

        let mut lines = Vec::<Line>::new();
        lines.reserve(points.len().pow(2));
        for a in &points {
            for b in &points {
                lines.push(Line::new(*a, *b));
            }
        }

        let noise = SuperSimplex::new();

        Lines { ctx, size_w, size_h, lines, points, noise }
    }

    fn draw_line(&self, line: &Line, pos: f64) {
        let f = |x: f64| (line.0.1-line.1.1) / (line.0.0-line.1.0) * (x - line.0.0) + line.0.1;

        self.ctx.move_to(line.0.0, line.0.1);
        let dist = ((line.1.0 - line.0.0).powf(2.)
                   +(line.1.1 - line.0.1).powf(2.)).sqrt();
        let num  = (dist * RESOLUTION) as i32;
        for i in 0..num {
            let x = line.0.0 + (i as f64/num as f64) * (line.1.0 - line.0.0);
            let y = f(x);
            //console::log_1(&JsValue::from_str(&format!("line {}..{}: {}", line.0.0, line.1.0, x)));

            let noise_x = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE,  pos]) as f64 * NOISE_RANGE;
            let noise_y = self.noise.get([x/NOISE_SCALE, y/NOISE_SCALE, -pos]) as f64 * NOISE_RANGE;
            //let noise_x = 0.;
            //let noise_y = 0.;

            self.ctx.line_to(x + noise_x, y + noise_y);
        }
        //for x_int in 0..((line.1.0 - line.0.0) as f64 *RESOLUTION) as i32 {
        //    let x = line.0.0 + x_int as f64 / RESOLUTION;
        //    self.ctx.line_to(line.0.0 + x, line.0.1 + f(x)); // TODO: add noise
        //}
        self.ctx.line_to(line.1.0, line.1.1);
        self.ctx.stroke();
        //console::log_1(&JsValue::from_str(&format!("\n")));
    }
    fn render(&self, pos: f64) {
        self.ctx.fill_rect(0., 0., self.size_w-10., self.size_h-10.);
        //self.ctx.clear_rect(0., 0., self.size_w, self.size_h);
        for line in &self.lines {
            self.draw_line(line, pos);
        }

        for point in &self.points {
            self.ctx.fill_rect(point.0 - 5., point.1 - 5., 10., 10.);
        }
        //console::log_1(&JsValue::from_str(&format!("herro after {}", pos)));
        //let window = web_sys::window().unwrap();
        //let document = window.document().unwrap();
        //let canvas = document.get_element_by_id("canvas").unwrap();
        //let canvas: web_sys::HtmlCanvasElement = canvas
        //    .dyn_into::<web_sys::HtmlCanvasElement>()
        //    .map_err(|_| ())
        //    .unwrap();
        //console::log_1(&JsValue::from_str(&format!("client size: {}, {}", canvas.width(), canvas.height())));
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

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    let (size_w, size_h) = (canvas.client_width(), canvas.client_height());
    console::log_1(&JsValue::from_str(&format!("client size: {}, {}", size_w, size_h)));

    let sim = Lines::new(canvas, NUM_POINTS);

    game_loop(sim, UPDATE_RATE, 0.1, |g| {
        // update fn
    }, |g| {
        g.game.render(g.number_of_updates() as f64 / UPDATE_RATE as f64);
    });

    Ok(())
}
