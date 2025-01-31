use crossterm::event::{read, Event::{self, Key}, KeyCode::{self}, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::Terminal;
mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View
}

impl Editor {
    // pub const fn default() -> Self {
    //     Self { 
    //         should_quit: false,
    //         view: View,            
    //     }
    // }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        self.refresh_screen()?;
        loop {            
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, kind: KeyEventKind::Press, ..
        }) = event
        {
            // println!("{}", code);
            match code {       
                KeyCode::Char(c) => {
                    if *c == 'q' && *modifiers == KeyModifiers::CONTROL {
                        self.should_quit = true;
                    } else {
                        //
                    }
                }
                
                _ => {
                    let _ = Terminal::move_cursor(*code);
                },
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
           
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    
    
}