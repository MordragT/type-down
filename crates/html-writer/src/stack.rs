use crate::DynHtmlElement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlStack {
    stack: Vec<HtmlLock>,
}

impl HtmlStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, lock: HtmlLock) {
        self.stack.push(lock);
    }

    pub fn push_open(&mut self, element: impl Into<DynHtmlElement>) -> usize {
        let lock = HtmlLock::new(element.into());
        let pos = self.stack.len();
        self.stack.push(lock);
        pos
    }

    pub fn push_close(&mut self, element: impl Into<DynHtmlElement>) -> usize {
        let lock = HtmlLock::Close(element.into());
        let pos = self.stack.len();
        self.stack.push(lock);
        pos
    }

    pub fn add_content(&mut self, content: impl AsRef<str>) {
        let dest = self
            .stack
            .iter_mut()
            .rev()
            .find_map(|lock| {
                if let HtmlLock::Open {
                    element: _,
                    content,
                } = lock
                {
                    Some(content)
                } else {
                    None
                }
            })
            .unwrap();

        dest.push_str(content.as_ref());
    }

    pub fn fold(&mut self, at: usize) -> DynHtmlElement {
        let mut iter = self.stack.drain(at..).map(Into::into);
        let el: DynHtmlElement = iter.next().unwrap();
        let el = iter.fold(el, |accu, el| accu.child(el));
        el
    }

    pub fn fold_push(&mut self, at: usize) {
        let el = self.fold(at);
        self.push_close(el);
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlLock {
    Open {
        element: DynHtmlElement,
        content: String,
    },
    Close(DynHtmlElement),
}

impl HtmlLock {
    pub fn new(element: DynHtmlElement) -> Self {
        Self::Open {
            element,
            content: String::new(),
        }
    }
}

impl From<DynHtmlElement> for HtmlLock {
    fn from(value: DynHtmlElement) -> Self {
        Self::new(value)
    }
}

impl Into<DynHtmlElement> for HtmlLock {
    fn into(self) -> DynHtmlElement {
        match self {
            HtmlLock::Open { element, content } => element.child(content),
            HtmlLock::Close(element) => element,
        }
    }
}
