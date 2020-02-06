use crate::prelude::*;
use super::id::{self, Identify};
use super::{SpriteMap, SpriteId, ActorMap, ActorId, TextMap, TextId};
use std::collections::VecDeque;
use regex::{Regex, Captures};

/// A script patch file, e.g. *.bscr, *.mscr, *.str.
/// This interface allows you to read a script file, process it, and then output it to a file.
#[derive(Debug)]
pub struct Script {
    /// The path this script is to be saved to.
    /// Defaults to the path the script was loaded from, if applicable.
    pub path: PathBuf,

    /// The empty-line-delimited blocks that make up this script.
    pub blocks: Vec<Block>,

    src_pkg_name: String,
}

impl Script {
    /// Reads and parses a script located at the given path. Comments are ignored
    pub fn load(src_pkg_name: &str, path: PathBuf) -> Result<Script> {
        let source = fs::read_to_string(&path)
            .with_context(|| format!("script file not found: {}", &path.display()))?;

        // Ignore comments in the source string.
        // FIXME: line numbers are not representative of the source if it has comments
        let source = {
            let mut string = String::new();
            let mut in_block_comment = false;
            let mut in_line_comment = false;
            let mut prev_ch = '_';

            for ch in source.chars() {
                if in_block_comment {
                    if ch == '/' && prev_ch == '%' {
                        in_block_comment = false;
                    }
                } else if in_line_comment {
                    if ch == '\n' {
                        in_line_comment = false;
                    }
                } else if ch == '\r' {
                    // Ignore carriage returns.
                } else if ch == '%' {
                    if prev_ch == '/' {
                        string.pop();
                        in_block_comment = true;
                    } else {
                        in_line_comment = true;

                        if prev_ch != '\n' {
                            string.push('\n');
                        }
                    }
                } else {
                    string.push(ch);
                }

                prev_ch = ch;
            }

            string.push('\n');
            string
        };

        Ok(Script {
            // Parse source into a vec of empty-line-delimited blocks.
            blocks: {
                let mut blocks = Vec::new();
                let mut line_buf = Vec::new();

                for (line_idx, line) in source.lines().enumerate() {
                    let line = line.trim();

                    if line == "" {
                        // End of block.
                        if !line_buf.is_empty() {
                            // Remove all lines from `line_buf` and create a Block from them.
                            blocks.push(Block::new(line_buf
                                .drain(..)
                                .collect::<Vec<(usize, String)>>()
                            )?);
                        }
                    } else {
                        // Start/continuation of block.
                        line_buf.push((line_idx + 1, line.to_owned()));
                    }
                }

                if blocks.is_empty() {
                    warn!("'{}' is empty", &path.file_name().unwrap().to_str().unwrap());
                }

                blocks
            },

            path,
            src_pkg_name: src_pkg_name.to_owned(),
        })
    }

