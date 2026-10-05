# RataTUI
https://ratatui.rs

```sh
cargo add ratatui crossterm
```

# Terminal Under the Hood
https://yakout.me/blog/terminal-under-the-hood/

pic

TTY is serial interface to a computer.
PTY is an emulated TTY which enables us to emulate multiple terminal interfaces to a computer.

Let's say you want to have multiple terminal emulators open, you want to have them side by side.
You want to have like multiple sessions in that case you will have multiple PTYs basically.

If you want to see the current TTY, you are on, run tty command.

So **Terminal** is a physical device with a keyboard and screen connected to a computer.
**TTY (TeleTypeWriter)** a device for typed messages, now used for text-based computer interfaces.
**PTY (Pseudo Terminal)** Software based terminal for processes to communicate as if they were using a real terminal.

pic

E.g. If we want to print colorful text as above, we have to write gibberish looking code. We call then ANSI escape sequences.

| ESC Code Sequence           | Description                                        |
|-----------------------------|----------------------------------------------------|
| ESC[38;2;{r};{g};{b}m       | Set foreground color as RGB                        |
| ESC[48;2;{r};{g};{b}m       | Set background colot as RGB                        |

\x1b[1;31m # Set style to bold, red background.

We can also do more stuffs such as control the cursor, setting the graphics mode like screen mode etc.

ANSI Escape sequence works like a session. You set something and it remains set for remaining session of the terminal.

To get more infromation about the terminal state on Linux you can use `stty` command.

In below we set something and return back to original setting

```sh
# Print current settings
stty -g

original_settings=$(stty -g)

# Disable canonical mode (line buffering) and echo
stty -icanon -echo 

# Send information over stdout
echo "This is a message sent over stdout"

# Restore original terminal settings
stty "$original_settings"

# Another way of restoring settings
reset
```

# TUI vs GUI
- TUIs are resource efficient compared to GUI
- Faster navigation because you are in terminal adn you have some shortcuts and command inputs.
- Remote accessibility. For UI access you need x11 e.g.

# TOP Picks
**Neovim**: Vim-fork focused on extensibility and usability.
**Helix**: A post-modern modal text editor.
**LazyGit**: Simple terminal UI for Git commands.
**GitUI**: Terminal user interface for Git.
**Gobang**: A cross-platform TUI database management tool.
**Bpytop**: Linux/OSX/FreeBSD resource monitor.
**Lnav**: Log file navigator.
**Yazi**: Terminal File manager based on async IO.
**Atuin**: Magical shell history.
**Bandwhich**: Terminal bandwidth utilization tool.

# Other TUI Libraries
## Ncurses

```c
#include <ncurses.h>

int main() {
    initscr();  /** start curses mode **/
    printw("Hello world!"); 
    refresh();  /** print it on to the real screen **/
    getch();    /** wait for user input **/
    endwin();   /** end curses mode **/

    return 0;
}
```

## cdk (curses development kit)
```c
#include <cdk/cdk.h>

int main() {
    // Initialize the CDK screen
    SCREEN *cdkScreen = initCDKScreen(NULL);

    // Create a CDK dialog box
    CDKDIALOG *dialog = newCDKDialog(cdkScreen, CENTER, CENTER, NULL, "Hello CDK",
    "Press Enter to exit", NULL, 0, TRUE, FALSE, FALSE);

    // Draw the dialog box
    activateCDKDialog(dialog, NULL);

    // Cleanup resources
    destroyCDKDialog(dialog);
    destroyCDKScreen(cdkScreen);

    return 0;
}
```

The reason this exists because doing things with ncurses is pretty difficult. If you want complex UI then it's really difficult to have it in ncurses. So peple created curses development kit. It provided some widgets such as dialogs, calanders etc.

## textualize.io
It's python framework for building TUIs. dophie  a TUI for monitoring MySQL in realtime. Textualize can also run in browser.

## Bubble Tea
A Go framework based on the ELM architecture.

## tui-rs
https://github.com/fdehau/tui-rs

## ratatui
A fork of tui-rs and it's most used TUI framework in Rust.

# Why RUST
When it comes to building TUIs we have lots of options. But why choose rust?

- Memory Safety
- Performance
- Cross-platform support
- Security
- Ecosystem/package management

In case of ncurses vs CDK, ncurses does the terminal handling and CDK does the UI rendering.

In the case of ratatui, terminal can be handled with couple of backends. You can choose e.g. crossterm, termion, termviz and Ratatui is responsible for rendering some widgets.

# Demo
Create a new project using:

```sh
cargo new hello-ratatui
cd hello-ratatui
cargo add ratatui crossterm
```

```rust
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::io::{stdout, Result, Stdout};

fn main() -> Result<()> {
    // Turnoff i/o for full control over the terminal (cooked mode!) 
    stdout().execute(EnterAlternateScreen);
    // enter secondary screen to not disturb normal output
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO: main loop
    // restore
    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}
```

EnterAlternateScreen is like new buffer in your terminal, so if you run your TUI app and you want to switch to a 
new screen and have like a clean page where you can render stuff. We also called it cooked mode. You switch to it
to have full control over terminal. In this mode I/O is turned off and you just have to handle your stuff yourself
and before exiting, you have to restore the terminal. Because you don't want to messup your output.

# Render Loop
Most important part while building the TUI is render loop. First you need to draw the UI in this case you can use 
the terminals `draw` method which takes a frame. It's a **closure** and it renders the entire screen.

```rust
terminal.draw(|frame| {
    let area = frame.size();
    frame.render_widget(
        Paragraph::new(
            "Hello Ratatui! (press 'q' to quit)",
        )
            .white()
            .on_blue(),
        area,
    );
})?;
```

Next I need to handle some event. In this case I am pulling some events from crossterm and if q is pressed I just
break from the loop.

```rust
if event::poll(std::time::Duration::from_millis(16))? {
    if let event::Event::Key(key) == event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            break;
        }
    }
}
```
 16ms is roughly 60 fps. So you have to wait a bit just to make sure that UI remains responsive, regardless of
weather we have new events pending. Below is the full code:

```rust
use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::io::{Result, stdout};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen);
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //main loop
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}
```

You might ask what happens in case of errors. You can use panic hooks. One such example is below:

```rust
pub fn initialize_panic_handler() {
    use better_panic::Settings;
    std::panic::set_hook(Box::new(|panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(panic_info);
    }));
}
```

This will restore your terminal in case of a panic.

# Concepts
## Area (Rect)
