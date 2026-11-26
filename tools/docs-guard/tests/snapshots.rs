use std::{fs, path::{Path,PathBuf}, process::{Command,Output}};
struct Repo { root: PathBuf, _temp: tempfile::TempDir }
impl Repo {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let repo = Self { root: temp.path().to_path_buf(), _temp: temp };
        repo.git(&["init","-q"]);
        repo.git(&["config","user.name","Test"]);
        repo.git(&["config","user.email","test@example.invalid"]);
        repo.git(&["config","core.autocrlf","false"]);
        repo.git(&["config","commit.gpgsign","false"]);
        repo
    }
    fn git(&self,args:&[&str]) { let out=Command::new("git").current_dir(&self.root).args(args).output().unwrap(); assert!(out.status.success(),"{:?}: {}",args,String::from_utf8_lossy(&out.stderr)); }
    fn put(&self,path:&str,text:&str) { let p=self.root.join(path); fs::create_dir_all(p.parent().unwrap()).unwrap(); fs::write(p,text).unwrap(); }
    fn run(&self,args:&[&str]) -> Output { Command::new(env!("CARGO_BIN_EXE_docs-guard")).arg("--root").arg(&self.root).arg("--json").args(args).output().unwrap() }
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
    assert!(!repo.root.join("docs/inventory").exists());
    repo.expect(&["check","--index"],1);
    repo.generated();
    let before=read(&repo.root,"docs/inventory/FILE_MANIFEST.tsv");
    assert_eq!(repo.expect(&["generate"],0)["changed_outputs"],serde_json::json!([]));
    assert_eq!(before,read(&repo.root,"docs/inventory/FILE_MANIFEST.tsv"));
    repo.git(&["commit","-qm","fixture"]);
    repo.expect(&["check"],0);
    repo.put("README.md","[missing](untracked.md)\n");
    repo.put("untracked.md","not staged\n");
    repo.expect(&["check","--index"],0);
    repo.git(&["add","README.md"]);
    let bad=repo.expect(&["check","--index"],1);
    assert!(bad["findings"].as_array().unwrap().iter().any(|f|f["code"]=="BROKEN_LINK"));
    repo.expect(&["check"],0);
    assert_eq!(before,read(&repo.root,"docs/inventory/FILE_MANIFEST.tsv"));
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
    assert_eq!(read(&repo.root,"docs/inventory/FILE_MANIFEST.tsv"),b"manual edit\n");
}

#[test]
fn cli_help_errors_and_json_contract() {
    let bin = env!("CARGO_BIN_EXE_docs-guard");
    for flag in ["--help","--version"] {
        let out=Command::new(bin).arg(flag).output().unwrap();
        assert!(out.status.success());
        assert!(!out.stdout.is_empty());
        assert!(out.stderr.is_empty());
    }
    let out=Command::new(bin).args(["--json","check","--index","--commit","HEAD"]).output().unwrap();
    assert_eq!(out.status.code(),Some(2));
    assert!(out.stdout.is_empty());
    assert!(!out.stderr.is_empty());
    let repo=Repo::new();
    assert_eq!(repo.expect(&["doctor"],0)["schema_version"],1);
    repo.fixture();
    let report=repo.expect(&["check","--index"],1);
    assert_eq!(report["schema_version"],1);
    assert_eq!(report["outcome"],"FAIL");
    let out=Command::new(bin).arg("--root").arg(&repo.root).args(["check","--index"]).output().unwrap();
    assert_eq!(out.status.code(),Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("MISSING_GENERATED"));
}
#[test]
fn refuses_to_overwrite_unmanaged_tracked_output() {
    let repo=Repo::new(); repo.fixture();
    repo.put("docs/inventory/DOCUMENT_REGISTER.md","User-authored document\n");
    repo.git(&["add","docs/inventory/DOCUMENT_REGISTER.md"]);
    let report=repo.expect(&["generate"],1);
    assert!(report["findings"].as_array().unwrap().iter().any(|f|f["code"]=="OUTPUT_UNMANAGED"));
    assert_eq!(read(&repo.root,"docs/inventory/DOCUMENT_REGISTER.md"),b"User-authored document\n");
    assert!(!repo.root.join("docs/inventory/FILE_MANIFEST.tsv").exists());
}
#[test]
fn rejects_git_scope_overrides() {
    let repo=Repo::new(); repo.fixture();
    let out=Command::new(env!("CARGO_BIN_EXE_docs-guard")).arg("--root").arg(&repo.root)
        .env("GIT_INDEX_FILE",repo.root.join("wrong-index")).args(["--json","check","--index"]).output().unwrap();
    assert_eq!(out.status.code(),Some(2));
    let report:serde_json::Value=serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["code"],"EXECUTION_ERROR");
    assert!(report["detail"].as_str().unwrap().contains("GIT_INDEX_FILE"));
}

