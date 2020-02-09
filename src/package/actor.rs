use crate::prelude::*;
use crate::sanitize;
use super::Package;
use super::id::{Identify, Identifier};
use super::script::Script;
use super::text::TextId;

pub type ActorMap = std::collections::HashMap<ActorId, Actor>;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct ActorId(Identifier);

impl Identify for ActorId {
    type T = Actor;

    fn new(pkg_name: &str, actor_name: &str) -> Self {
        Self(Identifier::new(pkg_name, actor_name))
    }

    fn identify(pkg: &Package, actor: &Actor) -> Self {
        Self(Identifier::from_package(pkg, &actor.name()))
    }
}

impl fmt::Debug for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Color::Fixed(12).normal().paint(format!("{{Actor:{:?}}}", self.0)))
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    dir: PathBuf,
    src_pkg_name: String,

    pub name: TextId,
    pub tattle: TextId,

    assembled_index: Option<u8>,
}

impl Actor {
    pub fn load(src_pkg_name: &str, dir: PathBuf) -> Result<Actor> {
        #[derive(Deserialize)]
        struct Manifest {
            name: String,
            tattle: String,

            // TODO: tattle_cam_offset & shadow_offset
        }

        let name = dir.file_name().unwrap().to_str().unwrap();
        let toml = fs::read_to_string(dir.join(format!("{}.toml", name)))
            .with_context(|| format!("toml file for actor '{}' not found", name))?;
        let manifest: Manifest = toml::from_str(&toml)
            .with_context(|| format!("parse error in actor manifest '{}.toml'", name))?;

        let actor = Actor {
            name: TextId::parse(&manifest.name, src_pkg_name)?,
            tattle: TextId::parse(&manifest.tattle, src_pkg_name)?,

            dir,
            src_pkg_name: src_pkg_name.to_owned(),
            assembled_index: None,
        };

        sanitize::export_name(&actor.name())?;

        Ok(actor)
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
        self.assembled_index
    }

    pub fn assemble(&mut self, actors_dir: &Path, index: u8) -> Result<Script> {
        let mut script = self.script()
            .with_context(|| format!("error parsing script for actor: {}", self.name()))?;

        script.path = actors_dir.join(format!("{:02X}_{}.bpat", index, self.name()));

        self.assembled_index = Some(index);

        Ok(script)
    }

    fn script(&self) -> Result<Script> {
        Script::load(&self.src_pkg_name, self.dir.join(format!("{}.bscr", self.name())))
    }
}
