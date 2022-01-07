use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::{self, Write};
use std::io;

pub struct BfMachine {
    program: Vec<Command>,
    data: Vec<u8>,
    prog_ptr: usize,
    data_ptr: usize,
}

#[derive(Debug)]
pub enum Error {
    Underflow,
    MismatchedBracket,
    IoError(io::Error),
}

#[derive(Debug, Clone, Copy)]
enum Command {
    IncP,  // >
    DecP,  // <
    IncB,  // +
    DecB,  // -
    Out,   // .
    In,    // ,
    Begin, // [
    End,   // ]
}

impl BfMachine {
    pub fn new(source: &str) -> Self {
        Self {
            program: source.chars().filter_map(Command::from_char).collect(),
            data: vec![0],
            prog_ptr: 0,
            data_ptr: 0,
        }
    }

    pub fn run<R: io::Read, W: io::Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
    ) -> Result<(), Error> {
        loop {
            match self.step(input, output) {
                Ok(true) => (),
                Ok(false) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn get_program_string_classic(&self) -> String {
        let mut result = String::new();
        for c in self.program.iter() {
            result.push(c.as_char_classic());
        }
        result
    }

    pub fn get_program_string_emoji_hands(&self) -> String {
        let mut result = String::new();
        for c in self.program.iter() {
            result.push(c.as_char_emoji_hand());
        }
        result
    }

    pub fn get_program_string_emoji_random(&self) -> String {
        let mut result = String::new();
        for c in self.program.iter() {
            result.push(c.as_char_emoji_random());
        }
        result
    }

    /// Returns false if program has finished.
    fn step<R: io::Read, W: io::Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
    ) -> Result<bool, Error> {
        if self.prog_ptr >= self.program.len() {
            return Ok(false);
        }
        let cmd = &self.program[self.prog_ptr];
        use Command::*;
        match cmd {
            IncP => {
                self.data_ptr += 1;
            }
            DecP => match self.data_ptr.checked_sub(1) {
                Some(new_ptr) => {
                    self.data_ptr = new_ptr;
                }
                None => return Err(Error::Underflow),
            },
            IncB => {
                self.ensure_data();
                self.data[self.data_ptr] = self.data[self.data_ptr].overflowing_add(1).0;
            }
            DecB => {
                self.ensure_data();
                self.data[self.data_ptr] = self.data[self.data_ptr].overflowing_sub(1).0;
            }
            Out => match write!(output, "{}", self.data[self.data_ptr] as char) {
                Ok(()) => (),
                Err(e) => return Err(Error::IoError(e)),
            },
            In => {
                let mut buf: [u8; 1] = [0];
                if let Err(e) = input.read_exact(&mut buf) {
                    return Err(Error::IoError(e));
                }
                self.data[self.data_ptr] = buf[0];
            }
            Begin => {
                self.ensure_data();
                if self.data[self.data_ptr] == 0 {
                    let mut lvl: usize = 0;
                    while self.prog_ptr != self.program.len() {
                        match self.program[self.prog_ptr] {
                            Begin => lvl += 1,
                            End => match lvl {
                                0 => return Err(Error::MismatchedBracket),
                                1 => break,
                                _ => lvl -= 1,
                            },
                            _ => (),
                        }
                        self.prog_ptr += 1;
                    }
                    if self.prog_ptr == self.program.len() {
                        return Err(Error::MismatchedBracket);
                    }
                }
            }
            End => {
                self.ensure_data();
                if self.data[self.data_ptr] != 0 {
                    let mut lvl: usize = 0;
                    loop {
                        match self.program[self.prog_ptr] {
                            End => lvl += 1,
                            Begin => match lvl {
                                0 => return Err(Error::MismatchedBracket),
                                1 => break,
                                _ => lvl -= 1,
                            },
                            _ => (),
                        }
                        if self.prog_ptr == 0 {
                            return Err(Error::MismatchedBracket);
                        }
                        self.prog_ptr -= 1;
                    }
                }
            }
        }
        self.prog_ptr += 1;
        Ok(self.prog_ptr < self.program.len())
    }

    fn ensure_data(&mut self) {
        if self.data_ptr >= self.data.len() {
            self.data.resize(self.data_ptr + 1, 0)
        }
    }
}

impl Command {
    fn from_char(c: char) -> Option<Self> {
        use Command::*;
        match c {
            '>' => Some(IncP),
            '<' => Some(DecP),
            '+' => Some(IncB),
            '-' => Some(DecB),
            '.' => Some(Out),
            ',' => Some(In),
            '[' => Some(Begin),
            ']' => Some(End),

            c if HAPPY_FACES_SET.contains(&c) => Some(IncP),
            c if SAD_FACES_SET.contains(&c) => Some(DecP),
            c if CARNIVORES_SET.contains(&c) => Some(In), // carnivores eat bytes
            c if HERBIVORES_SET.contains(&c) => Some(Out), // herbivores poop bytes
            c if WOMEN_SET.contains(&c) => Some(Begin),
            c if MEN_SET.contains(&c) => Some(End),
            c if HEARTS_SET.contains(&c) => Some(IncB),
            c if HATS_SET.contains(&c) => Some(DecB),

            '👆' => Some(IncB),
            '👇' => Some(DecB),
            '👉' => Some(IncP),
            '👈' => Some(DecP),
            '🤏' => Some(In),
            '🤌' => Some(Out),
            '🤜' => Some(Begin),
            '🤛' => Some(End),
            _ => None,
        }
    }

    fn as_char_classic(&self) -> char {
        use Command::*;
        match self {
            IncP => '>',
            DecP => '<',
            IncB => '+',
            DecB => '-',
            Out => '.',
            In => ',',
            Begin => '[',
            End => ']',
        }
    }

    fn as_char_emoji_hand(&self) -> char {
        use Command::*;
        match self {
            IncB => '👆',
            DecB => '👇',
            IncP => '👉',
            DecP => '👈',
            In => '🤏',
            Out => '🤌',
            Begin => '🤜',
            End => '🤛',
        }
    }

    fn as_char_emoji_random(&self) -> char {
        use Command::*;
        let set = match self {
            IncP => HAPPY_FACES,
            DecP => SAD_FACES,
            In => CARNIVORES,
            Out => HERBIVORES,
            Begin => WOMEN,
            End => MEN,
            IncB => HEARTS,
            DecB => HATS,
        };
        let mut rng = rand::thread_rng();
        *set.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.as_char_classic())
    }
}

