/**
 *  wc (word count) implementation in Rust
 *
 *  The purpose is to learn Rust a bit
 *
 *  author: michal.vyskocil@gmail.com
 *
 * TODO:
 *   * word splitting is done on space only, where manual page claims whitespace
 *   * it is twelve times slower than GNU wc, needs profiling!
 */

use std::io::{BufferedReader, Reader, File, Lines, Buffer, IoError};
use std::os::args;
use std::cmp::max;

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

fn is_pwr_ten(i: uint) -> bool {
    if i < 10 {
        return false;
    }

    let mut r: uint = i;
    while r > 10 {
        if r % 10 != 0 {
            return false;
        }
        r = r / 10;
    }

    return r == 10;
}

fn uint_len(i: uint) -> uint {

    if i < 10 {
        return 1;
    }

    let ret = (i as f64).log10().ceil() as uint;

    if is_pwr_ten(i) {
        return ret +1;
    }

    ret
}

#[test]
fn test_int_len() {

    for i in range(0u, 10002) {
        let s = format!("{:u}", i);
        //assert_eq!(s.len(), int_len(i));
        println!("{:u} {}", i, uint_len(i));
    }
}

#[test]
fn test_is_pwr_ten() {

    for i in range(0u, 1002) {
        if i == 10 || i == 100 || i == 1000 {
            assert_eq!(is_pwr_ten(i), true);
        }
        else {
            assert_eq!(is_pwr_ten(i), false);
        }
    }
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

fn spaces(n: uint) -> String {
    let mut ret = String::with_capacity(n);
    for _ in range(0, n) {
        ret.push(' ');
    }
    ret
}

fn uipad(i: uint, maxlen: uint) -> String {
    let strlen = uint_len(i);
    if strlen >= maxlen {
        return format!("{:u}", i);
    }
    return format!("{:s}{:u}", spaces(maxlen - strlen), i);
}

//newline, word, character, byte, maximum line length
fn print_results(results : &Vec<(WordCount, &str)>, cfg: &Cfg) {

    let mut lines_maxlen = 0u;
    let mut words_maxlen = 0u;
    let mut chars_maxlen = 0u;
    let mut bytes_maxlen = 0u;

    for &(wc, _) in results.iter() {
        lines_maxlen = max(lines_maxlen, wc.lines);
        words_maxlen = max(words_maxlen, wc.words);
        chars_maxlen = max(chars_maxlen, wc.chars);
        bytes_maxlen = max(bytes_maxlen, wc.bytes);
    }
    lines_maxlen = uint_len(lines_maxlen);
    words_maxlen = uint_len(words_maxlen);
    chars_maxlen = uint_len(chars_maxlen);
    bytes_maxlen = uint_len(bytes_maxlen);

    for i in results.iter() {
        let (wc, path) = *i;

        let mut fmtbuf = String::from_str("");

        if cfg.line_count {
            fmtbuf = format!("{:s} {:s}", fmtbuf, uipad(wc.lines, lines_maxlen));
        }

        //TODO: format! creates new object, investigate things like extend
        if cfg.word_count {
            fmtbuf = format!("{:s} {:s}", fmtbuf, uipad(wc.words, words_maxlen));
        }
        if cfg.char_count {
            fmtbuf = format!("{:s} {:s}", fmtbuf, uipad(wc.chars, chars_maxlen));
        }
        if cfg.byte_count {
            fmtbuf = format!("{:s} {:s}", fmtbuf, uipad(wc.bytes, bytes_maxlen));
        }

        println!("{:s} {:s}", fmtbuf, path);
    }

}

fn main() {

    let (cfg, paths) = parse_args(args());
    println!("cfg: {}, paths: {}", cfg, paths);

    if paths.len() == 0 {
        fail!("Reading from stdin is not yet implemented");
    }
    
    let mut total : WordCount = WordCount { lines: 0u, words: 0u, chars: 0u, bytes: 0u };
    let mut results = Vec::with_capacity(paths.len());

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
        results.push((res, path));
    }

    if paths.len() > 1 {
        results.push((total, "total"));
    }

    print_results(&results, &cfg);

}
