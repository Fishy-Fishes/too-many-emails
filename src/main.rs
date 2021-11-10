use std::io::{self, Write};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    stdout.write_all(b"something").unwrap();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let _hi = 4384;

    terminal.clear().unwrap();

    // loop {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        let _size = f.size();

        let block = Block::default()
            .title("Inbox")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let items = [
            ListItem::new("Heyo"),
            ListItem::new("Heyo"),
            ListItem::new("Heyo"),
        ];

        let list = List::new(items).block(block);

        f.render_widget(list, chunks[0]);
        okffaefm();

        let block2 = Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        f.render_widget(block2, chunks[1]);
    })?;

    okffaefm();

    // }
    Ok(())
}

fn okffaefm() {
    println!("a");

    println!("heo");
}

pub(crate) fn foo() {
    println!("b");

    println!("c");
}
