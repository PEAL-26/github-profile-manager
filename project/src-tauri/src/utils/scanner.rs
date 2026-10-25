use crate::models::profile::Profile;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Pastas ignoradas na varredura (doc 04 §3.1): diretórios conhecidos de
/// build/dependências + `.git` (o próprio marcador de repositório — não se
/// desce para o interior).
pub const IGNORED_DIRS: &[&str] = &[
    "node_modules",
    "target",
    ".venv",
    "vendor",
    "bin",
    "obj",
    // complementos às pastas de build conhecidas do doc
    "dist",
    "build",
    "__pycache__",
    ".git",
];

/// Repositório encontrado em disco durante a varredura.
pub struct FoundRepo {
    pub path: PathBuf,
    pub name: String,
}

/// Metadados Git lidos do repositório encontrado.
pub struct RepoMetadata {
    pub remote_url: Option<String>,
    pub git_name: Option<String>,
    pub git_email: Option<String>,
    pub ssh_command: Option<String>,
}

/// Varre as raízes recursivamente à procura de pastas que contêm `.git`,
/// respeitando a profundidade máxima (nº de subníveis) e a lista de ignores.
///
/// Não segue ligações simbólicas (evita ciclos) e deduplica repositórios
/// encontrados por várias raízes.
pub fn scan_roots(roots: &[PathBuf], max_depth: u32) -> Vec<FoundRepo> {
    let mut found: BTreeSet<PathBuf> = BTreeSet::new();

    for root in roots {
        if !root.is_dir() {
            continue;
        }
        walk(root, max_depth, &mut found);
    }

    found
        .into_iter()
        .map(|path| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());
            FoundRepo { path, name }
        })
        .collect()
}

fn walk(dir: &Path, depth_left: u32, out: &mut BTreeSet<PathBuf>) {
    if dir.join(".git").exists() {
        out.insert(dir.to_path_buf());
        return; // é um repositório: não desce para o interior
    }

    if depth_left == 0 {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return, // sem permissão / I/O — ignora silenciosamente
    };

    for entry in entries.flatten() {
        // file_type() não segue ligações simbólicas — evita ciclos.
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }
        let name = entry.file_name();
        if IGNORED_DIRS.contains(&name.to_string_lossy().as_ref()) {
            continue;
        }
        walk(&entry.path(), depth_left - 1, out);
    }
}

/// Lê remote URL e identidade local (`git config --local`) do repositório.
/// Subprocessos ocultos via `run_hidden`; qualquer falha vira `None`.
pub fn read_metadata(repo_path: &Path) -> RepoMetadata {
    RepoMetadata {
        remote_url: git_query(repo_path, &["remote", "get-url", "origin"]),
        git_name: git_query(repo_path, &["config", "--local", "user.name"]),
        git_email: git_query(repo_path, &["config", "--local", "user.email"]),
        ssh_command: git_query(repo_path, &["config", "--local", "core.sshCommand"]),
    }
}

/// Aplica a identidade de um perfil diretamente no `.git/config` do
/// repositório (Estratégia 1 do doc 04 §4.1).
pub fn inject_local_identity(
    repo_path: &Path,
    name: &str,
    email: &str,
    ssh_key_path: Option<&str>,
) -> Result<(), crate::error::AppError> {
    git_set(repo_path, &["user.name", name])?;
    git_set(repo_path, &["user.email", email])?;

    if let Some(key_path) = ssh_key_path {
        let ssh_command = format!("ssh -i \"{}\"", key_path);
        git_set(repo_path, &["core.sshCommand", &ssh_command])?;
    }

    Ok(())
}

fn git_set(repo_path: &Path, args: &[&str]) -> Result<(), crate::error::AppError> {
    let path_str = repo_path.to_string_lossy().to_string();
    let mut full: Vec<&str> = vec!["-C", &path_str, "config", "--local"];
    full.extend_from_slice(args);

    let output = crate::utils::process::run_hidden("git", &full)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(crate::error::AppError::SystemError(format!(
            "Falha ao configurar o repositório {}: {}",
            path_str,
            if stderr.is_empty() { "git config falhou" } else { &stderr }
        )));
    }
    Ok(())
}