static HAPPY_FACES: &'static [char] = &[
    '😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣', '🥲', '😊', '😇', '🙂', '🙃', '😉', '😌', '😍',
    '🥰', '😘', '😗', '😙', '😚', '😋', '😛', '😝', '😜', '🤪',
];

static SAD_FACES: &'static [char] = &[
    '😞', '😔', '😟', '😕', '🙁', '😣', '😖', '😫', '😩', '🥺', '😢', '😭', '😤', '😠', '😡', '🤬',
    '🤯', '🥵', '🥶', '😨', '😰', '😥', '😓', '🤢', '🤮', '🤒', '🤕',
];

static CARNIVORES: &'static [char] = &[
    '🐶', '🐱', '🦊', '🐻', '🐯', '🦁', '🐸', '🐧', '🦅', '🦉', '🐺', '🕷', '🦂', '🐍', '🦖', '🐙',
    '🦑', '🐬', '🐳', '🐋', '🦈', '🐊', '🐅', '🐆', '🐕', '🐩', '🦮', '🦦', '🦫',
];

static HERBIVORES: &'static [char] = &[
    '🐭', '🐹', '🐰', '🐼', '🐨', '🐮', '🐷', '🐽', '🐵', '🙈', '🙉', '🙊', '🐒', '🐦', '🐗', '🐴',
    '🦄', '🐛', '🦋', '🐌', '🦕', '🦓', '🦍', '🦧', '🦣', '🐘', '🦛', '🦏', '🐪', '🐫', '🦒', '🦘',
    '🦬', '🐃', '🐂', '🐄', '🐎', '🐖', '🐏', '🐑', '🦙', '🐐', '🦌', '🦜', '🕊', '🐇', '🦥', '🐁',
    '🐀', '🐿',
];

static WOMEN: &'static [char] = &['👧', '👩', '👵', '🧕', '👸', '🤶', '🤰', '💃'];

static MEN: &'static [char] = &['👦', '👨', '👴', '👲', '🕺'];

static HEARTS: &'static [char] = &[
    '🧡', '💛', '💚', '💙', '💜', '🖤', '🤍', '🤎', '💔', '💕', '💞', '💓', '💗', '💖', '💘',
    '💝', '💟', '🫀',
];

static HATS: &'static [char] = &['👑', '👒', '🎩', '🎓', '🧢', '⛑', '🪖'];

