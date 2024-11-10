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

#[derive(Debug, Default)]
pub struct App 
{
    /// Is the application running?
    running: bool,
}

impl App 
{
    /// Construct a new instance of [`App`].
    pub fn new() -> Self 
    {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> 
    {
        self.running = true;

        while self.running 
        {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    //Renders the user interface
    fn draw(&mut self, frame: &mut Frame) 
    {
        
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> Result<()> 
    {
        match event::read()? 
        {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) 
    {
        match (key.modifiers, key.code) 
        {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Quit the app
    fn quit(&mut self) 
    {
        self.running = false;
    }
}