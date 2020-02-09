use crate::prelude::*;
use duct::cmd;

#[derive(Debug)]
pub struct StarRod {
    dir: PathBuf,
}

impl StarRod {
    pub fn new() -> Option<StarRod> {
        let dir = crate::INSTALL_DIR.join("star-rod-0.2.0");

        if dir.is_dir() && dir.join("StarRod.jar").is_file() {
            Some(StarRod { dir })
        } else {
            None
        }
    }

    pub fn new_or_download() -> io::Result<StarRod> {
        const DOWNLOAD: &str = "https://github.com/nanaian/star-rod/archive/v0.2.0";

        match StarRod::new() {
            Some(sr) => Ok(sr),
            None => {
                info!("downloading Star Rod, please wait...");

                if cfg!(target_os = "windows") {
                    cmd!("PowerShell", "-c", format!(
                        r#"Invoke-WebRequest -Uri "{}.zip" -OutFile "{}\star-rod.zip""#,
                        DOWNLOAD,
                        crate::INSTALL_DIR.display(),
                    )).run()?;
                    cmd!("PowerShell", "-c", format!(
                        r#"Expand-Archive -Force -Path "{0}\star-rod.zip" -DestinationPath "{0}""#,
                        crate::INSTALL_DIR.display(),
                    )).run()?;
                    cmd!("PowerShell", "-c", format!(
                        r#"Remove-Item "{}\star-rod.zip""#,
                        crate::INSTALL_DIR.display(),
                    )).run()?;
                } else {
                    let archive = crate::INSTALL_DIR.join("star-rod.tar.gz");

                    cmd!("curl", "-o", &archive, "-L", format!("{}.tar.gz", DOWNLOAD)).run()?;
                    cmd!("tar", "-xf", &archive, "-C", crate::INSTALL_DIR.to_path_buf()).run()?;
                    cmd!("rm", &archive).run()?;
                }

                Ok(StarRod::new().unwrap())
            }
        }
    }
}
