use crate::prelude::*;
use super::Package;
use super::id::{Identify, Identifier};

pub type SpriteMap = std::collections::HashMap<SpriteId, Sprite>;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct SpriteId(Identifier);

impl Identify for SpriteId {
    type T = Sprite;

    fn new(pkg_name: &str, sprite_name: &str) -> Self {
        Self(Identifier::new(pkg_name, sprite_name))
    }

    fn identify(pkg: &Package, sprite: &Sprite) -> Self {
        Self(Identifier::from_package(pkg, &sprite.name()))
    }
}

impl fmt::Display for SpriteId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for SpriteId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Color::Fixed(12).normal().paint(format!("{{Sprite:{:?}}}", self.0)))
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    dir: PathBuf,

    palettes: Vec<String>,
    animations: Vec<String>,

    assembled_index: u8, // Zero => unassembled.
}

impl Sprite {
    pub fn load(_: &str, dir: &Path) -> Result<Sprite, SpriteLoadError> {
        let spritesheet = fs::read_to_string(dir.join("SpriteSheet.xml"))
            .map_err(SpriteLoadError::MissingSpriteSheet)?;
        let spritesheet = roxmltree::Document::parse(&spritesheet)
            .map_err(SpriteLoadError::MalformedSpriteSheet)?;

        Ok(Sprite {
            palettes: spritesheet
                .root_element()
                .children()
                .find(|n| n.tag_name().name() == "PaletteList")
                .ok_or(SpriteLoadError::SpriteSheetMissingPaletteList)?
                .children()
                .filter(|n| n.is_element())
                .enumerate()
                .map(|(idx, n)| match n.attribute("name") {
                    Some(attr) => attr.to_string(),
                    None => format!("{:X}", idx),
                })
                .collect(),

            animations: spritesheet
                .root_element()
                .children()
                .find(|n| n.tag_name().name() == "AnimationList")
                .ok_or(SpriteLoadError::SpriteSheetMissingAnimationList)?
                .children()
                .filter(|n| n.is_element())
                .enumerate()
                .map(|(idx, n)| match n.attribute("name") {
                    Some(attr) => attr.to_string(),
                    None => format!("{:X}", idx),
                })
                .collect(),

            dir: dir.to_owned(),
            assembled_index: 0,
        })
    }

    pub fn name(&self) -> String {
        self.dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn assembled_index(&self) -> Option<u8> {
        match self.assembled_index {
            0 => None,
            _ => Some(self.assembled_index),
        }
    }

    pub fn assemble(&mut self, out_dir: &Path, index: u8) -> Result<()> {
        for entry in self.dir.read_dir()? {
            let path = entry?.path();
            let target_path = out_dir.join(path.file_name().unwrap());

            fs::copy(&path, &target_path).with_context(|| {
                format!("failed to copy sprite: {} -> {}",
                    &path.display(),
                    &target_path.display())
            })?;
        }

        self.assembled_index = index;

        Ok(())
    }

    pub fn palette_by_name(&self, name: &str) -> Option<usize> {
        self.palettes.iter().position(|n| n == name)
    }

    pub fn animation_by_name(&self, name: &str) -> Option<usize> {
        self.animations.iter().position(|n| n == name)
    }
}

#[derive(Error, Debug)]
pub enum SpriteLoadError {
    #[error("missing SpriteSheet.xml")]
    MissingSpriteSheet(#[source] io::Error),

    #[error("malformed SpriteSheet.xml")]
    MalformedSpriteSheet(#[source] roxmltree::Error),

    #[error("SpriteSheet.xml is missing PaletteList")]
    SpriteSheetMissingPaletteList,

    #[error("SpriteSheet.xml is missing PaletteList")]
    SpriteSheetMissingAnimationList,
}
