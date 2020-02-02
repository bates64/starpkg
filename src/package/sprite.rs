use crate::prelude::*;
use super::Package;
use super::id::{Identify, Identifier};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct SpriteId(Identifier);

impl Identify for SpriteId {
    type T = Sprite;

    fn new(pkg_name: &str, sprite_name: &str) -> Self {
        Self(Identifier::new(pkg_name, sprite_name))
    }

    fn identify(pkg: &Package, sprite: &Sprite) -> Self {
        Self(Identifier::from_package(pkg, &sprite.name()))
    }

    fn resolve<'pkg>(&self, pkg: &'pkg Package) -> Option<&'pkg Sprite> {
        pkg.sprites.get(self)
    }
}

impl fmt::Display for SpriteId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    dir: PathBuf,

    assembled_index: u8, // Zero => unassembled.
}

impl Sprite {
    pub fn load(dir: PathBuf) -> Sprite {
        // TODO: load SpriteSheet.xml etc
        Sprite { dir, assembled_index: 0 }
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
}
