// Code modeled off this
// https://github.com/microsoft/terminal/blob/master/src/tools/ColorTool/ColorTool/ColorTable.cs




// const DarkBlack: u16 = 0;
// const DarkBlue: u16 = 1;
// const DarkGreen: u16 = 2;
// const DarkCyan: u16 = 3;
// const DarkRed: u16 = 4;
// const DarkMagenta: u16 = 5;
// const DarkYellow: u16 = 6;
// const DarkWhite: u16 = 7;
// const BrightBlack: u16 = 8;
// const BrightBlue: u16 = 9;
// const BrightGreen : u16 = 10;
// const BrightCyan : u16 = 11;
// const BrightRed : u16 = 12;
// const BrightMagenta : u16 = 13;
// const BrightYellow : u16 = 14;
// const BrightWhite : u16 = 15;

// const Foregrounds: [u16; 17] = [
//     BrightWhite,
//     DarkBlack,
//     BrightBlack,
//     DarkRed,
//     BrightRed,
//     DarkGreen,
//     BrightGreen,
//     DarkYellow,
//     BrightYellow,
//     DarkBlue,
//     BrightBlue,
//     DarkMagenta,
//     BrightMagenta,
//     DarkCyan,
//     BrightCyan,
//     DarkWhite,
//     BrightWhite, 
// ];

// const Backgrounds: [u16; 8] = [
//     DarkBlack,
//     DarkRed,
//     DarkGreen,
//     DarkYellow,
//     DarkBlue,
//     DarkMagenta,
//     DarkCyan,
//     DarkWhite,  
// ];

// const AnsiForegroundSequences: [&'static str; 18] = [
//     "m",
//     "1m",
//     "30m",
//     "1;30m",
//     "31m",
//     "1;31m",
//     "32m",
//     "1;32m",
//     "33m",
//     "1;33m",
//     "34m",
//     "1;34m",
//     "35m",
//     "1;35m",
//     "36m",
//     "1;36m",
//     "37m",
//     "1;37m",
// ];

// const AnsiBackgroundSequences: [&'static str; 9] = [
//     "m",
//     "40m",
//     "41m",
//     "42m",
//     "43m",
//     "44m",
//     "45m",
//     "46m",
//     "47m",
// ];

// const TEST_TEXT: &str = "  gYw  ";
// const TEST_TEXT: &str = "NuShell";
const ANSI_PREFIX: &str = "\x1B[";
const ANSI_RESET: &str = "\x1B[0m";

fn main() {
    // println!("\x1b[0;31mStackOverflow\x1b[0m");
    print_table();
}

pub fn print_table() {
    let ansi_fg_sequences = vec!["m", "1m", "30m", "1;30m", "31m", "1;31m", "32m", "1;32m", "33m", "1;33m", "34m", "1;34m", "35m", "1;35m", "36m", "1;36m", "37m", "1;37m", ];
    let ansi_bg_sequences = vec!["40m", "41m", "42m", "43m", "44m", "45m", "46m", "47m", ];

    // Print the column header
    print!("\t");
    for (bg, bg_color) in ansi_bg_sequences.iter().enumerate() {
        print!("{: >9}", bg_color);
        print!(" ");
    }
    print!("\n"); 

    for (fg, fg_color) in ansi_fg_sequences.iter().enumerate() {
        let mut cur_fg:String;
        let mut cur_bg:String;

        if fg >= 0 { 
            // Write the row header
            cur_fg = format!("{}\t", fg_color);
            print!("{: >7}{}", cur_fg, ANSI_RESET);
        }
        if fg == 0 { 
            cur_fg = format!("{}{}", ANSI_PREFIX, "37m");
        } 
        else {
            cur_fg = format!("{}{}", ANSI_PREFIX, fg_color);
        }
        for (bg, bg_color) in ansi_bg_sequences.iter().enumerate() {
            if bg > 0 { print!(" "); }
            if bg == 0 {
                cur_bg = format!("{}{}", ANSI_PREFIX, "40m");
            } else {
                cur_bg = format!("{}{}", ANSI_PREFIX, bg_color);
            }
            // print!("{}{}{}{}{}", cur_fg, cur_bg, TEST_TEXT, ANSI_RESET);
            print!("{: >7}{}{};{}{}", cur_fg, cur_bg, fg_color, bg_color, ANSI_RESET);
        }
        print!("\n");
    }
    print!("\n");
    print!("{}", ANSI_RESET);
}