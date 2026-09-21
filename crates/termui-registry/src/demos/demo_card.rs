use ratatui::layout::Rect;
use ratatui::Frame;
use termui_renderer::render_frame;
use termui_widgets::termui;

const CARD_DEMO_HEIGHT: u16 = 9;

fn demo_card(frame: &mut Frame<'_>, area: Rect) {
    termui! {
        frame: frame,
        area: area,
        Card {
            CardHeader {
                CardTitle { "Card Title" }
                CardDescription { "Card Description" }
            }
            CardContent {
                p { "Card Content" }
            }
            CardFooter {
                p { "Card Footer" }
            }
        }
    }
}

pub(super) fn render() -> Vec<String> {
    render_frame(CARD_DEMO_HEIGHT, |frame| {
        demo_card(frame, frame.area());
    })
}
