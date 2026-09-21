use std::time::Duration;

use termui_renderer::render_frame;
use termui_widgets::toast::Toast;

pub(super) fn render() -> Vec<String> {
    render_frame(5, |frame| {
        let mut toast = Toast::error(std::io::Error::other(
            "Theme preferences could not be saved",
        ));
        toast.tick(Duration::from_millis(160));
        toast.render(frame, frame.area());
    })
}