lazy_static! {
    static ref HAPPY_FACES_SET: HashSet<char> = HashSet::from_iter(HAPPY_FACES.iter().cloned());
    static ref SAD_FACES_SET: HashSet<char> = HashSet::from_iter(SAD_FACES.iter().cloned());
    static ref CARNIVORES_SET: HashSet<char> = HashSet::from_iter(CARNIVORES.iter().cloned());
    static ref HERBIVORES_SET: HashSet<char> = HashSet::from_iter(HERBIVORES.iter().cloned());
    static ref WOMEN_SET: HashSet<char> = {
        let mut m = HashSet::from_iter(WOMEN.iter().cloned());
        m.insert('♀');
        m
    };

    static ref MEN_SET: HashSet<char> = {
        let mut m = HashSet::from_iter(MEN.iter().cloned());
        m.insert('♂');
        m
    };

    static ref HEARTS_SET: HashSet<char> = {
        let mut m = HashSet::from_iter(HEARTS.iter().cloned());
        m.insert('❤');
        m
    };
    static ref HATS_SET: HashSet<char> = HashSet::from_iter(HATS.iter().cloned());
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Underflow => write!(f, "data pointer went below zero"),
            MismatchedBracket => write!(f, "mismatched bracket"),
            IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::BfMachine;
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_empty() {
        let mut bf = BfMachine::new("");
        let mut reader = Cursor::new(Vec::new());
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_ok());
    }

    #[test]
    fn test_rw() {
        let mut bf = BfMachine::new(",.");
        let mut reader = Cursor::new(vec![55]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_ok());
        assert_eq!(writer.len(), 1);
        assert_eq!(writer[0], 55);
    }

    #[test]
    fn test_bracket1() {
        let mut bf = BfMachine::new("[");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_err());
    }

    #[test]
    fn test_bracket2() {
        let mut bf = BfMachine::new("[[]]");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_ok());
    }

    #[test]
    fn test_bracket3() {
        let mut bf = BfMachine::new("++]");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_err());
    }

    #[test]
    fn test_bracket4() {
        let mut bf = BfMachine::new("[[");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_err());
    }

    #[test]
    fn test_hw() {
        let mut bf = BfMachine::new("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_ok());
        let s = String::from_utf8(writer).unwrap();
        assert_eq!(s, "Hello World!\n");
    }

    #[test]
    fn test_hw_hands() {
        let mut bf = BfMachine::new("👆👆👆👆👆👆👆👆🤜👉👆👆👆👆🤜👉👆👆👉👆👆👆👉👆👆👆👉👆👈👈👈👈👇🤛👉👆👉👆👉👇👉👉👆🤜👈🤛👈👇🤛👉👉🤌👉👇👇👇🤌👆👆👆👆👆👆👆🤌🤌👆👆👆🤌👉👉🤌👈👇🤌👈🤌👆👆👆🤌👇👇👇👇👇👇🤌👇👇👇👇👇👇👇👇🤌👉👉👆🤌👉👆👆🤌");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_ok());
        let s = String::from_utf8(writer).unwrap();
        assert_eq!(s, "Hello World!\n");
    }

    #[test]
    fn test_hw_emoji() {
        let mut bf = BfMachine::new("💔💚❤️🧡💗🤎💔🤍🤰😋💕🖤🤎🫀💃😇💚❤😀💕🤎🧡😂💓💘💝😉💝😰😡😣🤢👒👴😜💓😋💖🥰⛑😃😃❤💃🙁♂😠👒♂😌🥰🐐😉🎩👒🎓🦋🧡🖤💓💛🤍💟💞🕊🐀💞💚💕🐵🙃😜🙈😖👑🦄😔🐨🖤🤎🧡🐹⛑🧢👑🪖🧢🧢🐹👑⛑🪖🎩🎓🪖⛑🎩🐂😝😝❤🐮🥰🤎💔🐼");
        let mut reader = Cursor::new(vec![]);
        let mut writer: Vec<u8> = Vec::new();
        assert!(bf.run(&mut reader, &mut writer).is_ok());
        let s = String::from_utf8(writer).unwrap();
        assert_eq!(s, "Hello World!\n");
    }

    #[test]
    fn test_intersect() {
        assert_eq!(CARNIVORES_SET.intersection(&HERBIVORES_SET).count(), 0);
        assert_eq!(HAPPY_FACES_SET.intersection(&SAD_FACES_SET).count(), 0);
        assert_eq!(WOMEN_SET.intersection(&MEN_SET).count(), 0);
    }
}