    /// Writes updated sourcecode. To 'save as,' set `self.path` beforehand.
    pub fn save(&self) -> io::Result<()> {
        use std::io::Write;

        let mut f = fs::File::create(&self.path)?;

        for block in &self.blocks {
            for (_, line) in &block.lines {
                writeln!(f, "{}", line)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }

    /// Resolves expressions:
    ///
    /// - `{Sprite:id}`
    /// - `{Sprite:id:anim}`
    /// - `{Sprite:id:anim:palette}
    /// - `{String:id}` - overloads Star Rod's
    /// - `{Actor:id}`
    pub fn resolve_expressions(
        &mut self,
        sprites: &SpriteMap,
        texts: &TextMap,
        actors: &ActorMap,
    ) -> Result<(), ResolveError> {
        use ResolveError::*;

        lazy_static! {
            static ref SPRITE_ID: Regex = Regex::new(
                r"\{Sprite:([^:}]*)\}"
            ).unwrap();

            static ref SPRITE_ID_ANIM: Regex = Regex::new(
                r"\{Sprite:([^:}]*):([^:}]*)\}"
            ).unwrap();

            static ref SPRITE_ID_ANIM_PALETTE: Regex = Regex::new(
                r"\{Sprite:([^:}]*):([^:}]*):([^:}]*)\}"
            ).unwrap();

            static ref STRING_ID: Regex = Regex::new(
                r"\{String:([^:}]*)\}"
            ).unwrap();

            static ref ACTOR_ID: Regex = Regex::new(
                r"\{Actor:([^:}]*)\}"
            ).unwrap();
        }

        fn replace<F: Fn(Captures) -> Result<String, ResolveError>>(
            regex: &Regex,
            text: &str,
            func: F,
        ) -> Result<String, ResolveError> {
            let mut new = String::with_capacity(text.len());
            let mut last_match = 0;
            for cap in regex.captures_iter(text) {
                let m = cap.get(0).unwrap();
                new.push_str(&text[last_match..m.start()]);
                last_match = m.end();

                new.push_str(&func(cap)?);
            }
            new.push_str(&text[last_match..]);
            Ok(new)
        }

        let mut new = VecDeque::new();
        for block in &self.blocks {
            new.push_front(block.lines.iter().map(|(line_no, line)| {
                let line_no = *line_no;

                // {Sprite:id}
                let mut line = replace(&SPRITE_ID, line, |g| {
                    let id = g.get(1).unwrap().as_str();
                    let id = SpriteId::parse(id, &self.src_pkg_name)
                        .map_err(|err| IdParseError {
                            path: self.path.clone(),
                            line_no,
                            id_string: id.to_string(),
                            parse_error: err,
                        })?;

                    match id.resolve(sprites) {
                        Some(sprite) => Ok(format!("{:02X}",
                            sprite
                                .assembled_index()
                                .expect("unassembled sprite")
                        )),
                        None => Err(UnknownSprite { path: self.path.clone(), line_no, id }),
                    }
                })?;

                // {Sprite:id:anim}
                line = replace(&SPRITE_ID_ANIM, &line, |g| {
                    let id = g.get(1).unwrap().as_str();
                    let id = SpriteId::parse(id, &self.src_pkg_name)
                        .map_err(|err| IdParseError {
                            path: self.path.clone(),
                            line_no,
                            id_string: id.to_string(),
                            parse_error: err,
                        })?;

                    let anim = g.get(2).unwrap().as_str();

                    match id.resolve(sprites) {
                        Some(sprite) => Ok(format!(
                            "00{index:02X}00{anim:02X}",
                            index = sprite
                                .assembled_index()
                                .expect("unassembled sprite"),
                            anim = sprite.animation_by_name(anim)
                                .ok_or_else(|| SpriteLacksAnimation {
                                    path: self.path.clone(),
                                    line_no,
                                    id,
                                    animation: anim.to_string(),
                                })?
                        )),
                        None => Err(UnknownSprite { path: self.path.clone(), line_no, id }),
                    }
                })?;

                // {Sprite:id:anim:palette}
                line = replace(&SPRITE_ID_ANIM_PALETTE, &line, |g| {
                    let id = g.get(1).unwrap().as_str();
                    let id = SpriteId::parse(id, &self.src_pkg_name)
                        .map_err(|err| IdParseError {
                            path: self.path.clone(),
                            line_no,
                            id_string: id.to_string(),
                            parse_error: err,
                        })?;

                    let anim = g.get(2).unwrap().as_str();
                    let palette = g.get(3).unwrap().as_str();

                    match id.resolve(sprites) {
                        Some(sprite) => Ok(format!(
                            "00{index:02X}{palette:02X}{anim:02X}",
                            index = sprite
                                .assembled_index()
                                .expect("unassembled sprite"),
                            anim = sprite.animation_by_name(anim)
                                .ok_or_else(|| SpriteLacksAnimation {
                                    path: self.path.clone(),
                                    line_no,
                                    id: id.clone(),
                                    animation: anim.to_string(),
                                })?,
                            palette = sprite.palette_by_name(anim)
                                .ok_or_else(|| SpriteLacksPalette {
                                    path: self.path.clone(),
                                    line_no,
                                    id,
                                    palette: palette.to_string(),
                                })?
                        )),
                        None => Err(UnknownSprite { path: self.path.clone(), line_no, id }),
                    }
                })?;

                // {String:id}
                line = replace(&STRING_ID, &line, |g| {
                    let id = g.get(1).unwrap().as_str();
                    let id = TextId::parse(id, &self.src_pkg_name)
                        .map_err(|err| IdParseError {
                            path: self.path.clone(),
                            line_no,
                            id_string: id.to_string(),
                            parse_error: err,
                        })?;

                    match id.resolve(texts) {
                        Some(text) => Ok(text.assembled_hex_id().expect("unassembled text")),
                        None => Err(UnknownText { path: self.path.clone(), line_no, id }),
                    }
                })?;

                // {Actor:id}
                line = replace(&ACTOR_ID, &line, |g| {
                    let id = g.get(1).unwrap().as_str();
                    let id = ActorId::parse(id, &self.src_pkg_name)
                        .map_err(|err| IdParseError {
                            path: self.path.clone(),
                            line_no,
                            id_string: id.to_string(),
                            parse_error: err,
                        })?;

                    match id.resolve(actors) {
                        Some(actor) => Ok(format!("{:02X}",
                            actor
                                .assembled_index()
                                .expect("unassembled actor")
                        )),
                        None => Err(UnknownActor { path: self.path.clone(), line_no, id }),
                    }
                })?;

                Ok((line_no, line))
            }).collect::<Result<Vec<_>, ResolveError>>()?);
        }

        for mut block in self.blocks.iter_mut() {
            block.lines = new.pop_back().unwrap();
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("{path}:{line_no}: failed to parse id '{id_string}': {parse_error}")]
    IdParseError {
        path: PathBuf,
        line_no: usize,
        id_string: String,

        #[source]
        parse_error: id::ParseError,
    },

    #[error("{path}:{line_no}: unknown sprite: {id:#?}")]
    UnknownSprite {
        path: PathBuf,
        line_no: usize,
        id: SpriteId,
    },

    #[error("{path}:{line_no}: sprite {id:#?} has no animation '{animation}'")]
    SpriteLacksAnimation {
        path: PathBuf,
        line_no: usize,
        id: SpriteId,
        animation: String,
    },

    #[error("{path}:{line_no}: sprite {id:#?} has no palette '{palette}'")]
    SpriteLacksPalette {
        path: PathBuf,
        line_no: usize,
        id: SpriteId,
        palette: String,
    },

    #[error("{path}:{line_no}: unknown string: {id:#?}")]
    UnknownText {
        path: PathBuf,
        line_no: usize,
        id: TextId,
    },

    #[error("{path}:{line_no}: unknown actor: {id:#?}")]
    UnknownActor {
        path: PathBuf,
        line_no: usize,
        id: ActorId,
    },
}

#[derive(Debug)]
pub struct Block {
    /// The type of block this is.
    pub kind: BlockKind,

    /// (sourcefile line number, line)
    pub lines: Vec<(usize, String)>,
}

impl Block {
    pub fn new(lines: Vec<(usize, String)>) -> Result<Self> {
        assert!(!lines.is_empty());

        Ok(Block {
            kind: {
                lazy_static! {
                    static ref STRING_NAMED: Regex = Regex::new(
                        r"#string:([0-9A-F]{2}):\((.*)\)"
                    ).unwrap();
                }

                let (line_no, header) = &lines[0];

                if let Some(g) = STRING_NAMED.captures(&header) {
                    trace!("STRING_NAMED {:?}", g);

                    BlockKind::StringNamed {
                        section: u8::from_str_radix(g.get(1).unwrap().as_str(), 16)
                            .with_context(||
                                format!("bad string section index on line {}", line_no))?,
                        name: g.get(2).unwrap().as_str().to_owned(),
                    }
                } else {
                    BlockKind::Other
                }
            },
            lines,
        })
    }
}

#[derive(Debug)]
pub enum BlockKind {
    /// `#string:XX:(Name)`
    StringNamed {
        section: u8,
        name: String,
    },

    Other,
}
