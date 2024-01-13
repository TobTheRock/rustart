use std::cmp::max;

use wasm_bindgen::prelude::*;

use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    drawing::IntoDrawingArea,
    element::PathElement, // Fix: Import the correct module `element::line` instead of `element::Line`
    series::LineSeries,
    style::BLACK,
    style::WHITE,
};
use plotters_canvas::CanvasBackend;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub fn initialize() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Initializing wasm lib!");
}

#[wasm_bindgen]
pub fn ask_deep_thought(question: &str) -> u32 {
    log::info!("Asking deep thought: {}", question);
    let time_to_think_parameter: u64 = question.len().try_into().unwrap_or(0);

    log::info!("DeepThought™ will need to think...");
    for i in 0..time_to_think_parameter {
        let mut x: u64 = 0;
        for j in 0..1_000_000 {
            x += i * j / 42;
        }

        log::info!("... {:#b}", x);
    }

    log::info!("DeepThought™ has awoken!");

    return 42;
}

#[wasm_bindgen]
pub struct PlotCanvas {
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl PlotCanvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        log::info!("Creating PlotCanvas");

        Self { canvas }
    }

    #[wasm_bindgen]
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), JsValue> {
        log::info!("Drawing line");
        let backend = CanvasBackend::with_canvas_object(self.canvas.clone()).unwrap();
        log::info!("Backend created");

        let root = backend.into_drawing_area();
        log::info!("Drawing area created");
        root.fill(&WHITE).unwrap();
        log::info!("Drawing area filled");

        let mut chart = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..max(x1, x2) + 5, 0..max(y1, y2) + 5)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        let plotting_area = chart.plotting_area();
        plotting_area
            .draw(&PathElement::new([(x1, y1), (x2, y2), (80, 20)], &BLACK))
            .unwrap();

        root.present().unwrap();

        Ok(())
    }
}
