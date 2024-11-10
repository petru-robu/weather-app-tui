use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame, DefaultTerminal
};

use crate::{
    app::App,
};


pub fn render(app: &mut App, frame: &mut Frame)
{
    let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ])
            .split(frame.area());

    let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(layout[0]);
}