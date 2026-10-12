use gpui::*;
use std::{error::Error, rc::Rc};
use wry::{Rect, WebViewBuilder, dpi};

pub struct WebView {
    webview: Rc<wry::WebView>,
}

impl WebView {
    pub fn new(window: &Window, url: &str) -> Result<Self, Box<dyn Error>> {
        let webview = WebViewBuilder::new().with_url(url).build_as_child(window)?;

        Ok(Self {
            webview: Rc::new(webview),
        })
    }

    pub fn navigate(&self, url: &str) -> Result<(), Box<dyn Error>> {
        self.webview.load_url(url)?;
        Ok(())
    }
}

impl Render for WebView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let webview = Rc::clone(&self.webview);

        canvas(
            move |bounds, _, _| {
                let rect = Rect {
                    position: dpi::Position::Logical(dpi::LogicalPosition::new(
                        bounds.origin.x.into(),
                        bounds.origin.y.into(),
                    )),
                    size: dpi::Size::Logical(dpi::LogicalSize::new(
                        bounds.size.width.into(),
                        bounds.size.height.into(),
                    )),
                };

                if let Err(e) = webview.set_bounds(rect) {
                    eprintln!("Could not position WebView: {}", e);
                }
            },
            |_, _, _, _| {},
        )
        .size_full()
    }
}
