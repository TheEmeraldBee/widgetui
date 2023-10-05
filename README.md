# Widgetui
Turn
<details>
<summary>Ratatui Minimal</summary>

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World!");
            frame.render_widget(greeting, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
}
```
</details>

Into

<details>
<summary>Much Better</summary>

```rust
use ratatui::widgets::Paragraph;

use widgetui::*;

use std::{cell::RefMut, error::Error};

fn widget(frame: &mut WidgetFrame, mut events: RefMut<Events>) -> WidgetResult {
    frame.render_widget(Paragraph::new("Hello, world!", frame.size()));

    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?.widgets(widget).run()
}
```
</details>

The goal of this project is to simplify the requirements to make a good project using tui.
It removes boilerplate, and improves the developer experience by using the power of typemaps, and dependency injection

# Installation
Run the following within your project directory
```bash
cargo add widgetui
```
# Introduction

Widgetui is a wrapper over Ratatui's Crossterm backend which allows for powerful abstraction, and simplifies creating a good app within Ratatui.
## Why pick this over Ratatui?
Widgetui isn't meant to replace or undermine Ratatui. It is simply a wrapper. Without Ratatui, this crate would not exist, as well, you will still require Ratatui and Crossterm crates just to work with the apps.

**TLDR; Don't, use both together to improve developer experience, and build your apps faster!**

# Quickstart
```rust
use ratatui::widgets::Paragraph;

use widgetui::*;

use std::{cell::RefMut, error::Error};

fn widget(frame: &mut WidgetFrame, mut events: RefMut<Events>) -> WidgetResult {
    frame.render_widget(Paragraph::new("Hello, world!", frame.size()));

    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?.widgets(widget).run()
}
```

The above will create an application that will display an empty terminal window, then close once you press `q`.

This application, with many less lines, will render the same thing that Ratatui's Quickstart renders.
# Documentation
Documentation can be found on [docs.rs](https://docs.rs/widgetui).
Need help? Check the [wiki](https://github.com/TheEmeraldBee/widgetui/wiki)!

# Fun Facts
- I chose `WidgetFrame` because if I just used `Widget`, then you couldn't do the awesome
```rust
use widgetui::*;
use widgetui::ratatui::prelude::*;
```

- It took about 10 hours to get this project initially set up!
	- Much longer after I decided to add bevy system like widget methods.

- You used to have to take in a States struct, but in order to fix it, there is a lot of behind the scenes things going on!

- You can only use 11 states in the same widget!
	- If you need more, please add an issue with details on why, and we may consider adding access to more at once!