#[test]
fn adopts_only_reviewed_legacy_objects() {
    let repo=Repo::new(); repo.fixture();
    let register="docs/inventory/DOCUMENT_REGISTER.md";
    let manifest="docs/inventory/FILE_MANIFEST.tsv";
    repo.put(register,"Legacy generated register\n");
    repo.put(manifest,"Legacy generated manifest\n");
    repo.git(&["add","docs/inventory"]);
    let oid=|path:&str| {
        let out=Command::new("git").current_dir(&repo.root).args(["rev-parse",&format!(":{path}")]).output().unwrap();
        assert!(out.status.success());
        String::from_utf8(out.stdout).unwrap().trim().to_owned()
    };
    let r=oid(register);
    let m=oid(manifest);
    let args=["--adopt-register",r.as_str(),"--adopt-manifest",m.as_str()];
    let bad=repo.expect(&["generate","--adopt-register","wrong","--adopt-manifest",&m],1);
    assert!(bad["findings"].as_array().unwrap().iter().any(|f|f["code"]=="ADOPTION_MISMATCH"));
    let report=repo.expect(&[&["plan"][..],&args].concat(),0);
    assert_eq!(report["legacy_adoptions"].as_array().unwrap().len(),2);
    assert_eq!(read(&repo.root,register),b"Legacy generated register\n");
    repo.expect(&[&["generate"][..],&args].concat(),0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index"],0);
    repo.expect(&[&["generate"][..],&args].concat(),1);
}
#[test]
fn registers_root_and_nonstandard_docs_and_rename_delete() {
    let repo=Repo::new(); repo.fixture();
    repo.put("CHANGELOG.md","Release notes\n");
    repo.put("manual/user guide.md","Guide\n");
    repo.put("notes/optional.txt","Optional evidence\n");
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture")
        .replace("optional_documents = []","optional_documents = [\"notes/optional.txt\"]");
    repo.put("docs-guard.toml",&cfg);
    repo.git(&["add","."]);
    repo.generated();
    let path="docs/inventory/DOCUMENT_REGISTER.md";
    let text=String::from_utf8(read(&repo.root,path)).unwrap();
    assert!(text.contains("CHANGELOG.md"));
    assert!(text.contains("user%20guide.md"));
    assert!(text.contains("notes/optional.txt"));
    repo.git(&["mv","manual/user guide.md","manual/renamed.md"]);
    repo.git(&["rm","CHANGELOG.md"]);
    repo.expect(&["check","--index"],1);
    repo.generated();
    let text=String::from_utf8(read(&repo.root,path)).unwrap();
    assert!(text.contains("manual/renamed.md"));
    assert!(!text.contains("CHANGELOG.md"));
    assert!(!text.contains("user%20guide.md"));
}
#[test]
fn refuses_clean_filter_and_required_exclusion() {
    let repo=Repo::new(); repo.fixture();
    repo.put(".gitattributes","docs/inventory/* filter=custom\n");
    repo.git(&["add",".gitattributes"]);
    let result=repo.expect(&["generate"],2);
    assert!(result["detail"].as_str().unwrap().contains("attribute"));
    assert!(!repo.root.join("docs/inventory").exists());
    repo.git(&["rm","-f",".gitattributes"]);
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture")
        .replace("link_exclusions = []","link_exclusions = [\"README.md\"]");
    repo.put("docs-guard.toml",&cfg);
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["check","--index"],2);
}

