use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A month grid with an optional highlighted day.
#[must_use]
pub struct Calendar {
    year: i32,
    month: u8,
    selected_day: Option<u8>,
}

impl Calendar {
    pub fn new(year: i32, month: u8) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            selected_day: None,
        }
    }

    pub fn selected_day(mut self, day: u8) -> Self {
        self.selected_day = Some(day);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let first_weekday = weekday(self.year, self.month);
        let days = days_in_month(self.year, self.month);
        let mut lines = vec![
            Line::from(format!("{} {}", MONTHS[self.month as usize - 1], self.year)),
            Line::from("Su Mo Tu We Th Fr Sa"),
        ];
        let mut day = 1;
        for week in 0..6 {
            let mut spans = Vec::new();
            for weekday_index in 0..7 {
                if (week == 0 && weekday_index < first_weekday) || day > days {
                    spans.push(Span::raw("   "));
                } else {
                    let text = format!("{day:>2} ");
                    let style = if self.selected_day == Some(day) {
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    };
                    spans.push(Span::styled(text, style));
                    day += 1;
                }
            }
            lines.push(Line::from(spans));
            if day > days {
                break;
            }
        }
        frame.render_widget(Paragraph::new(lines), area);
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn weekday(year: i32, month: u8) -> usize {
    const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = year - i32::from(month < 3);
    (year + year / 4 - year / 100 + year / 400 + OFFSETS[month as usize - 1] + 1).rem_euclid(7)
        as usize
}
