//! Renderer module. Provides infrastructure to render the game scene to the canvas.
//!
//! Uses the Canvas2D drawing API, an operates directly on the 2d drawing context from the DOM API.
//!
//! The renderer is based on a *Pipeline State Object* design. You create a PSO for different drawing
//! styles (contains stuff like draw color, font etc...), and bind it to the drawing context before
//! drawing the actual scene objects. This is done to simplify usage of the statefull drawing API
//! provides by the Canvas 2d drawing context.
//!
//! TODO: Improve the PSO Api. E.g by creating a scene API, which takes scene objects + their assigned
//! PSO for every drawing step, and uses this data to automatically sort and optimize out PSO bindings. This
//! can be extended by making the PSO smart, when they are rebound (only setting state, which changed for the
//! drawing context to avoid resetting every single property).

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// An abstraction over the **CanvasRenderingContext2d**. Thus is essentially all the state, which the game needs for
/// drawing the scene (except the input from the **update()** function from the **Game** object).
pub struct PixelScreen {
    canvas_ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl PixelScreen {
    /// Create a new PXS by creating a drawing cotext atop the **HtmlCanvasElement**.
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        // **get_context()** returns a untyped **JsValue**. **dyn_into::<T>()** performs an
        // upcast into the real type. (Can error when the type does not match)
        // --------------------------------------------------------------------------------
        let ctx = canvas.get_context("2d").unwrap();
        let ctx = ctx.unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

        Self {
            canvas_ctx: ctx,
            width: canvas.width(),
            height: canvas.height(),
        }
    }

    /// Draw a list of **Renderable** trait object references in the order they are supplied.
    pub fn draw(&self, display_list: &[&dyn Renderable]) {
        for renderable in display_list {
            renderable.draw(self);
        }
    }

    /// Draw text at *pos* helper.
    pub fn draw_text(&self, text: &str, pos: Pos) {
        self.canvas_ctx.fill_text(text, pos.x, pos.y).unwrap();
    }

    /// Draw rectangle at *pos* with *width* and *height* helper.
    pub fn draw_rect(&self, pos: Pos, width: f64, height: f64) {
        self.canvas_ctx.fill_rect(pos.x, pos.y, width, height);
    }

    /// Clear the screen helper.
    pub fn clear(&self) {
        self.canvas_ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }
}

/// Css color type. Used as a type, which can be supplied to DOM API by calling **into()**.
/// 
/// TODO: This type is convenient, but has a real performance impact, because every time it
/// is used as a property in a PSO, before binding it has to be turned into a Css property, 
/// which incurs an extra string buffer allocation, formatter API call, and JsValue creation.
/// Creating a caching system for that kind of stuff should be a good way to go.
#[derive(Copy, Clone, PartialEq)]
pub struct CssColor {
    r: u8,
    g: u8,
    b: u8,
}

impl CssColor {
    /// Create new color by rgb args.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Into<JsValue> for CssColor {
    /// Convert color into a usable CSS property string (JsValue).
    /// 
    /// TODO: Caching system.
    fn into(self) -> JsValue {
        JsValue::from(format!("rgb({},{},{})", self.r, self.g, self.b))
    }
}

/// Css font helper. Use it to create a new font, and call **into()** to convert it into a String
/// Css property string.
/// 
/// TODO: Also cache this stuff.
#[derive(Copy, Clone, PartialEq)]
pub struct CssFont {
    size: u32,
    family: &'static str,
}

impl CssFont {
    /// New font by *family* and *size* in px.
    pub fn new(size: u32, family: &'static str) -> Self {
        Self { size, family }
    }

    /// monospace font helper with *size* in px.
    pub fn monospace(size: u32) -> Self {
        Self::new(size, "Courier New")
    }
}

impl Into<String> for CssFont {
    /// Turn the font into a usable css property string.
    ///
    /// TODO: Use caching system.
    fn into(self) -> String {
        format!("{}px {}", self.size, self.family)
    }
}

/// Position on screen type. Uses floats.
#[derive(Copy, Clone)]
pub struct Pos {
    x: f64,
    y: f64,
}

impl Pos {
    /// Create a new position.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Pipeline state object is used to bind to **PixelScreen**. On binding it will set all the required state
/// of the canvas 2d rendering context.
#[derive(PartialEq, Default)]
pub struct Pso {
    pub fill_color: Option<CssColor>,
    pub font: Option<CssFont>,
}

impl Pso {
    /// Bind to **PixelScreen**.
    /// Will only bind properties, which are `Some(_)`. Others are left as they were before.
    pub fn bind(&self, pxs: &PixelScreen) {
        if let Some(c) = self.fill_color {
            pxs.canvas_ctx.set_fill_style(&c.into());
        }

        if let Some(f) = self.font {
            let s: String = f.into();
            pxs.canvas_ctx.set_font(&s);
        }
    }
}

/// Trait for renderable types.
pub trait Renderable {
    /// This will draw the current object to the given **PixelScreen**.
    ///
    /// Note: PSO usage is not specified by **Renderable**. This method may rebind another or even multiple PSOs,
    /// but it does not have to.
    fn draw(&self, pxs: &PixelScreen);
}
