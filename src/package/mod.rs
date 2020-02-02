use crate::prelude::*;

/// Represents a package directory.
#[derive(Debug)]
pub struct Package {
    pub dir: PathBuf,
    manifest: Manifest,

    pub sprites: Vec<Sprite>,
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
            },
            sprites: Vec::new(),
        };

        package.write_manifest()?;
        fs::create_dir_all(package.dir.join("src/sprites"))?;

        Ok(package)
    }

    /// Loads the package at the given directory.
    pub fn from_dir(dir: &Path) -> Result<Package, FromDirError> {
        if !dir.is_dir() {
            return Err(FromDirError::NotDirectory(dir.to_owned()));
        }

        // Read starpkg.toml.
        let mut pkg = Package {
            dir: dir.to_owned(),
            manifest: {
                let string = fs::read_to_string(dir.join("starpkg.toml"))
                    .map_err(FromDirError::UnfoundManifest)?;

                toml::from_str(&string)?
            },
            sprites: Vec::new()
        };

        // Read NPC sprites.
        let npcs_dir = dir.join("src/sprite/npc");
        if npcs_dir.is_dir() {
            for entry in npcs_dir.read_dir().unwrap() {
                let dir = entry.unwrap().path();

                pkg.sprites.push(Sprite {
                    identifier: Identifier::new(&pkg, dir
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()),
                    dir,
                });
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
            match Package::from_dir(&path) {
                Ok(package) => return Ok(package),
                Err(FromDirError::UnfoundManifest(_)) => (),
                Err(err) => return Err(FindError::FromDirError(err)),
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
    pub fn assemble(&self, build_dir: &Path) -> Result<()> {
        // Populate sprite directory.
        fs::write(build_dir.join("sprite/SpriteTable.xml"), {
            let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#.to_string();

            xml += "<SpriteTable>";

            // NPC sprites
            xml += "<NpcSprites>";
            let mut npc_id = 1u8;
            for sprite in &self.sprites {
                xml += &format!(r#"<Sprite id="{id:X}" src="{src:02X}" name="{name}"/>"#,
                    id   = npc_id,
                    src  = npc_id,
                    name = sprite.identifier,
                );

                // Copy the sprite's directory across.
                let npc_dir = build_dir.join(format!("sprite/npc/src/{:02X}", npc_id));
                let _ = fs::create_dir_all(&npc_dir);
                for entry in sprite.dir.read_dir()? {
                    let path = entry?.path();
                    fs::copy(&path, npc_dir.join(path.file_name().unwrap()))?;
                }

                debug!("assembled npc sprite {} -> {:02X}", sprite.identifier, npc_id);

                npc_id += 1;
            }
            xml += "</NpcSprites>";

            // Player sprites (TODO: support patching these?)
            xml += r#"
            <PlayerSprites>
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

            xml += "</SpriteTable>";

            xml
        })?;

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

impl fmt::Display for Package {
    /// `{}`   -> `{name} v{version}` in cyan
    /// `{:#}` -> `{name}_v{version}`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            // Safe-identifier mode
            write!(f, "{}",
                format!("{}_{}", &self.manifest.name, &self.manifest.version)
                    .replace(".", "_")
                    .replace(" ", "_")
                    .replace(":", "_")
            )
        } else {
            write!(f, "{}", Color::Fixed(14).normal().paint(
                format!("{} v{}", &self.manifest.name, &self.manifest.version)
            ))
        }
    }
}

#[derive(Error, Debug)]
pub enum FromDirError {
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
    FromDirError(#[from] FromDirError),
}

/// A starpkg.toml.
#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    name: String,
    version: Version,
}

/// A fully-qualified identifier.
#[derive(Debug)]
pub struct Identifier {
    package_id: String,
    name: String,
}

impl Identifier {
    pub fn new(package: &Package, name: String) -> Identifier {
        Identifier {
            package_id: format!("{:#}", package),
            name,
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.package_id, self.name)
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub dir: PathBuf,
    pub identifier: Identifier,
}
