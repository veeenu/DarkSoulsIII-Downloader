use console::Term;
use dialoguer::{Select, Input, PasswordInput};
use std::path::PathBuf;
use std::process::{Command, Stdio};

const DEPOT_DOWNLOADER_PATH: &'static str = "DepotDownloader/DepotDownloader.exe";

#[derive(Debug)]
struct Manifest {
  app_id: u64,
  depot: u64,
  manifest: u64
}

#[derive(Debug)]
struct PatchSet {
  patch_id: String,
  target_folder: String,
  manifests: Vec<Manifest>
}

impl Manifest {
  fn new(app_id: u64, depot: u64, manifest: u64) -> Manifest {
    Manifest { app_id, depot, manifest }
  }

  fn download(&self, path: &str, username: &str, password: &str) {
    println!("Downloading {} | {} | {} to {}", self.app_id, self.depot, self.manifest, path);
    let mut exe_path = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .to_path_buf();
    exe_path.push(PathBuf::from(DEPOT_DOWNLOADER_PATH));
    Command::new(&exe_path)
      .stdin(Stdio::inherit())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .args(&[
        "-app", &format!("{}", self.app_id),
        "-depot", &format!("{}", self.depot),
        "-manifest", &format!("{}", self.manifest),
        "-username", username,
        "-password", password,
        "-dir", path
      ])
      .spawn()
      .expect("Couldn't start subprocess")
      .wait()
      .expect("Couldn't download depot");
  }
}

impl PatchSet {
  fn new(desc: &str, target_folder: &str, manifests: Vec<Manifest>) -> PatchSet {
    PatchSet {
      patch_id: desc.to_owned(),
      target_folder: target_folder.to_owned(),
      manifests
    }
  }

  fn download(&self, username: &str, password: &str) {
    println!("Downloading patch {}", self.patch_id);
    for manifest in &self.manifests {
      manifest.download(&self.target_folder, username, password);
    }
  }
}

impl ToString for PatchSet {
  fn to_string(&self) -> String {
    self.patch_id.clone()
  }
}

fn main() {
  let patches = vec![
    PatchSet::new("Ver 1.04 / Reg. 1.05 (Any%)", "v1.04", vec![
      Manifest::new(374320, 374321, 7552375533020122115)
    ]),
    PatchSet::new("Ver 1.08 / Reg. 1.22 (Any% No TearDrop)", "v1.08", vec![
      Manifest::new(374320, 374321, 8582612580945347462),
      Manifest::new(374320, 506970, 6554758566340383649),
    ]),
    PatchSet::new("Ver 1.12 / Reg. 1.32 (All Bosses)", "v1.12", vec![
      Manifest::new(374320, 374321, 627215463385297895),
      Manifest::new(374320, 506970, 2441039608905796121),
      Manifest::new(374320, 506971, 2910451490878009764),
    ]),
  ];

  let term = Term::stdout();
  term.clear_screen().unwrap();

  let username = Input::<String>::new()
    .with_prompt("Username")
    .interact()
    .unwrap();

  let password = PasswordInput::new()
    .with_prompt("Password")
    .interact()
    .unwrap();

  let chosen = Select::new()
    .items(&patches)
    .default(0)
    .with_prompt("Patch to download")
    .interact_on(&term)
    .unwrap();

  patches[chosen].download(&username, &password);
}
