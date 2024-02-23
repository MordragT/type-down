use crate::{DynHtmlElement, HtmlRender};

#[derive(Debug)]
pub struct HtmlStack {
    stack: Vec<DynHtmlElement>,
}

impl HtmlStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn start(&mut self, element: impl Into<DynHtmlElement>) -> usize {
        let pos = self.stack.len();
        self.stack.push(element.into());
        pos
    }

    pub fn end(&mut self, start: usize) -> DynHtmlElement {
        let mut iter = self.stack.drain(start..).map(Into::into);
        let el: DynHtmlElement = iter.next().unwrap();
        let el = iter.fold(el, |accu, el| accu.child(el));
        el
    }

    pub fn add_child(&mut self, child: impl HtmlRender + 'static) {
        let item = self.stack.last_mut().unwrap();
        item.add_child(child);
    }

    pub fn fold(&mut self, start: usize) {
        let el = self.end(start);
        self.add_child(el);
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
