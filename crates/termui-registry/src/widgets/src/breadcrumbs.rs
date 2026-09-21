use ratatui::text::{Line, Span};

/// A compact path through nested views.
#[must_use]
pub struct Breadcrumbs<'a> {
    items: &'a [&'a str],
    separator: &'a str,
}

impl<'a> Breadcrumbs<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self {
            items,
            separator: "/",
        }
    }

    pub fn separator(mut self, separator: &'a str) -> Self {
        self.separator = separator;
        self
    }

    pub fn line(&self) -> Line<'a> {
        let mut spans = Vec::new();
        for (index, item) in self.items.iter().enumerate() {
            if index > 0 {
                spans.push(Span::raw(format!(" {} ", self.separator)));
            }
            spans.push(Span::raw(*item));
        }
        Line::from(spans)
    }
}
