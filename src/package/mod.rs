mod id;
mod sprite;

use crate::prelude::*;
use id::Identify;
use sprite::{SpriteId, Sprite};
use std::collections::HashMap;

const SPRITESHEET_XML_PLAYERSPRITES: &'static str = r#"<PlayerSprites>
    <Sprite id="1" src="01" name="Mario 1"/>
    <Sprite id="2" src="02" name="Mario 2"/>
    <Sprite id="3" src="03" name="Mario 3"/>
    <Sprite id="4" src="04" name="Mario 4"/>
    <Sprite id="5" src="05" name="Mario 5"/>
    <Sprite id="6" src="06" name="Mario 6"/>
    <Sprite id="7" src="07" name="Mario 7"/>
    <Sprite id="8" src="08" name="Mario 8"/>
    <Sprite id="9" src="09" name="Mario 9"/>
    <Sprite id="A" src="0A" name="Peach 1"/>
    <Sprite id="B" src="0B" name="Peach 2"/>
    <Sprite id="C" src="0C" name="Peach 3"/>
    <Sprite id="D" src="0D" name="Peach 4"/>
</PlayerSprites>"#;

/// Represents a package directory.
#[derive(Debug)]
pub struct Package {
    pub dir: PathBuf,

    manifest: Manifest,

    /// Flattened dependency graph.
    dependencies: Vec<Package>,

    /// A map of Identifier -> Sprite for this package and its dependencies.
    /// Identifiers are flat - they do not care if dependencies are nested.
    sprites: HashMap<SpriteId, Sprite>,
}

impl Package {
    /// Creates a new package at `dir` with the given name. If `dir` already has any files in it,
    /// the new package is placed in a subdirectory of `dir` instead.
    pub fn new(dir: &Path, name: String) -> Result<Package> {
        if dir.join("starpkg.toml").exists() {
            return Err(anyhow!("directory {} is already a package", dir.display()))
        }

        let dir = if !dir.exists() {
            fs::create_dir_all(dir)?;
            dir.to_owned()
        } else if dir.read_dir()?.count() > 0 {
            debug!("package dir {} already has files - using subdirectory", dir.display());

            let subdir = dir.join(&name);
            fs::create_dir(&subdir)?;
            subdir
        } else {
            dir.to_owned()
        };

        let package = Package {
            dir,

            manifest: Manifest {
                name,
                version: Version::parse("0.1.0").unwrap(),

                dependencies: HashMap::new(),
            },

            dependencies: Vec::new(),
            sprites: HashMap::new(),
        };

        package.write_manifest()?;
        fs::write(package.dir.join(".gitignore"), "/.build\n")?;
        fs::create_dir(package.dir.join("src"))?;

        Ok(package)
    }

    /// Loads the package at the given directory.
    pub fn load(dir: &Path) -> Result<Package, LoadError> {
        if !dir.is_dir() {
            return Err(LoadError::NotDirectory(dir.to_owned()));
        }

        // Read starpkg.toml.
        let manifest: Manifest = {
            let string = fs::read_to_string(dir.join("starpkg.toml"))
                .map_err(LoadError::UnfoundManifest)?;

            toml::from_str(&string)?
        };

        // Load all dependencies into a flat vec.
        let deps = manifest
            .load_dependencies(dir)?
            .into_iter()
            .fold(Vec::new(), |mut deps, mut dep| {
                deps.append(&mut dep.dependencies);
                deps.push(dep);

                deps
            });

        let mut pkg = Package {
            dir: dir.to_owned(),

            manifest,

            // Copy in dependency sprites.
            sprites: {
                let mut map = HashMap::new();

                for dep in &deps {
                    for (id, sprite) in &dep.sprites {
                        // XXX: cloning
                        map.insert(id.clone(), sprite.clone());
                    }
                }

                map
            },

            dependencies: deps,
        };

        // Load this package's sprites.
        let sprites_dir = dir.join("src/sprite");
        if sprites_dir.is_dir() {
            for entry in sprites_dir.read_dir().unwrap() {
                let dir = entry.unwrap().path();
                let sprite = Sprite::load(dir);

                debug!("{}: reading sprite {}", &pkg, &sprite.name());

                pkg.sprites.insert(SpriteId::identify(&pkg, &sprite), sprite);
            }
        }

        Ok(pkg)
    }

