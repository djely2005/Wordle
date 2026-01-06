use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use crate::modules::{config::Config, revelation::Revelation, state::State};
#[derive(Default)]
pub struct Game {
    config: Config,
    title: String,
    guess: String,
    revelations: Option<Vec<Vec<Revelation>>>,
    win_state: bool,
    finished: bool,
    attempt: usize,
    exit: bool,
}

const FILE_PATH: &str = "./valid-wordle-words.txt";
const WORDS: &str = include_str!("../../valid-wordle-words.txt");

impl Game {
    fn new(config: Config) -> Game {
        Game {
            config,
            title: String::from("Wordle"),
            ..Default::default()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_end();
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn start() -> Game {
        let config = Config::new(FILE_PATH.to_string(), WORDS.to_string());
        Game::new(config)
    }
    fn handle_end(&mut self) {
        if self.attempt == 5 {
            self.finished = true;
            self.finish();
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Esc, _) => self.exit(),
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => *self = Game::start(),
            _ => {}
        }
        if self.finished {
            return;
        }
        match key_event.code {
            KeyCode::Char(c) => self.add_char(c),
            KeyCode::Delete | KeyCode::Backspace => self.remove_char(),
            KeyCode::Enter => self.enter_guess(),
            _ => {}
        }
    }

    fn add_char(&mut self, c: char) {
        if self.guess.as_bytes().len() < 5 {
            self.guess.push(c);
        }
    }

    fn remove_char(&mut self) {
        if self.guess.as_bytes().len() > 0 {
            self.guess.pop();
        }
    }

    fn clear_guess(&mut self) {
        self.guess.clear();
    }
    fn enter_guess(&mut self) {
        if self.guess.as_bytes().len() != 5 {
            self.title = String::from(" Word must be of 5 letters ");
            return;
        }
        if !self.config.word_exists(&self.guess) {
            self.title = String::from(" Word doesn't exist in dectionary ");
            return;
        }
        let revelation = self.config.check(&self.guess);
        if Game::check_game_over(&revelation) {
            self.win_state = true;
            self.finished = true;
            self.finish();
        }
        match &mut self.revelations {
            Some(revelations) => revelations.push(revelation),
            None => self.revelations = Some(vec![revelation]),
        }
        self.attempt += 1;
        self.clear_guess();
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn check_game_over(revelation: &Vec<Revelation>) -> bool {
        revelation.iter().all(|f| f.state == State::Correct)
    }
    fn finish(&mut self) {
        if !self.finished {
            return;
        }
        if self.win_state {
            self.title = String::from(" Congratulation ");
        } else {
            self.title = format!(" The true word was : {} ", self.config.chosen_word);
        }
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" {} ", self.title).bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Esc> ".blue().bold(),
            " - ".bold(),
            " Restart ".into(),
            "<Ctrl-r> ".red().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let mut guess_revelations =
            vec![Line::from(format!(" {} possible words", "0".to_string()))];
        if let Some(revelations) = &self.revelations {
            for revelation in revelations {
                let mut revelation_display: Vec<_> = vec![];
                for r in revelation {
                    match r.state {
                        State::Wrong => {
                            revelation_display.push(Span::from(r.letter.red().bold()));
                        }
                        State::Change => {
                            revelation_display.push(Span::from(r.letter.yellow().bold()));
                        }
                        State::Correct => {
                            revelation_display.push(Span::from(r.letter.green().bold()));
                        }
                    }
                }
                guess_revelations.push(Line::from(revelation_display));
            }
        }
        let current_guess = Line::from(&self.guess[..]);
        guess_revelations.push(current_guess);

        Paragraph::new(guess_revelations)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
