use std::{path::Path, io};
use colour::yellow_ln;

/// Structure representing the possible command line arguments.
#[derive(Clone)]
pub struct Arguments {
    pub file_path:String,
    pub window_width:usize,
    pub window_height:usize,
    pub verbose:bool,
    pub fill_state:i32,
    pub gen_states:Vec<i32>
}

impl Arguments {
    pub fn new_blank() -> Arguments {
        Arguments {file_path:String::new(), window_width:0, window_height:0, verbose:false, fill_state:0, gen_states:vec![] }
    }
}

/// Structure used to maintain the state of the command line argument parser.
/// Use the 'parse' method to get the results.
#[derive(Clone)]
struct ArgParseState {
    args:Vec<String>,
    cur_arg:String,
    cur_arg_index:usize,
    result:Arguments
}

impl ArgParseState {
    fn new(args:Vec<String>) -> ArgParseState {
        let cur_a = String::from(&args[0]);
        ArgParseState { args:args, cur_arg:cur_a, cur_arg_index:0, result:Arguments::new_blank() }
    }

    fn parse(&mut self, fp:String) -> io::Result<Arguments> {
        self.result.file_path = fp;

        while self.cur_arg_index < self.args.len() {
            
            // parsing window size
            if self.cur_arg == "-size" {
                self.advance();
                if !self.parse_size() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid size parameters!"));
                }
                yellow_ln!("Warning: -size arguments do not currently work with the default renderer.");
            }

            // parsing debug mode flag
            if self.cur_arg == "-verbose" {
                self.advance();
                self.result.verbose = true;
            }

            // help flag raised
            if self.cur_arg == "-help" {
                self.advance();
                print_help();
            }

            // Parse the fill flag
            if self.cur_arg == "-fill" {
                self.advance();
                if !self.parse_fill() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameter given for -fill option. Expecting a valid <state>."));
                }
            }

            if self.cur_arg == "-gen" {
                if !self.parse_gen() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameter given for -gen option. Expecting a valid list of <state>."));
                }
            }

        }
        Ok(self.result.clone())
    }

    fn parse_gen(&mut self) -> bool {
        while self.advance() && self.cur_arg.chars().all(char::is_numeric) {
            let state = self.cur_arg.parse::<i32>();
            match state {
                Ok(n) => self.result.gen_states.push(n),
                Err(_e) => return false
            };
        }
        true
    }

    fn parse_fill(&mut self) -> bool {
        // parse the state
        let state = self.cur_arg.parse::<i32>();
        self.advance();
        match state {
            Ok(n) => self.result.fill_state = n,
            Err(_e) => return false
        };
        true
    }

    /// Attempts to parse window size parameters from the args.
    fn parse_size(&mut self) -> bool {

        // parse the width
        let width = self.cur_arg.parse::<usize>();
        match width {
            Ok(n) => self.result.window_width = n,
            Err(_e) => return false
        };
        self.advance();

        // parse the height
        let height = self.cur_arg.parse::<usize>();
        match height {
            Ok(n) => self.result.window_height = n,
            Err(_e) => return false
        };
        self.advance();
        
        true
    }

    fn advance(&mut self) -> bool {
        self.cur_arg_index += 1;
        if self.cur_arg_index < self.args.len() {
            self.cur_arg = self.args[self.cur_arg_index].to_owned();
            return true;
        }
        false
    }
}

/// Attempts to parse command line arguments for the command line interface.
/// Accepts a vector string of args that should omit the working directory argument.
pub fn parse_args(args:&Vec<String>) -> io::Result<Arguments> {
    let fp:&String = &args[0];

    // Check if the file exists
    match Path::new(fp).exists() {
        true => (),
        false => return Err(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' does not exist.", fp)))
    };

    // If there are more arguments besides the file path, parse them.
    if args.len() > 1 {
        let mut parser = ArgParseState::new(args[1..].to_vec());
        let p_result = parser.parse(fp.to_string());

        p_result
    } else {
        Ok(Arguments { file_path:fp.to_string(), window_width:0, window_height:0, verbose:false, fill_state:0, gen_states:vec![] })
    }
}

/// Prints a help message to the console.
pub fn print_help() {
    colour::cyan!("CellM 0.1 --- Usage: cellm.exe <filename> [arguments...]");
    print!("
    -verbose                          Print output from tokenizer and parser.
    -help                             Print help screen.
    -fill <state>                     Fill the grid with <state> cells at the start.
    -gen  <state> ...                 Randomly place all given states into cells on the grid at the start.
    -size <width> <height>            Indicate desired size of simulation window.");

    yellow_ln!("\t<- Not implemented for default renderer.");

    print!("
Example run:
    cellm <filename> -verbose");
}