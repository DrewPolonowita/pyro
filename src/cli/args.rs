use crate::Result;
use crate::cli::error::{AppError, Error, ErrorType};

pub struct Args {
    pub command: Option<Command>,
    pub argv: Vec<String>,
    pub flags: Flags
}

#[derive(Debug)]
pub enum Command {
    Cd,
    Ls,
    New,
    Del,
    Echo,
    Cat,
    Info,
    Pwd,
    Move
}

impl Command {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "cd" => Some(Command::Cd),
            "ls" => Some(Command::Ls),
            "new" => Some(Command::New),
            "del" => Some(Command::Del),
            "echo" => Some(Command::Echo),
            "cat" => Some(Command::Cat),
            "info" => Some(Command::Info),
            "pwd" => Some(Command::Pwd),
            "move" => Some(Command::Move),
            _ => None,
        }
    }
}

#[derive(Default, Debug)]
pub struct Flags {
    pub long: bool,
    pub help: bool,
    pub human: bool,
    pub recursive: bool,
    pub dir: bool,
    pub force: bool,
    pub time: bool,
    pub all: bool,
    pub reverse: bool
}

#[derive(Default, Debug)]
pub struct FlagsBuilder {
    flags: Flags,
}

impl FlagsBuilder {
    pub fn new() -> Self {
        Self {
            flags: Flags::default(),
        }
    }

    pub fn long(mut self) -> Self {
        self.flags.long = true;
        self
    }

    pub fn human(mut self) -> Self {
        self.flags.human = true;
        self
    }

    pub fn recursive(mut self) -> Self {
        self.flags.recursive = true;
        self
    }

    pub fn dir(mut self) -> Self {
        self.flags.dir = true;
        self
    }

    pub fn force(mut self) -> Self {
        self.flags.force = true;
        self
    }

    pub fn time(mut self) -> Self {
        self.flags.time = true;
        self
    }

    pub fn all(mut self) -> Self {
        self.flags.all = true;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.flags.reverse = true;
        self
    }

    pub fn parse(&mut self, token: &str) -> Result<bool> {
        match token {
            "--long" => {
                self.flags.long = true;
            },
            "--help" => {
                self.flags.help = true;
            },
            "--human" => {
                self.flags.human = true;
            },
            "--recursive" => {
                self.flags.recursive = true;
            },
            "--dir" => {
                self.flags.dir = true;
            },
            "--force" => {
                self.flags.force = true;
            },
            "--time" => {
                self.flags.time = true;
            },
            "--all" => {
                self.flags.all = true;
            },
            "--reverse" => {
                self.flags.reverse = true;
            },
            _ => {
                let mut chars = token.chars();
                if let Some(first_char) = chars.next() {
                    if first_char == '-' {
                        while let Some(char) = chars.next() {
                            match char {
                                'l' => {
                                    self.flags.long = true;
                                },
                                'h' => {
                                    self.flags.human = true;
                                },
                                'r' => {
                                    self.flags.recursive = true;
                                },
                                'd' => {
                                    self.flags.dir = true;
                                },
                                'f' => {
                                    self.flags.force = true;
                                },
                                't' => {
                                    self.flags.time = true;
                                },
                                'a' => {
                                    self.flags.all = true;
                                },
                                'R' => {
                                    self.flags.reverse = true;
                                }
                                token => {
                                    let error_bad_short_string = Error {
                                        error_type: ErrorType::InvalidToken,
                                        error_message: String::leak(format!("Invalid token '{}'; this token is not a short flag option", token))
                                    };

                                    return Err(AppError::from(error_bad_short_string));
                                }
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                };
            }
        }

        Ok(true)
    }

    pub fn build(self) -> Flags {
        self.flags
    }
}

pub enum ArgRunStatus {
    Conditional(String),
    UnConditional(String),
    Pipeline(String),
}