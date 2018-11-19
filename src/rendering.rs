use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct PixelScreen {
    canvas_ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl PixelScreen {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx = canvas.get_context("2d").unwrap();
        let ctx = ctx.unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

        Self {
            canvas_ctx: ctx,
            width: canvas.width(),
            height: canvas.height(),
        }
    }

    pub fn draw(&mut self, display_list: &[&dyn Renderable]) {
        for renderable in display_list {
            renderable.draw(self);
        }
    }

    pub fn draw_text(&mut self, text: &str, pos: Pos) {
        self.canvas_ctx.fill_text(text, pos.x, pos.y).unwrap();
    }

    pub fn draw_rect(&mut self, pos: Pos, width: f64, height: f64) {
        self.canvas_ctx.fill_rect(pos.x, pos.y, width, height);
    }

    pub fn clear(&mut self) {
        self.canvas_ctx
            .set_fill_style(&CssColor::new(255, 255, 255).into());
        self.canvas_ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct CssColor {
    r: u8,
    g: u8,
    b: u8,
}

impl CssColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Into<JsValue> for CssColor {
    fn into(self) -> JsValue {
        JsValue::from(format!("rgb({},{},{})", self.r, self.g, self.b))
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct CssFont {
    size: u32,
    family: &'static str,
}

impl CssFont {
    pub fn new(size: u32, family: &'static str) -> Self {
        Self { size, family }
    }

    pub fn monospace(size: u32) -> Self {
        Self::new(size, "Courier New")
    }
}

impl ToString for CssFont {
    fn to_string(&self) -> String {
        format!("{}px {}", self.size, self.family)
    }
}

#[derive(Copy, Clone)]
pub struct Pos {
    x: f64,
    y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Default)]
pub struct Pso {
    pub fill_color: Option<CssColor>,
    pub font: Option<CssFont>,
}

impl Pso {
    pub fn bind(&self, pxs: &mut PixelScreen) {
        if let Some(c) = self.fill_color {
            pxs.canvas_ctx.set_fill_style(&c.into());
        }

        if let Some(f) = self.font {
            pxs.canvas_ctx.set_font(&f.to_string());
        }
    }
}

pub trait Renderable {
    fn draw(&self, pxs: &mut PixelScreen);
}