    /// Traverses upwards from the give root path, looking for the first package we see.
    pub fn find(root: &Path) -> Result<Package, FindError> {
        // Make the root path absolute.
        let mut path = root.canonicalize()
            .map_err(|source| FindError::UnfoundRoot {
                root: root.to_owned(),
                source,
            })?;

        loop {
            match Package::load(&path) {
                Ok(package) => return Ok(package),
                Err(LoadError::UnfoundManifest(_)) => (),
                Err(err) => return Err(FindError::LoadError(err)),
            }

            match path.parent() {
                Some(parent_dir) => path = parent_dir.to_owned(),
                None => return Err(FindError::NotFound {
                    root: root.to_owned(),
                }),
            }
        }
    }

    /// Assembles the package to a mod directory, ready to be compiled by Star Rod.
    /// This is the core of starpkg.
    pub fn assemble(&mut self, build_dir: &Path) -> Result<()> {
        let _ = fs::create_dir_all(build_dir);

        self.assemble_sprite_table(&build_dir.join("sprite"))?;

        Ok(())
    }

    fn assemble_sprite_table(&mut self, sprite_dir: &Path) -> Result<()> {
        let _ = fs::create_dir_all(sprite_dir);

        fs::write(sprite_dir.join("SpriteTable.xml"), {
            let mut xml = String::new();
            writeln!(xml, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
            writeln!(xml, "<SpriteTable>")?;

            // NPC sprites.
            writeln!(xml, "    <NpcSprites>")?;
            let mut sprite_index = 1u8;
            for (sprite_id, sprite) in &mut self.sprites {
                writeln!(xml, r#"        <Sprite id="{idx:X}" src="{idx:02X}" name="{name}"/>"#,
                    idx  = sprite_index,
                    name = sprite_id,
                )?;

                // Assemble the sprite's directory.
                let this_dir = sprite_dir.join(format!("npc/src/{:02X}", sprite_index));
                let _ = fs::create_dir_all(&this_dir);
                sprite.assemble(&this_dir, sprite_index)?;

                debug!("assembled npc sprite {} -> {:02X}", &sprite_id, sprite_index);

                sprite_index += 1;
            }
            writeln!(xml, "    </NpcSprites>")?;

            // Player sprites (TODO: support patching these?)
            writeln!(xml, "{}", SPRITESHEET_XML_PLAYERSPRITES)?;

            xml += "</SpriteTable>";
            xml
        }).with_context(|| "unable to write to SpriteTable.xml")?;

        Ok(())
    }

    /// Writes `self.manifest` to starpkg.toml.
    fn write_manifest(&self) -> Result<()> {
        fs::write(
            self.dir.join("starpkg.toml"),
            toml::to_string(&self.manifest)?,
        ).map_err(Into::into)
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.manifest.name == other.manifest.name && self.manifest.version == other.manifest.version
    }
}
impl Eq for Package {}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Color::Fixed(14).normal().paint(
            format!("{} v{}", &self.manifest.name, &self.manifest.version)
        ))
    }
}

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("not a directory: {0}")]
    NotDirectory(PathBuf),

    #[error("missing starpkg.toml - not a package?")]
    UnfoundManifest(#[source] io::Error),

    #[error("malformed starpkg.toml: {0}")]
    MalformedManifest(#[from] toml::de::Error),
}

#[derive(Error, Debug)]
pub enum FindError {
    #[error("root path {root} does not exist")]
    UnfoundRoot {
        root: PathBuf,

        #[source]
        source: io::Error,
    },

    #[error("could not find starpkg.toml in '{root}' or any parent directory")]
    NotFound {
        root: PathBuf,
    },

    #[error(transparent)]
    LoadError(#[from] LoadError),
}

/// A starpkg.toml.
#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    name: String,
    version: Version,

    #[serde(default)]
    dependencies: HashMap<String, Dependency>,
}

impl Manifest {
    fn load_dependencies(&self, pkg_dir: &Path) -> Result<Vec<Package>, LoadError> {
        let mut packages = Vec::new();

        for (name, dep) in &self.dependencies {
            if dep.path.is_absolute() {
                warn!("dependency '{}' uses an absolute path", name);
            }

            let path = pkg_dir.join(&dep.path);

            debug!("loading dependency '{}' from path: {}", name, &path.display());

            // TODO: detect cyclical dependencies, e.g.
            //
            // child = { path = "child" }
            //
            // parent = { path = ".." }

            let package = Package::load(&path)?; // TODO: error context

            if package.manifest.name != *name {
                warn!("dependency '{}' actually has name '{}'", name, &package.manifest.name)
            }

            packages.push(package);
        }

        Ok(packages)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    path: PathBuf,
}
