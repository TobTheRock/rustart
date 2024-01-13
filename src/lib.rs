use wasm_bindgen::prelude::*;

use plotters::{
    chart::ChartBuilder, drawing::IntoDrawingArea, series::LineSeries, style::BLACK, style::WHITE,
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
    pub fn draw_line(&self) -> Result<(), JsValue> {
        log::info!("Drawing line");
        let backend = CanvasBackend::with_canvas_object(self.canvas.clone()).unwrap();
        log::info!("Backend created");

        let root = backend.into_drawing_area();
        log::info!("Drawing area created");
        root.fill(&WHITE).unwrap();
        log::info!("Drawing area filled");

        let mut chart = ChartBuilder::on(&root)
            .build_cartesian_2d(-10..10, -10..10)
            .unwrap();

        chart
            .configure_mesh()
            // .disable_x_mesh()
            // .disable_y_mesh()
            .draw()
            .unwrap();

        let plotting_area = chart.plotting_area();

        chart
            .draw_series(LineSeries::new((-10..=10).map(|x| (x, x)), &BLACK))
            .unwrap();

        root.present().unwrap();

        Ok(())
    }
}

#[wasm_bindgen]
pub fn plot(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    log::info!("Generating Canvas");

    todo!();
}
