use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use qolor::palette::Palette;
use colored::Colorize;
use anyhow::{Context, Result};
use qolor::ui;



fn main() -> Result<()> {
    let catppuccin = Palette::load("catppuccin")?;
    // let keyboard = vec!["`1234567890-=", "qwertyuiop[]", "asdfghjkl;'", "zxcvbnm,./"];
    let selection: char = ui::keyboard_selection(&catppuccin)?;

    



    //copied to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    
    println!("char = {}, color = {}", &selection, catppuccin.get_name(selection));

    let to_copy = &catppuccin.get_string(selection);
    ctx.set_contents(to_copy.into()).unwrap();


    //exit gui
    Ok(())
}


//using println
fn selection(palette: Palette) {

    let keyboard = vec!["`1234567890-=", "qwertyuiop[]", "asdfghjkl;'", "zxcvbnm,./"];

    let offsets = ["", "  ", "   ","     "];

    for row in 0..keyboard.len() {
        println!("{}", offsets[row]);
        for button in keyboard[row].chars() {
            let col = palette.get_rgba(button);
            print!("{}", &format!(" {} ", button).on_truecolor(col[0], col[1], col[2]));
        }
        println!("\n");

    }
}

//     initscr();
// /* Print to the back buffer. */
//
//     let offsets = ["", "  ", "   ","     "];
//
//     for row in 0..keyboard.len() {
//         addstr(offsets[row]);
//         for button in keyboard[row].chars() {
//             let col = palette.get_rgba(button);
//             addstr(&format!(" {} ", button).on_truecolor(col[0], col[1], col[2]));
//         }
//         addstr("\n");
//
//     }

    // for character in keyboard.chars() {
    //     let col = palette.get_rgba(character);
    //     println!("{} = {}",k
    //              palette.get_name(character),
    //              palette.get_string(character)
    //                 .truecolor(17,17,27)
    //                 .on_truecolor(col[0], col[1], col[2]));
    // }


    // /* Update the screen. */
    // refresh();
    //
    // /* Wait for a key press. */
    // getch();
    //
    // /* Terminate ncurses. */
    // endwin();

