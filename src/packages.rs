use anyhow::{Context, Result};
use std::{fs, io::{self, Write}, path::{Path, PathBuf}, process::Command};

fn run(cmd: &str, args: &[&str], cwd: &Path) -> Result<()> {
    let status = Command::new(cmd).args(args).current_dir(cwd).status()
        .with_context(|| format!("failed to start {cmd}"))?;
    if !status.success() { anyhow::bail!("{cmd} exited with {status}"); }
    Ok(())
}
fn confirm(action: &str) -> Result<bool> {
    print!("{action} [y/N] "); io::stdout().flush()?;
    let mut answer=String::new(); io::stdin().read_line(&mut answer)?;
    Ok(matches!(answer.trim().to_lowercase().as_str(),"y"|"yes"))
}
fn valid_package(name:&str)->bool {
    !name.is_empty() && name.len()<=200 && name.chars().all(|c|c.is_ascii_alphanumeric()||"+.-".contains(c))
}
pub fn update()->Result<()> {
    if !confirm("Run apt package index update (sudo may prompt)?")? { return Ok(()); }
    run("sudo",&["apt-get","update"],Path::new("/"))
}
pub fn search(query:&str)->Result<()> { run("apt-cache",&["search",query],Path::new("/")) }
pub fn install(name:&str)->Result<()> {
    if !valid_package(name){anyhow::bail!("invalid package name");}
    if !confirm(&format!("Install package {name} using APT?"))?{return Ok(());}
    run("sudo",&["apt-get","install","-y",name],Path::new("/"))
}
pub fn remove(name:&str)->Result<()> {
    if !valid_package(name){anyhow::bail!("invalid package name");}
    if !confirm(&format!("Remove package {name} using APT?"))?{return Ok(());}
    run("sudo",&["apt-get","remove","-y",name],Path::new("/"))
}

/// Build the Debian Trixie live image. Rust owns configuration, orchestration,
/// binary staging and checksum generation; live-build assembles the Debian image.
pub fn build_image()->Result<()> {
    let root=repository_root()?;
    #[cfg(unix)]
    if unsafe { libc::geteuid() } != 0 {
        anyhow::bail!("image construction requires root; run: sudo target/release/triad image");
    }
    #[cfg(not(unix))]
    anyhow::bail!("the Debian live image must be built on a Linux host");

    for tool in ["lb","debootstrap","xorriso","mksquashfs","sha256sum"] {
        if !command_exists(tool){anyhow::bail!("required image-build tool is missing: {tool}");}
    }
    let executable=std::env::current_exe().context("cannot locate triad executable")?;
    let staged=root.join("config/includes.chroot/usr/bin/triad");
    if let Some(parent)=staged.parent(){fs::create_dir_all(parent)?;}
    fs::copy(&executable,&staged).with_context(||format!("stage binary at {}",staged.display()))?;
    set_executable(&staged)?;

    let args=[
        "config","noauto","--mode","debian","--distribution","trixie",
        "--architectures","amd64","--binary-images","iso-hybrid",
        "--archive-areas","main contrib non-free non-free-firmware",
        "--debian-installer","live","--apt-recommends","true","--apt-indices","true",
        "--security","false","--memtest","none","--iso-application","TriadOS",
        "--iso-publisher","TriadOS Project","--iso-volume","TRIADOS_Trixie_amd64"
    ];
    println!("[1/3] Configure Debian Trixie live-build");
    run("lb",&args,&root)?;
    println!("[2/3] Build hybrid amd64 ISO (this may take a while)");
    run("lb",&["build"],&root)?;

    let iso=root.join("live-image-amd64.hybrid.iso");
    if !iso.is_file()||fs::metadata(&iso)?.len()==0 {
        anyhow::bail!("live-build did not produce {}",iso.display());
    }
    println!("[3/3] Generate SHA-256 checksum");
    let output=Command::new("sha256sum").arg(&iso).current_dir(&root).output()?;
    if !output.status.success(){anyhow::bail!("sha256sum failed");}
    let checksum=String::from_utf8(output.stdout)?;
    let checksum_path=root.join("live-image-amd64.hybrid.iso.sha256");
    fs::write(&checksum_path,checksum)?;
    println!("ISO: {}",iso.display());
    println!("Checksum: {}",checksum_path.display());
    Ok(())
}
fn repository_root()->Result<PathBuf>{
    let current=std::env::current_dir()?;
    for dir in current.ancestors(){
        if dir.join("Cargo.toml").is_file()&&dir.join("config/package-lists/triados.list.chroot").is_file(){
            return Ok(dir.to_path_buf());
        }
    }
    anyhow::bail!("run this command from inside the TriadOS repository")
}
fn command_exists(command:&str)->bool{
    std::env::var_os("PATH").unwrap_or_default().to_string_lossy().split(':')
        .any(|dir|Path::new(dir).join(command).is_file())
}
#[cfg(unix)]
fn set_executable(path:&Path)->Result<()>{
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path,fs::Permissions::from_mode(0o755))?; Ok(())
}
#[cfg(not(unix))]
fn set_executable(_path:&Path)->Result<()>{Ok(())}
