/**
 *  wc (word count) implementation in Rust
 *
 *  The purpose is to learn Rust a bit
 *
 *  author: michal.vyskocil@gmail.com
 *
 * TODO:
 *   * output is not properly formatted
 *   * word splitting is done on space only, where manual page claims whitespace
 */

use std::io::{BufferedReader, Reader, File, Lines, Buffer, IoError};
use std::os::args;

#[deriving(Show)]
pub struct WordCount {
    pub lines: uint,
    pub words: uint,
    pub chars: uint,
    pub bytes: uint
}

impl WordCount {
    fn sum(&mut self, other: &WordCount) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
        self.bytes += other.bytes;
    }
}

#[deriving(Show)]
struct Cfg {
    pub byte_count: bool,
    pub char_count: bool,
    pub line_count: bool,
    pub word_count: bool,
}

fn wc<'r, T: Buffer>(lines : &'r mut Lines<'r, T>) -> Result<WordCount, IoError> {
    let mut ret : WordCount = WordCount { lines: 0u, words: 0u, chars: 0u, bytes: 0u };

    for res in lines {
        let line = try!(res);
        let slice: &str = line.as_slice();

        ret.lines += 1;
        ret.words += slice.split(' ').count();
        ret.chars += slice.chars().count();
        ret.bytes += slice.bytes().count();

    }

    return Ok(ret);
}

fn parse_args(args: Vec<String>) -> (Cfg, Vec<String>) {
    
    let mut cfg = Cfg{byte_count: false, char_count: false, line_count: false, word_count: false};
    let mut idx = 1u;
    let mut paths = Vec::new();

    while idx < args.len() {
        match args[idx].as_slice() {
            "--lines" => cfg.line_count = true,
            "-l"      => cfg.line_count = true,
            "--bytes" => cfg.byte_count = true,
            "-c"      => cfg.byte_count = true,
            "--chars" => cfg.char_count = true,
            "-m"      => cfg.char_count = true,
            "--words" => cfg.word_count = true,
            "-w"      => cfg.word_count = true,
            _ => paths.push(args[idx].clone())  //FIXME: unecessary clone
        }
        idx += 1;
    }

    if !cfg.byte_count && (!cfg.char_count && ! cfg.line_count && !cfg.word_count) {
        cfg.char_count = true;
        cfg.line_count = true;
        cfg.word_count = true;
    }

    (cfg, paths)
}

//newline, word, character, byte, maximum line length
fn print_results(wc : &WordCount, cfg: &Cfg, path: &str) {

    let mut buf = String::new();

    if cfg.line_count {
        buf = format!("{:u}", wc.lines);
    }

    //TODO: add creates new object, investigate things like extend
    if cfg.word_count {
        buf = buf.add(&format!(" {:u}", wc.words));
    }
    if cfg.char_count {
        buf = buf.add(&format!(" {:u}", wc.chars));
    }
    if cfg.byte_count {
        buf = buf.add(&format!(" {:u}", wc.bytes));
    }

    let mut sbuf = buf.as_slice();
    
    if sbuf.char_at(0) == ' ' {
        sbuf = sbuf.slice_from(1);
    }

    println!("{:s} {:s}", sbuf, path);
}

fn main() {

    let (cfg, paths) = parse_args(args());
    println!("cfg: {}, paths: {}", cfg, paths);

    if paths.len() == 0 {
        fail!("Reading from stdin is not yet implemented");
    }
    
    let mut total : WordCount = WordCount { lines: 0u, words: 0u, chars: 0u, bytes: 0u };

    for path_i in paths.iter() {

        let path = path_i.as_slice();

        // TODO: clone is just a workaround
        let p = Path::new(path);
        // errors can be checked on opening file ...
        let f = match File::open(&p) {
            Err(why) => fail!("Can't open {}: {}", p.display(), why.desc),
            Ok(f) => f,
        };
        let mut br = BufferedReader::new(f);

        let res = match wc(&mut br.lines()) {
            Ok(res) => res,
            Err(e) => fail!(e)
        };

        total.sum(&res);
        print_results(&res, &cfg, path);
    }

    if paths.len() > 1 {
        print_results(&total, &cfg, "total");
    }

}
