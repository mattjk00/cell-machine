use std::{path::Path, io};

use colour::yellow_ln;

#[derive(Clone)]
pub struct Arguments {
    pub file_path:String,
    pub window_width:usize,
    pub window_height:usize,
    pub verbose:bool
}

impl Arguments {
    fn new_blank() -> Arguments {
        Arguments {file_path:String::new(), window_width:0, window_height:0, verbose:false }
    }
}

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

        }
        Ok(self.result.clone())
    }

    /// Attempts to parse window size parameters from the args.
    fn parse_size(&mut self) -> bool {
        self.advance();

        // parse the width
        let width = self.cur_arg.parse::<usize>();
        match width {
            Ok(n) => self.result.window_width = n,
            Err(e) => return false
        };
        self.advance();

        // parse the height
        let height = self.cur_arg.parse::<usize>();
        match height {
            Ok(n) => self.result.window_height = n,
            Err(e) => return false
        };
        self.advance();
        
        true
    }

    fn advance(&mut self) {
        self.cur_arg_index += 1;
        if self.cur_arg_index < self.args.len() {
            self.cur_arg = self.args[self.cur_arg_index].to_owned();
        }
    }
}

pub fn parse_args(args:&Vec<String>) -> io::Result<Arguments> {
    let fp:&String = &args[0];

    // Check if the file exists
    match Path::new(fp).exists() {
        true => (),
        false => return Err(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' does not exist.", fp)))
    };

    if args.len() > 1 {
        let mut parser = ArgParseState::new(args[1..].to_vec());
        let p_result = parser.parse(fp.to_string());

        p_result
    } else {
        Ok(Arguments { file_path:fp.to_string(), window_width:0, window_height:0, verbose:false })
    }
}