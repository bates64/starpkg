mod id;
mod script;

// TODO: make these share a common trait so code doesn't have to be copied everywhere.
// Call them 'exports'?
mod sprite;
mod text;
mod actor;

use crate::prelude::*;
use std::collections::HashMap;
use id::Identify;
use script::Script;

use sprite::*;
use actor::*;
use text::*;

const SPRITESHEET_XML_PLAYERSPRITES: &str = r#"<PlayerSprites>
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

/// A package is a collection of sprites, actors, etc that optionally depends on other packages.
/// They can be assembled into a Star Rod mod folders to be compiled.
#[derive(Debug)]
pub struct Package {
    /// The directory holding this package.
    pub dir: PathBuf,

    /// Parsed starpkg.toml manifest file.
    manifest: Manifest,

    /// Flattened dependency graph.
    dependencies: Vec<Package>,

    /// Maps of Identifier -> export for this package and its cummulative dependencies.
    sprites: SpriteMap,
    actors: ActorMap,
    texts: TextMap,
}

impl Package {
    /// Creates a new package at `dir` with the given name. If `dir` already has any files in it,
    /// the new package is placed in a subdirectory of `dir` instead.
    pub fn new(dir: &Path, name: String) -> Result<Package> {
        let dir = if !dir.exists() {
            fs::create_dir_all(dir)?;
            dir.to_owned()
        } else if dir.read_dir()?.count() > 0 {
            if dir.join("starpkg.toml").exists() {
                warn!("a package is here already - creating a subdirectory");
            }

            debug!("package dir {} already has files - using subdirectory", dir.display());

            let subdir = dir.join(&name);
            let _ = fs::create_dir(&subdir);
            subdir
        } else {
            dir.to_owned()
        };

        if dir.join("starpkg.toml").exists() {
            return Err(anyhow!("directory {} is already a package", dir.display()))
        }

        let package = Package {
            dir,

            manifest: Manifest {
                name,
                version: Version::parse("0.1.0").unwrap(),

                dependencies: HashMap::new(),
            },

            dependencies: Vec::new(),

            sprites: HashMap::new(),
            actors: HashMap::new(),
            texts: HashMap::new(),
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

        let dir = relative_path_to(dir)
            .map_err(|err| LoadError::Other(err.into()))?;

        debug!("loading package: {}", dir.display());

        // Read starpkg.toml.
        let manifest: Manifest = {
            let string = fs::read_to_string(dir.join("starpkg.toml"))
                .map_err(LoadError::UnfoundManifest)?;

            toml::from_str(&string)?
        };

        // Load all dependencies into a flat vec.
        let mut deps = manifest
            .load_dependencies(&dir)?
            .into_iter()
            .fold(Vec::new(), |mut deps, mut dep| {
                deps.append(&mut dep.dependencies);
                deps.push(dep);

                deps
            });
        deps.sort_unstable_by_key(|dep| dep.name().to_owned());
        deps.dedup(); // Removes dependencies with the same name and same version.

        // Check for dependencies with the same name but unequal versions.
        let deps_unchecked_length = deps.len();
        deps.dedup_by_key(|dep| dep.name().to_owned()); // Removes dependencies with the same name.
        if deps.len() != deps_unchecked_length {
            // TODO: give more info when throwing this error
            return Err(LoadError::MultiDependencyVersionMismatch);
        }

        fn sum_hashmaps<K: Eq + std::hash::Hash, V>(
            mut sum_map: HashMap<K, V>,
            map: HashMap<K, V>
        ) -> HashMap<K, V> {
            for (key, value) in map.into_iter() {
                sum_map.insert(key, value);
            }

            sum_map
        }

        let mut pkg = Package {
            dir: dir.clone(),

            manifest,

            sprites: deps
                .iter()
                .map(|dep| dep.sprites.clone())
                .fold(HashMap::new(), sum_hashmaps),

            actors: deps
                .iter()
                .map(|dep| dep.actors.clone())
                .fold(HashMap::new(), sum_hashmaps),

            texts: deps
                .iter()
                .map(|dep| dep.texts.clone())
                .fold(HashMap::new(), sum_hashmaps),

            dependencies: deps,
        };

        // Load this package's sprites.
        let sprites_dir = dir.join("src/sprite");
        if sprites_dir.is_dir() {
            for entry in sprites_dir.read_dir().unwrap() {
                let dir = entry.unwrap().path();
                let sprite = Sprite::load(pkg.name(), &dir)
                    .with_context(|| format!("unable to load sprite: {}", dir.display()))?;

                let id = SpriteId::identify(&pkg, &sprite);
                info!("loaded {:?}", &id);

                pkg.sprites.insert(id, sprite);
            }
        }

        // Load this package's actors.
        let actors_dir = dir.join("src/actor");
        if actors_dir.is_dir() {
            for entry in actors_dir.read_dir().unwrap() {
                let dir = entry.unwrap().path();
                let actor = Actor::load(pkg.name(), dir)?;

                let id = ActorId::identify(&pkg, &actor);
                info!("loaded {:?}", &id);

                pkg.actors.insert(id, actor);
            }
        }

        // Load this package's texts (strings).
        let texts_dir = dir.join("src/string");
        if texts_dir.is_dir() {
            for entry in texts_dir.read_dir().unwrap() {
                let file = entry.unwrap().path();
                let texts = Text::load_many(pkg.name(), file.clone())
                    .with_context(|| format!("error parsing strings file: {}", &file.display()))?;

                for text in texts {
                    let id = TextId::identify(&pkg, &text);
                    info!("loaded {:?}", &id);

                    pkg.texts.insert(id, text);
                }
            }
        }

        Ok(pkg)
    }

    /// Traverses upwards from the given root path, looking for the first package we see.
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

    pub fn name(&self) -> &str {
        &self.manifest.name
    }

    /// Assembles the package to a mod directory, ready to be compiled by Star Rod.
    pub fn assemble(&mut self, build_dir: &Path) -> Result<()> {
        let _ = fs::create_dir_all(build_dir);

        // Scripts can reference assembled exports, so we'll process them after assembling
        // everything else.
        let mut scripts = Vec::new();

        // Sprites.
        self.assemble_sprites(&build_dir.join("sprite"))?;

        // Texts.
        self.assemble_strings(&build_dir.join("strings"))?;

        // Actors.
        scripts.append(&mut self.assemble_actors(&build_dir.join("battle"))?);

        // TODO: assemble battles
        // TODO: assemble npcs
        // TODO: assemble maps

        // Assembly is done - time to process + save scripts!
        for mut script in scripts {
            script.resolve_expressions(&self.sprites, &self.texts, &self.actors)?;
            script.save()?;
        }

        Ok(())
    }

    fn assemble_sprites(&mut self, sprites_dir: &Path) -> Result<()> {
        let _ = fs::create_dir_all(sprites_dir);

        fs::write(sprites_dir.join("SpriteTable.xml"), {
            let mut xml = String::new();
            writeln!(xml, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
            writeln!(xml, "<SpriteTable>")?;

            // NPC sprites.
            writeln!(xml, "    <NpcSprites>")?;
            let mut index = 1u8;
            for (sprite_id, sprite) in &mut self.sprites {
                writeln!(xml, r#"        <Sprite id="{idx:X}" src="{idx:02X}" name="{name}"/>"#,
                    idx  = index,
                    name = sprite_id,
                )?;

                let sprite_dir = sprites_dir.join(format!("npc/src/{:02X}", index));
                let _ = fs::create_dir_all(&sprite_dir);

                sprite.assemble(&sprite_dir, index)?;
                debug!("npc sprite {:02X} = {:?}", index, &sprite_id);

                index += 1; // TODO: check overflow
            }
            writeln!(xml, "    </NpcSprites>")?;

            // Player sprites (TODO: support patching these?)
            writeln!(xml, "{}", SPRITESHEET_XML_PLAYERSPRITES)?;

            xml += "</SpriteTable>";
            xml
        }).with_context(|| "unable to write to SpriteTable.xml")?;

        Ok(())
    }

    fn assemble_strings(&mut self, strings_dir: &Path) -> Result<()> {
        // Clear the directory.
        let _ = fs::remove_dir_all(&strings_dir);
        fs::create_dir_all(strings_dir)?;

        let mut sections = [0u16; 255];
        for text in self.texts.values_mut() {
            let section_idx = text.string_section() as usize;

            text.assemble(strings_dir, sections[section_idx])?;

            sections[section_idx] += 1;
        }

        Ok(())
    }

    fn assemble_actors(&mut self, battle_dir: &Path) -> Result<Vec<Script>> {
        let actors_dir = battle_dir.join("formation/import/actor");
        let _ = fs::create_dir_all(&actors_dir);

        let mut scripts = Vec::new();

        fs::write(battle_dir.join("ActorTypes.xml"), {
            let mut xml = String::new();
            writeln!(xml, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
            writeln!(xml, "<ActorTypes>")?;

            let mut index = 0u8;
            for (actor_id, actor) in &mut self.actors {
                writeln!(xml, r#"   <Actor id="{idx:02X}" name="{name}" tattle="{tattle}"/>"#,
                    idx  = index,
                    name = actor.name.resolve(&self.texts)
                        .with_context(|| format!("actor name not found: {:?}", actor.name))?
                        .assembled_hex_id()
                        .expect("actor name string was not assembled"),
                    tattle = actor.tattle.resolve(&self.texts)
                        .with_context(|| format!("actor tattle not found: {:?}", actor.tattle))?
                        .assembled_hex_id()
                        .expect("actor tattle string was not assembled"),
                )?;

                scripts.push(actor.assemble(&actors_dir, index)?);
                debug!("actor {:02X} = {:?}", index, &actor_id);

                index += 1; // TODO: check overflow
            }

            xml += "</ActorTypes>";
            xml
        }).with_context(|| "unable to write to ActorTypes.xml")?;

        Ok(scripts)
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

    #[error("dependencies with the same name but different versions found")]
    MultiDependencyVersionMismatch,

    #[error(transparent)]
    Other(#[from] Error),
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

/// Finds the relative path to the given target path from the current working directory.
fn relative_path_to(target: &Path) -> io::Result<PathBuf> {
    use std::path::Component;

    // Canonicalize (make absolute) both paths so we can compare them properly.
    // This also normalizes weird Windows-only behaviour where paths begin with a special prefix
    // that says "this is a very long path" - we can compare paths only if they *both* have it.
    let target = target.canonicalize()?;
    let pwd = std::env::current_dir()?.canonicalize()?;

    // Find the common prefix of `target` and the current working directory.
    let prefix: Vec<Component> = pwd.components()
        .zip(target.components())
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();

    // We need to back up from the current directory to get to `prefix`:
    let parent_count = pwd.components().count() - prefix.len();
    let parents = combine_path_components(vec![Component::ParentDir; parent_count]);

    // Find the path we must take in order to get from `prefix` to `target`.
    let to_target = target.strip_prefix(combine_path_components(prefix)).unwrap();

    // Join these to find the relative path from the current working directory to `target`!
    Ok(parents.join(to_target))
}

/// Converts a Vec<Component> to a PathBuf. No idea why the standard library doesn't support this.
fn combine_path_components(components: Vec<std::path::Component>) -> PathBuf {
    let mut buf = PathBuf::from(".");

    for component in components {
        if let std::path::Component::CurDir = component {
            continue
        }

        buf.push(component);
    }

    buf
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
