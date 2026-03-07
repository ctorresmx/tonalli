use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

use crate::agents::Role;

use super::app::App;

// Shared with mod.rs so the layout split is defined in one place.
pub(super) const HISTORY_PANE_PERCENT: u16 = 80;

const SPINNER: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

const USER_STYLE: Style = Style::new().fg(Color::Cyan);
const USER_HEADER_STYLE: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
const MODEL_STYLE: Style = Style::new().fg(Color::Green);
const MODEL_HEADER_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
const THINKING_STYLE: Style = Style::new().fg(Color::DarkGray);
const DIM_INPUT_STYLE: Style = Style::new().fg(Color::DarkGray);

pub(super) fn compute_max_scroll(app: &App, inner_width: u16, inner_height: u16) -> u16 {
    let width = inner_width.max(1);
    let mut total: u16 = 0;
    for (_, text) in &app.messages {
        total += 1; // role header
        for line in text.lines() {
            let chars = line.chars().count() as u16;
            total += ((chars + width - 1) / width).max(1);
        }
        total += 1; // blank separator
    }
    if app.loading {
        total += 2; // "Model" + spinner line
    }
    total.saturating_sub(inner_height)
}

pub fn render(app: &App, frame: &mut Frame, tick: u8) {
    let chunks = Layout::vertical([
        Constraint::Percentage(HISTORY_PANE_PERCENT),
        Constraint::Percentage(100 - HISTORY_PANE_PERCENT),
    ])
    .split(frame.area());

    render_history(app, frame, chunks[0], tick);
    render_input(app, frame, chunks[1]);
}

fn render_history(app: &App, frame: &mut Frame, area: Rect, tick: u8) {
    let block = Block::bordered().title("Chat History");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = build_history_lines(app, tick);
    let para = Paragraph::new(lines)
        .scroll((app.scroll_offset, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(para, inner);
}

fn build_history_lines<'a>(app: &'a App, tick: u8) -> Vec<Line<'a>> {
    let mut lines: Vec<Line<'a>> = Vec::new();

    for (role, text) in &app.messages {
        match role {
            Role::User => {
                lines.push(Line::from(Span::styled("You", USER_HEADER_STYLE)).right_aligned());
                for line in text.lines() {
                    lines.push(Line::from(Span::styled(line, USER_STYLE)).right_aligned());
                }
            }
            Role::Model => {
                lines.push(Line::from(Span::styled("Model", MODEL_HEADER_STYLE)));
                for line in text.lines() {
                    lines.push(Line::from(Span::styled(line, MODEL_STYLE)));
                }
            }
            _ => {}
        }
        lines.push(Line::default());
    }

    if app.loading {
        let spinner = SPINNER[(tick as usize) % SPINNER.len()];
        lines.push(Line::from(Span::styled("Model", MODEL_HEADER_STYLE)));
        lines.push(Line::from(Span::styled(
            format!("{} Thinking...", spinner),
            THINKING_STYLE,
        )));
    }

    lines
}

fn render_input(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title("Message")
        .title_top(Line::from("[Send]").right_aligned())
        .title_bottom(Line::from("Enter: send | ↑↓/PgUp/PgDn: scroll | Esc: quit").centered());
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let input_style = if app.loading { DIM_INPUT_STYLE } else { Style::new() };
    let para = Paragraph::new(app.input.as_str())
        .style(input_style)
        .wrap(Wrap { trim: false });
    frame.render_widget(para, inner);

    if !app.loading {
        frame.set_cursor_position(cursor_position(app, inner));
    }
}

fn cursor_position(app: &App, inner: Rect) -> Position {
    let before_cursor = &app.input[..app.input_cursor];
    let wrap_width = inner.width.max(1) as usize;
    let mut row = 0u16;
    let mut col = 0usize;

    for ch in before_cursor.chars() {
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
            if col >= wrap_width {
                row += 1;
                col = 0;
            }
        }
    }

    Position {
        x: inner.x + col as u16,
        y: inner.y + row,
    }
}
