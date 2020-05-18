use crate::actions::get_file_lines;
use crate::util::event::{Event, Events};
use chrono::{prelude::*, Duration};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
  backend::{Backend, TermionBackend},
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  terminal::Frame,
  widgets::{Block, Borders, List, ListState, Paragraph, Text},
  Terminal,
};

struct App {
  // `items` is the state managed by your application.
  items: Vec<String>,
  // `state` is the state that can be modified by the UI. It stores the index of the selected
  // item as well as the offset computed during the previous draw call (used to implement
  // natural scrolling).
  state: ListState,
  current_date: Date<Local>,
}

impl App {
  fn new(items: Vec<String>) -> App {
    App {
      items,
      state: ListState::default(),
      current_date: Local::today(),
    }
  }

  // pub fn set_items(&mut self, items: Vec<String>) {
  //   self.items = items;
  //   self.state = ListState::default();
  //   self.current_date = Local::today();
  // }

  pub fn previous_date(&mut self) {
    self.current_date = self.current_date - Duration::days(30);
    let date_str = format!(
      "{}-{:2>0}",
      self.current_date.year(),
      self.current_date.month()
    );
    self.items = get_file_lines(Some(date_str));
  }

  pub fn next_date(&mut self) {
    self.current_date = self.current_date + Duration::days(30);
    let date_str = format!(
      "{}-{:2>0}",
      self.current_date.year(),
      self.current_date.month()
    );
    self.items = get_file_lines(Some(date_str));
  }

  // Select the next item. This will not be reflected until the widget is drawn in the
  // `Terminal::draw` callback using `Frame::render_stateful_widget`.
  pub fn next(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }

  // Select the previous item. This will not be reflected until the widget is drawn in the
  // `Terminal::draw` callback using `Frame::render_stateful_widget`.
  pub fn previous(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }

  // Unselect the currently selected item if any. The implementation of `ListState` makes
  // sure that the stored offset is also reset.
  // pub fn unselect(&mut self) {
  //   self.state.select(None);
  // }
}

pub fn render_ui() -> Result<(), Box<dyn Error>> {
  // Terminal initialization
  let stdout = io::stdout().into_raw_mode()?;
  let stdout = MouseTerminal::from(stdout);
  let stdout = AlternateScreen::from(stdout);
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.hide_cursor()?;

  // Setup event handlers
  let events = Events::new();
  let mut app = App::new(get_file_lines(None));
  // By default select the first element
  app.next();
  loop {
    terminal.draw(|mut f| {
      // Create chunks to identify specific screen spaces
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
          [
            Constraint::Percentage(15),
            Constraint::Percentage(75),
            Constraint::Percentage(10),
          ]
          .as_ref(),
        )
        .split(f.size());

      // Draw header
      draw_header(&mut f, chunks[0]);

      // Render list
      let title = format!(
        "{}-{:0>2}",
        app.current_date.year(),
        app.current_date.month()
      );
      let style = Style::default().fg(Color::Gray);
      let items = app.items.iter().map(Text::raw);
      let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(&title))
        .style(style)
        .highlight_style(
          style
            .fg(Color::White)
            .bg(Color::LightBlue)
            .modifier(Modifier::BOLD),
        );
      f.render_stateful_widget(list, chunks[1], &mut app.state);
    })?;

    if let Event::Input(key) = events.next()? {
      if key == Key::Up {
        app.previous();
      }
      if key == Key::Down {
        app.next();
      }
      if key == Key::Left {
        app.previous_date();
      }
      if key == Key::Right {
        app.next_date();
      }
      if key == Key::Char(':') {
        break;
      }
      if key == Key::Char('q') {
        break;
      }
    }
  }
  Ok(())
}

fn draw_header<B>(f: &mut Frame<B>, layout_chunk: Rect)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(50),
      ]
      .as_ref(),
    )
    .split(layout_chunk);

  let current_date = Local::now();
  // Render infos
  let infos = [
    Text::styled("Current date: ", Style::default().fg(Color::Yellow)),
    Text::raw(format!(
      "{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
      current_date.year(),
      current_date.month(),
      current_date.day(),
      current_date.hour(),
      current_date.minute(),
      current_date.second()
    )),
  ];

  let block = Block::default()
    .borders(Borders::NONE)
    .title_style(Style::default());
  let paragraph = Paragraph::new(infos.iter())
    .block(block.clone())
    .alignment(Alignment::Left);

  f.render_widget(paragraph, chunks[0]);

  // Render shortcuts
  let shortcuts = [
    Text::styled("<a>  ", Style::default().fg(Color::Blue)),
    Text::raw("Add time\n"),
    Text::styled("<d>  ", Style::default().fg(Color::Blue)),
    Text::raw("Remove time\n"),
    Text::styled("<→>  ", Style::default().fg(Color::Blue)),
    Text::raw("Next month\n"),
    Text::styled("<←>  ", Style::default().fg(Color::Blue)),
    Text::raw("Previous month\n"),
    Text::styled("<q>  ", Style::default().fg(Color::Blue)),
    Text::raw("Quit\n"),
  ];

  let block = Block::default()
    .borders(Borders::NONE)
    .title_style(Style::default());
  let paragraph = Paragraph::new(shortcuts.iter())
    .block(block.clone())
    .alignment(Alignment::Left);

  f.render_widget(paragraph, chunks[1]);
  // Render logo
  let text = [
    Text::raw(" ______ __   __  ____  \n"),
    Text::raw("|______|  \\_/  |/ ___| \n"),
    Text::raw("  |  | | |\\_/| | |  _  \n"),
    Text::raw("  |  | | |   | | |_| | \n"),
    Text::raw("  |__| |_|   |_|\\____| \n"),
    Text::raw("\n"),
    Text::styled("v0.1   \n", Style::default().fg(Color::Red)),
  ];

  // Render header
  let block = Block::default()
    .borders(Borders::NONE)
    .title_style(Style::default().modifier(Modifier::BOLD));
  let paragraph = Paragraph::new(text.iter())
    .block(block.clone())
    .alignment(Alignment::Right);

  f.render_widget(paragraph, chunks[2]);
}
