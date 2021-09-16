use console::Term;
use dialoguer::{Input, PasswordInput, Select};
use std::path::PathBuf;
use std::process::{Command, Stdio};

const DEPOT_DOWNLOADER_PATH: &'static str = "darksouls3-downloader-tools/DepotDownloader.exe";
const MANIFEST_PATCHER_PATH: &'static str = "darksouls3-downloader-tools/SteamDepotDownpatcher.exe";

#[derive(Debug)]
struct Manifest {
    app_id: u64,
    depot: u64,
    manifest: u64,
}

#[derive(Debug)]
struct PatchSet {
    patch_id: String,
    target_folder: String,
    manifests: Vec<Manifest>,
}

fn patch_steam() -> Result<(), String> {
    let mut exe_path = std::env::current_exe()
        .map_err(|e| format!("{}", e))?
        .parent()
        .ok_or_else(|| String::from("Directory has no parent"))?
        .to_path_buf();
    exe_path.push(PathBuf::from(MANIFEST_PATCHER_PATH));
    Command::new(&exe_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("Couldn't start subprocess: {}", e))?
        .wait()
        .map_err(|e| format!("Couldn't download depot: {}", e))?;

    Ok(())
}

impl Manifest {
    fn new(app_id: u64, depot: u64, manifest: u64) -> Manifest {
        Manifest {
            app_id,
            depot,
            manifest,
        }
    }

    fn download(&self, path: &str, username: &str, password: &str) -> Result<(), String> {
        println!(
            "Downloading {} | {} | {} to {}",
            self.app_id, self.depot, self.manifest, path
        );
        let mut exe_path = std::env::current_exe()
            .map_err(|e| format!("{}", e))?
            .parent()
            .ok_or_else(|| String::from("Directory has no parent"))?
            .to_path_buf();
        exe_path.push(PathBuf::from(DEPOT_DOWNLOADER_PATH));
        Command::new(&exe_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .args(&[
                "-app",
                &format!("{}", self.app_id),
                "-depot",
                &format!("{}", self.depot),
                "-manifest",
                &format!("{}", self.manifest),
                "-username",
                username,
                "-password",
                password,
                "-dir",
                path,
            ])
            .spawn()
            .map_err(|e| format!("Couldn't start subprocess: {}", e))?
            .wait()
            .map_err(|e| format!("Couldn't download depot: {}", e))?;

        Ok(())
    }
}

impl PatchSet {
    fn new(desc: &str, target_folder: &str, manifests: Vec<Manifest>) -> PatchSet {
        PatchSet {
            patch_id: desc.to_owned(),
            target_folder: target_folder.to_owned(),
            manifests,
        }
    }

    fn download(&self, username: &str, password: &str) -> Result<(), String> {
        println!("Downloading patch {}", self.patch_id);
        for manifest in &self.manifests {
            manifest.download(&self.target_folder, username, password)?;
        }
        Ok(())
    }
}

impl ToString for PatchSet {
    fn to_string(&self) -> String {
        self.patch_id.clone()
    }
}

fn get_credentials(term: &Term, patches: &[PatchSet]) -> Result<(String, String, usize), String> {
    let username = Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .map_err(|e| format!("{}", e))?;

    let password = PasswordInput::new()
        .with_prompt("Password")
        .interact()
        .map_err(|e| format!("{}", e))?;

    let chosen = Select::new()
        .items(&patches)
        .default(0)
        .with_prompt("Patch to download")
        .interact_on(&term)
        .map_err(|e| format!("{}", e))?;

    Ok((username, password, chosen))
}

fn main() {
    let patches = vec![
        PatchSet::new(
            "Ver 1.04 / Reg. 1.05 (Any%)",
            "v1.04",
            vec![Manifest::new(374320, 374321, 7552375533020122115)],
        ),
        PatchSet::new(
            "Ver 1.08 / Reg. 1.22 (Any% No TearDrop)",
            "v1.08",
            vec![
                Manifest::new(374320, 374321, 8582612580945347462),
                Manifest::new(374320, 506970, 6721400008311921502),
            ],
        ),
        PatchSet::new(
            "Ver 1.12 / Reg. 1.32 (All Bosses)",
            "v1.12",
            vec![
                Manifest::new(374320, 374321, 627215463385297895),
                Manifest::new(374320, 506970, 1039201774019499553),
                Manifest::new(374320, 506971, 8981258257885112790),
            ],
        ),
    ];

    let term = Term::stdout();
    term.clear_screen().unwrap();

    println!(
        r#"********************************************************************************
* Dark Souls III Patch downloader                                              *
********************************************************************************

Welcome! To download earlier patches of Dark Souls III from Steam, you will have
to supply your username and password.

You will then be asked to choose which patch you want to download.

If you have it enabled, the Depot Downloader will ask for your 2FA code.

Let's begin!

Patching Steam, enabling manifest download...
"#
    );

    if let Err(e) = patch_steam() {
        println!("\n  ERROR: {}", e);
        term.read_key().ok();
        return;
    }

    println!("\nSteam patched for manifest downloads. Please enter your credentials now!\n");

    if let Err(e) = get_credentials(&term, &patches)
        .map(|(username, password, chosen)| patches[chosen].download(&username, &password))
    {
        println!("\n  ERROR: {}", e);
    } else {
        println!("\n ALL DONE!");
    }
    term.read_key().ok();
}
