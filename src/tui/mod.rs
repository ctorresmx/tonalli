use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use futures_util::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::mpsc;
use tokio::task::{LocalSet, spawn_local};

use crate::agents::{Agent, Chat};

mod app;
mod ui;

use app::App;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

pub async fn run<A: Agent + 'static>(chat: Chat<A>) -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let (tx_send, mut rx_send) = mpsc::channel::<String>(32);
    let (tx_resp, mut rx_resp) = mpsc::channel::<Result<String, String>>(32);

    let local = LocalSet::new();

    local
        .run_until(async move {
            // Dropping tx_send when this block ends signals rx_send that no more
            // messages are coming, causing the background task to exit cleanly.
            spawn_local(async move {
                let mut chat = chat;
                while let Some(text) = rx_send.recv().await {
                    let result = chat.send(&text).await.map_err(|e| e.to_string());
                    if tx_resp.send(result).await.is_err() {
                        break;
                    }
                }
            });

            let mut app = App::new();
            let mut tick: u8 = 0;
            let mut event_reader = EventStream::new();

            loop {
                // Clamp scroll offset based on current terminal size
                let size = terminal.size()?;
                let inner_w = size.width.saturating_sub(2);
                let inner_h = ((size.height as u32 * ui::HISTORY_PANE_PERCENT as u32 / 100) as u16).saturating_sub(2);
                let max_scroll = ui::compute_max_scroll(&app, inner_w, inner_h);
                app.scroll_offset = app.scroll_offset.min(max_scroll);

                terminal.draw(|f| ui::render(&app, f, tick))?;
                tick = tick.wrapping_add(1);

                if app.should_quit {
                    break;
                }

                tokio::select! {
                    maybe_event = event_reader.next() => {
                        if let Some(Ok(Event::Key(key))) = maybe_event {
                            match (key.code, key.modifiers) {
                                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                                | (KeyCode::Esc, _) => {
                                    app.should_quit = true;
                                }
                                (KeyCode::Enter, _) if !app.loading => {
                                    let text = app.send_message();
                                    if !text.trim().is_empty() {
                                        app.push_user(text.clone());
                                        let _ = tx_send.send(text).await;
                                        app.loading = true;
                                        app.scroll_to_bottom();
                                    }
                                }
                                (KeyCode::Backspace, _) => {
                                    app.delete_char();
                                }
                                (KeyCode::Left, _) => {
                                    app.move_cursor_left();
                                }
                                (KeyCode::Right, _) => {
                                    app.move_cursor_right();
                                }
(KeyCode::Char(c), _) => {
                                    app.insert_char(c);
                                }
                                _ => {}
                            }
                        }
                    }
                    maybe_resp = rx_resp.recv() => {
                        match maybe_resp {
                            Some(Ok(text)) => {
                                app.push_model(text);
                                app.loading = false;
                                app.scroll_to_bottom();
                            }
                            Some(Err(e)) => {
                                app.push_model(format!("[Error: {}]", e));
                                app.loading = false;
                            }
                            None => {}
                        }
                    }
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        // periodic tick for spinner animation
                    }
                }
            }

            Ok::<(), Box<dyn Error>>(())
        })
        .await?;

    Ok(())
}
