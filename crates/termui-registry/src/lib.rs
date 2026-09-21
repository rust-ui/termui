mod demos;

pub fn render_demo(name: &str) -> Option<Vec<String>> {
    demos::render(name)
}

pub fn registered_demo_names() -> impl Iterator<Item = &'static str> {
    demos::names()
}
