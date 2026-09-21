use termui_renderer::render_frame;
use termui_widgets::breadcrumbs::Breadcrumbs;

pub(super) fn render() -> Vec<String> {
    let items = ["Home", "Projects", "TermUI"];
    render_frame(1, |frame| {
        frame.render_widget(Breadcrumbs::new(&items).line(), frame.area());
    })
}
