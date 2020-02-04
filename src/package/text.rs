use crate::prelude::*;
use super::Package;
use super::id::{Identify, Identifier};
use super::script::{Script, BlockKind};

pub type TextMap = std::collections::HashMap<TextId, Text>;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct TextId(Identifier);

impl Identify for TextId {
    type T = Text;

    fn new(pkg_name: &str, text_name: &str) -> Self {
        Self(Identifier::new(pkg_name, text_name))
    }

    fn identify(pkg: &Package, text: &Text) -> Self {
        Self(Identifier::from_package(pkg, &text.name()))
    }
}

impl fmt::Debug for TextId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Color::Fixed(12).paint(format!("{{String:{:?}}}", self.0)))
    }
}

/// A '#string:XX:(Name)' block. See also: [script::BlockKind::StringNamed].
/// Note that Star Rod calls these 'strings' - therefore, so do we, externally.
#[derive(Clone, Debug)]
pub struct Text {
    section: u8,
    name: String,
    string: String,

    assembled_index: Option<u16>,
}

impl Text {
    pub fn load_many(src_pkg_name: &str, str_file_path: PathBuf) -> Result<Vec<Text>> {
        Script::load(src_pkg_name, str_file_path)?.blocks
            .into_iter()
            .map(|block| match block.kind {
                BlockKind::StringNamed { section, name } => Ok(Text {
                    section,
                    name,
                    string: block.lines
                        .into_iter()
                        .skip(1)
                        .map(|(_, line)| line)
                        .collect::<Vec<String>>()
                        .join("\n"),
                    assembled_index: None,
                }),
                _ => Err(anyhow!("only `#string:XX:(id)` blocks allowed in text exports")),
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn string_section(&self) -> u8 {
        self.section
    }

    pub fn assembled_hex_id(&self) -> Option<String> {
        if let Some(index) = self.assembled_index {
            Some(format!("{:04X}{:04X}", self.section, index))
        } else {
            None
        }
    }

    pub fn assemble(&mut self, out_dir: &Path, index: u16) -> Result<()> {
        fs::write(out_dir.join(format!("{:04X}{:04X}.str", self.section, index)), {
            let mut source = format!("#string:{:02X}:{:03X}\n", self.section, index);
            source.push_str(&self.string);
            source
        })?;

        self.assembled_index = Some(index);

        Ok(())
    }
}