fn git_query(repo_path: &Path, args: &[&str]) -> Option<String> {
    let path_str = repo_path.to_string_lossy().to_string();
    let mut full: Vec<&str> = vec!["-C", &path_str];
    full.extend_from_slice(args);

    let output = crate::utils::process::run_hidden("git", &full).ok()?;
    if !output.status.success() {
        return None; // valor não definido / sem remote / fora de um repo
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Resultado da detecção de perfil para um repositório (doc 04 §5 + critério
/// alinhado: e-mail primeiro, remote URL como fallback; se os dois apontarem
/// para perfis diferentes → `conflict` para escolha manual na UI).
pub struct Detection {
    pub suggested_profile_id: Option<String>,
    pub conflict: bool,
}

pub fn detect_profile(
    git_email: Option<&str>,
    remote_url: Option<&str>,
    profiles: &[Profile],
) -> Detection {
    let email_match = git_email.and_then(|email| {
        let email = email.trim();
        profiles.iter().find(|p| p.email.eq_ignore_ascii_case(email))
    });

    let remote_match = remote_url.and_then(parse_github_owner).and_then(|owner| {
        profiles
            .iter()
            .find(|p| p.github_username.eq_ignore_ascii_case(&owner))
    });

    let suggested = email_match.or(remote_match).map(|p| p.id.clone());
    let conflict = match (email_match, remote_match) {
        (Some(a), Some(b)) => a.id != b.id,
        _ => false,
    };

    Detection {
        suggested_profile_id: suggested,
        conflict,
    }
}

/// Extrai o owner (usuário/organização) de URLs remotas GitHub:
/// - `git@github.com:owner/repo.git`
/// - `https://github.com/owner/repo.git`
/// - `ssh://git@github.com/owner/repo.git`
pub fn parse_github_owner(remote_url: &str) -> Option<String> {
    let url = remote_url.trim();
    let lower = url.to_ascii_lowercase();

    // Formato SCP (git@github.com:owner/repo)
    if let Some(idx) = lower.find("github.com:") {
        let rest = &url[idx + "github.com:".len()..];
        return owner_from_path(rest);
    }

    // Formato com esquema (https://github.com/owner/repo)
    if let Some(idx) = lower.find("github.com/") {
        let rest = &url[idx + "github.com/".len()..];
        return owner_from_path(rest);
    }

    None
}

fn owner_from_path(rest: &str) -> Option<String> {
    let rest = rest.strip_suffix(".git").unwrap_or(rest);
    let owner = rest.split('/').next()?.trim();
    if owner.is_empty() {
        None
    } else {
        Some(owner.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    struct TempTree {
        root: PathBuf,
    }

    impl TempTree {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!("ghpm_scan_{}", Uuid::new_v4()));
            std::fs::create_dir_all(&root).unwrap();
            Self { root }
        }

        fn dir(&self, rel: &str) -> PathBuf {
            let p = self.root.join(rel);
            std::fs::create_dir_all(&p).unwrap();
            p
        }

        fn cleanup(&self) {
            std::fs::remove_dir_all(&self.root).ok();
        }
    }

    fn make_git_repo(dir: &Path) {
        std::fs::create_dir_all(dir.join(".git")).unwrap();
    }

    #[test]
    fn test_scan_finds_repos_and_respects_depth() {
        let tree = TempTree::new();
        make_git_repo(&tree.dir("proj-a"));
        make_git_repo(&tree.dir("nivel1/proj-b"));
        make_git_repo(&tree.dir("nivel1/nivel2/proj-c"));
        tree.dir("documentos"); // sem .git — ignorado

        // profundidade = nº de subníveis avaliados (doc 04 §3.1):
        // d=3 alcança nivel1/nivel2/proj-c
        let deep = scan_roots(std::slice::from_ref(&tree.root), 3);
        assert_eq!(deep.len(), 3);

        // d=2 alcança proj-a (nível 1) e proj-b (nível 2), não proj-c (nível 3)
        let mid = scan_roots(std::slice::from_ref(&tree.root), 2);
        let names: Vec<_> = mid.iter().map(|r| r.name.as_str()).collect();
        assert!(names.contains(&"proj-a"));
        assert!(names.contains(&"proj-b"));
        assert!(!names.contains(&"proj-c"));

        // d=1 só vê o que está imediatamente sob a raiz
        let shallow = scan_roots(std::slice::from_ref(&tree.root), 1);
        assert_eq!(shallow.len(), 1);
        assert_eq!(shallow[0].name, "proj-a");

        tree.cleanup();
    }

    #[test]
    fn test_scan_ignores_dependency_dirs() {
        let tree = TempTree::new();
        make_git_repo(&tree.dir("node_modules/oculto"));
        make_git_repo(&tree.dir("target/oculto"));
        make_git_repo(&tree.dir("meu-proj"));

        let found = scan_roots(std::slice::from_ref(&tree.root), 5);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "meu-proj");

        tree.cleanup();
    }

    #[test]
    fn test_scan_dedupes_overlapping_roots() {
        let tree = TempTree::new();
        make_git_repo(&tree.dir("grupo/proj"));

        let grupo = tree.root.join("grupo");
        let roots = vec![tree.root.clone(), grupo];
        let found = scan_roots(&roots, 3);

        assert_eq!(found.len(), 1, "raízes sobrepostas não duplicam resultados");
        tree.cleanup();
    }

    #[test]
    fn test_detect_prefers_email_and_flags_conflict() {
        let profiles = vec![
            profile("id-pessoal", "Pessoal", "eu@pessoal.com", "eu-gh"),
            profile("id-corp", "Corp", "eu@corp.com", "corp-gh"),
        ];

        // 1) e-mail casa → sugere esse perfil
        let d = detect_profile(Some("eu@corp.com"), None, &profiles);
        assert_eq!(d.suggested_profile_id.as_deref(), Some("id-corp"));
        assert!(!d.conflict);

        // 2) sem e-mail → fallback para owner do remote
        let d = detect_profile(None, Some("git@github.com:eu-gh/proj.git"), &profiles);
        assert_eq!(d.suggested_profile_id.as_deref(), Some("id-pessoal"));
        assert!(!d.conflict);

        // 3) e-mail e remote apontam para perfis diferentes → conflito
        let d = detect_profile(
            Some("eu@corp.com"),
            Some("https://github.com/eu-gh/proj.git"),
            &profiles,
        );
        assert_eq!(d.suggested_profile_id.as_deref(), Some("id-corp"));
        assert!(d.conflict);

        // 4) nada detetado
        let d = detect_profile(None, Some("git@gitlab.com:outro/x.git"), &profiles);
        assert!(d.suggested_profile_id.is_none());
        assert!(!d.conflict);
    }

    #[test]
    fn test_parse_github_owner_formats() {
        assert_eq!(
            parse_github_owner("git@github.com:peal/repo.git").as_deref(),
            Some("peal")
        );
        assert_eq!(
            parse_github_owner("https://github.com/PEAL-26/repo").as_deref(),
            Some("PEAL-26")
        );
        assert_eq!(
            parse_github_owner("ssh://git@github.com/org/repo.git").as_deref(),
            Some("org")
        );
        assert_eq!(parse_github_owner("git@gitlab.com:a/b.git"), None);
    }

    #[test]
    fn test_read_metadata_and_inject_local_identity() {
        // git disponível na VM de teste?
        if crate::utils::process::run_hidden("git", &["--version"]).is_err() {
            eprintln!("git indisponível — teste ignorado");
            return;
        }

        let tree = TempTree::new();
        let repo_dir = tree.dir("proj");
        // Repositório real (um .git vazio não basta para git config)
        let repo_str = repo_dir.to_string_lossy().to_string();
        let init = crate::utils::process::run_hidden("git", &["-C", &repo_str, "init", "-q"])
            .expect("git init falhou");
        assert!(init.status.success(), "git init devia ter sucesso");

        // Sem config local → tudo None
        let meta = read_metadata(&repo_dir);
        assert!(meta.git_email.is_none());

        // Injeção (Estratégia 1) e leitura de volta
        inject_local_identity(&repo_dir, "João Silva", "joao@corp.com", None).unwrap();
        let meta = read_metadata(&repo_dir);
        assert_eq!(meta.git_name.as_deref(), Some("João Silva"));
        assert_eq!(meta.git_email.as_deref(), Some("joao@corp.com"));

        tree.cleanup();
    }

    fn profile(id: &str, name: &str, email: &str, gh: &str) -> Profile {
        Profile {
            id: id.to_string(),
            name: name.to_string(),
            github_username: gh.to_string(),
            email: email.to_string(),
            host_alias: format!("alias-{}", id),
            ssh_key_id: None,
            default_directory: None,
            default_branch: "main".to_string(),
            description: None,
            avatar_url: None,
            is_active: false,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}