#[test]
fn change_rules_use_trusted_base_and_keep_old_policy() {
    let repo=Repo::new(); repo.fixture();
    repo.generated();
    repo.git(&["commit","-qm","baseline"]);
    // Introduce rule in a policy-only commit first.
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture")
        + "\n[[change_rules]]\nid = \"source-status\"\nwatch = [\"src\"]\nrequire = [\"docs/PROJECT_STATUS.md\"]\nreview_document = \"AGENTS.md\"\n";
    // The example declares an empty array; TOML cannot redeclare it as array of tables.
    let cfg=cfg.replace("change_rules = []\n","");
    repo.put("docs-guard.toml",&cfg);
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index","--base","HEAD"],0);
    repo.git(&["commit","-qm","policy"]);
    repo.put("src/main.ts","export const a = 3;\n");
    repo.git(&["add","src/main.ts"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index"],2);
    let report=repo.expect(&["check","--index","--base","HEAD"],1);
    assert_eq!(report["change_rules_checked"],true);
    assert!(report["findings"].as_array().unwrap().iter().any(|f|f["code"]=="DOCUMENT_CHANGE_REQUIRED"));
    // Removing the rule in the same change does not bypass the trusted base rule.
    repo.put("docs-guard.toml",&include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture"));
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index","--base","HEAD"],1);
    repo.put("docs/PROJECT_STATUS.md","Source change reviewed.\n");
    repo.git(&["add","docs/PROJECT_STATUS.md"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index","--base","HEAD"],0);
    repo.git(&["commit","-qm","source and documentation"]);
    repo.expect(&["check","--base","HEAD~1"],0);
}

#[test]
fn reports_manifest_drift_and_review_alternative() {
    let repo=Repo::new(); repo.fixture();
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture")
        .replace("change_rules = []\n","")
        + "\n[[change_rules]]\nid = \"source-status\"\nwatch = [\"src\"]\nrequire = [\"docs/PROJECT_STATUS.md\"]\nreview_document = \"AGENTS.md\"\n";
    // Bootstrap on a prior commit without configured rules.
    repo.generated();
    repo.git(&["commit","-qm","baseline"]);
    repo.put("docs-guard.toml",&cfg);
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index","--base","HEAD"],0);
    repo.git(&["commit","-qm","policy"]);
    repo.git(&["rm","src/main.ts"]);
    repo.put("src/new.ts","export const b = 2;\n");
    repo.git(&["add","src/new.ts"]);
    let report=repo.expect(&["check","--index","--base","HEAD"],1);
    let findings=report["findings"].as_array().unwrap();
    assert!(findings.iter().any(|f| f["code"]=="STALE_GENERATED" && f["detail"].as_str().unwrap().contains("src/main.ts")));
    assert!(findings.iter().any(|f| f["code"]=="DOCUMENT_CHANGE_REQUIRED"));
    repo.put("AGENTS.md","Reviewed source-only change; no status update required.\n");
    repo.git(&["add","AGENTS.md"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.expect(&["check","--index","--base","HEAD"],0);
}
#[test]
fn rejects_nonportable_generated_path_and_oversize_markdown() {
    let repo=Repo::new(); repo.fixture();
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture")
        .replace("docs/inventory/DOCUMENT_REGISTER.md","docs/inventory/CON.md");
    repo.put("docs-guard.toml",&cfg);
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["generate"],2);
    assert!(!repo.root.join("docs/inventory").exists());
    repo.put("docs-guard.toml",&include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture"));
    repo.put("docs/large.md",&"x".repeat(4 * 1024 * 1024 + 1));
    repo.git(&["add","."]);
    let report=repo.expect(&["generate"],2);
    assert!(report["detail"].as_str().unwrap().contains("size limit"));
}

#[test]
fn read_only_checks_preserve_index_and_worktree() {
    let repo=Repo::new(); repo.fixture();
    repo.generated();
    repo.git(&["commit","-qm","baseline"]);
    repo.put("README.md","unstaged edit\n");
    let inspect=|args:&[&str]| {
        let out=Command::new("git").current_dir(&repo.root).args(args).output().unwrap();
        assert!(out.status.success());
        out.stdout
    };
    let status=inspect(&["status","--porcelain=v1","-z"]);
    let index=inspect(&["ls-files","--stage","-z"]);
    let manifest=read(&repo.root,"docs/inventory/FILE_MANIFEST.tsv");
    let register=read(&repo.root,"docs/inventory/DOCUMENT_REGISTER.md");
    repo.expect(&["plan"],0);
    repo.expect(&["check"],0);
    repo.expect(&["check","--index"],0);
    repo.expect(&["status","--index"],0);
    assert_eq!(status,inspect(&["status","--porcelain=v1","-z"]));
    assert_eq!(index,inspect(&["ls-files","--stage","-z"]));
    assert_eq!(manifest,read(&repo.root,"docs/inventory/FILE_MANIFEST.tsv"));
    assert_eq!(register,read(&repo.root,"docs/inventory/DOCUMENT_REGISTER.md"));
    assert_eq!(read(&repo.root,"README.md"),b"unstaged edit\n");
}

#[test]
fn rejects_generated_alias_and_nested_repository() {
    let repo=Repo::new(); repo.fixture();
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture");
    repo.put("docs-guard.toml",&cfg.replace("docs/inventory/DOCUMENT_REGISTER.md","readme.md"));
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["generate"],2);
    assert_eq!(read(&repo.root,"README.md"),b"[index](docs/INDEX.md)\n");
    repo.put("docs-guard.toml",&cfg.replace("docs/inventory/DOCUMENT_REGISTER.md","nested/register.md"));
    repo.git(&["add","docs-guard.toml"]);
    fs::create_dir_all(repo.root.join("nested/.git")).unwrap();
    repo.expect(&["generate"],2);
    assert!(!repo.root.join("nested/register.md").exists());
    assert!(!repo.root.join("docs/inventory").exists());
}
#[test]
fn rejects_unstaged_attribute_drift_and_oversize_config() {
    let repo=Repo::new(); repo.fixture();
    repo.put(".gitattributes","docs/inventory/* eol=lf\n");
    repo.git(&["add",".gitattributes"]);
    repo.put(".gitattributes","docs/inventory/* filter=custom\n");
    let error=repo.expect(&["generate"],2);
    assert!(error["detail"].as_str().unwrap().contains("attributes differ"));
    assert!(!repo.root.join("docs/inventory").exists());
    repo.put("docs-guard.toml",&format!("{}\n# {}",include_str!("../examples/docs-guard.toml"),"x".repeat(256*1024)));
    repo.git(&["add","docs-guard.toml"]);
    let error=repo.expect(&["check","--index"],2);
    assert!(error["detail"].as_str().unwrap().contains("size limit"));
}
#[test]
fn mode_only_document_change_cannot_satisfy_rule() {
    let repo=Repo::new(); repo.fixture(); repo.generated();
    repo.git(&["commit","-qm","baseline"]);
    let cfg=include_str!("../examples/docs-guard.toml").replace("replace-with-project-id","fixture").replace("change_rules = []\n","")
        + "\n[[change_rules]]\nid = \"source-status\"\nwatch = [\"src\"]\nrequire = [\"docs/PROJECT_STATUS.md\"]\n";
    repo.put("docs-guard.toml",&cfg);
    repo.git(&["add","docs-guard.toml"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    repo.git(&["commit","-qm","policy"]);
    repo.put("src/main.ts","export const a = 4;\n");
    repo.git(&["add","src/main.ts"]);
    repo.git(&["update-index","--chmod=+x","docs/PROJECT_STATUS.md"]);
    repo.expect(&["generate"],0);
    repo.git(&["add","docs/inventory"]);
    let report=repo.expect(&["check","--index","--base","HEAD"],1);
    assert!(report["findings"].as_array().unwrap().iter().any(|f|f["code"]=="DOCUMENT_CHANGE_REQUIRED"));
}
