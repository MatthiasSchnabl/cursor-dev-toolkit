use std::{fs, path::{Path,PathBuf}, process::{Command,Output}, sync::atomic::{AtomicU64,Ordering}};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Repo(PathBuf);
impl Drop for Repo { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }
impl Repo {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("docs-guard-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));
        fs::create_dir(&root).unwrap();
        let repo = Self(root);
        repo.git(&["init","-q"]);
        repo.git(&["config","user.name","Test"]);
        repo.git(&["config","user.email","test@example.invalid"]);
        repo.git(&["config","core.autocrlf","false"]);
        repo.git(&["config","commit.gpgsign","false"]);
        repo
    }
    fn git(&self,args:&[&str]) { let out=Command::new("git").current_dir(&self.0).args(args).output().unwrap(); assert!(out.status.success(),"{:?}: {}",args,String::from_utf8_lossy(&out.stderr)); }
    fn put(&self,path:&str,text:&str) { let p=self.0.join(path); fs::create_dir_all(p.parent().unwrap()).unwrap(); fs::write(p,text).unwrap(); }
    fn run(&self,args:&[&str]) -> Output { Command::new(env!("CARGO_BIN_EXE_docs-guard")).arg("--root").arg(&self.0).arg("--json").args(args).output().unwrap() }
    fn expect(&self,args:&[&str],code:i32) -> serde_json::Value {
        let out=self.run(args);
        assert_eq!(out.status.code(),Some(code),"{:?}: {} {}",args,String::from_utf8_lossy(&out.stdout),String::from_utf8_lossy(&out.stderr));
        serde_json::from_slice(&out.stdout).unwrap()
    }
    fn fixture(&self) {
        self.expect(&["init"],0);
        self.put("docs-guard.toml",&include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture"));
        self.put("README.md","[index](docs/INDEX.md)\n");
        self.put("AGENTS.md","Read [README](README.md).\n");
        self.put("docs/INDEX.md","[status](PROJECT_STATUS.md)\n");
        self.put("docs/PROJECT_STATUS.md","Synthetic fixture.\n");
        self.put("src/main.ts","export const a = 1;\n");
        self.git(&["add","."]);
    }
    fn generated(&self) {
        self.expect(&["generate"],0);
        self.git(&["add","docs/inventory"]);
        self.expect(&["check","--index"],0);
    }
}
fn read(root:&Path,p:&str)->Vec<u8> {fs::read(root.join(p)).unwrap()}
#[test]
fn generation_snapshot_and_negative_cases() {
    let repo=Repo::new(); repo.fixture();
    assert_eq!(repo.expect(&["plan"],0)["outcome"],"PLAN_READY");
    assert!(!repo.0.join("docs/inventory").exists());
    repo.expect(&["check","--index"],1);
    repo.generated();
    let before=read(&repo.0,"docs/inventory/FILE_MANIFEST.tsv");
    assert_eq!(repo.expect(&["generate"],0)["changed_outputs"],serde_json::json!([]));
    assert_eq!(before,read(&repo.0,"docs/inventory/FILE_MANIFEST.tsv"));
    repo.git(&["commit","-qm","fixture"]);
    repo.expect(&["check"],0);
    repo.put("README.md","[missing](untracked.md)\n");
    repo.put("untracked.md","not staged\n");
    repo.expect(&["check","--index"],0);
    repo.git(&["add","README.md"]);
    let bad=repo.expect(&["check","--index"],1);
    assert!(bad["findings"].as_array().unwrap().iter().any(|f|f["code"]=="BROKEN_LINK"));
    repo.expect(&["check"],0);
    assert_eq!(before,read(&repo.0,"docs/inventory/FILE_MANIFEST.tsv"));
    repo.put("README.md","[index](docs/INDEX.md)\n");
    repo.git(&["add","README.md"]);
    repo.put("src/main.ts","export const a = 2;\n");
    repo.git(&["add","src/main.ts"]);
    repo.expect(&["check","--index"],1);
    repo.generated();
    repo.expect(&["check"],0);
}
#[test]
fn refuses_overwrite_and_unstaged_required() {
    let repo=Repo::new(); repo.fixture();
    repo.expect(&["init"],2);
    repo.git(&["rm","--cached","AGENTS.md"]);
    repo.expect(&["generate"],1);
    repo.git(&["add","AGENTS.md"]);
    repo.generated();
    repo.put("docs/inventory/FILE_MANIFEST.tsv","manual edit\n");
    let bad=repo.expect(&["generate"],1);
    assert!(bad["findings"].as_array().unwrap().iter().any(|f|f["code"]=="OUTPUT_DIRTY"));
    assert_eq!(read(&repo.0,"docs/inventory/FILE_MANIFEST.tsv"),b"manual edit\n");
}
