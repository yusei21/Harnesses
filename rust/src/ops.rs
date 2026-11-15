//! Local-first capability discovery, structural audit and offline insights.
//! No execution of Skills, third-party hooks, external URLs or user-provided commands.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

const MAX_CATALOG: u64 = 256 * 1024;
const MAX_EVENTS: u64 = 1024 * 1024;
const VALID_DOMAINS: &[&str] = &["planning","research","engineering","security","quality","intelligence","integrations","orchestration","machine-learning","rag","ai","saas","frontend","backend"];

#[derive(Debug, Deserialize)]
pub struct Catalog {
    pub schema_version: u32,
    pub capabilities: Vec<Capability>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Capability {
    pub id: String,
    pub domain: String,
    pub kind: String,
    pub triggers: Vec<String>,
    pub path: String,
    pub docs: String,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Finding {
    pub severity: String,
    pub code: String,
    pub location: String,
    pub message: String,
}
#[derive(Debug, Serialize)]
pub struct Audit {
    pub status: String,
    pub checked: usize,
    pub findings: Vec<Finding>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub kind: String,
    pub component: String,
}
#[derive(Debug, Serialize)]
pub struct Suggestion {
    pub proposal: String,
    pub component: String,
    pub evidence_count: usize,
    pub recommendation: String,
}
#[derive(Debug, Serialize)]
pub struct Insights {
    pub observations: usize,
    pub suggestions: Vec<Suggestion>,
}

fn is_slug(s: &str) -> bool {
    !s.is_empty() && s.len() <= 80
        && !s.starts_with('-') && !s.ends_with('-')
        && s.as_bytes().iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}
fn validated_rel(s: &str) -> Option<PathBuf> {
    let p = Path::new(s);
    if p.is_absolute() || s.contains('\\') { return None; }
    let mut count = 0;
    for part in p.components() {
        if !matches!(part, std::path::Component::Normal(_)) { return None; }
        count += 1;
    }
    if count == 0 { None } else { Some(p.to_owned()) }
}
fn within_root(root: &Path, rel: &str) -> Result<PathBuf, String> {
    let p = validated_rel(rel).ok_or("invalid_relative_path")?;
    let root = root.canonicalize().map_err(|_| "invalid_root")?;
    let full = root.join(p).canonicalize().map_err(|_| "missing_path")?;
    if !full.starts_with(root) || !full.is_file() {
        return Err("unsafe_or_missing_path".into());
    }
    Ok(full)
}
fn read_bounded(path: &Path, limit: u64) -> Result<String, String> {
    let meta = fs::metadata(path).map_err(|_| "missing_file")?;
    if meta.len() > limit { return Err("file_too_large".into()); }
    fs::read_to_string(path).map_err(|_| "unreadable_file".into())
}
pub fn load_catalog(root: &Path) -> Result<Catalog, String> {
    let file = within_root(root, "capabilities/catalog.json")?;
    let raw = read_bounded(&file, MAX_CATALOG)?;
    let catalog: Catalog = serde_json::from_str(&raw).map_err(|_| "invalid_catalog_json")?;
    if catalog.schema_version != 1 || catalog.capabilities.len() > 300 {
        return Err("unsupported_catalog".into());
    }
    Ok(catalog)
}
pub fn suggest<'a>(catalog: &'a Catalog, query: &str, limit: usize) -> Vec<&'a Capability> {
    let tokens: Vec<String> = query.to_lowercase().split(|c:char| !c.is_alphanumeric())
        .filter(|s| s.len() >= 3).map(str::to_owned).collect();
    let mut ranked: Vec<(usize, &Capability)> = catalog.capabilities.iter().filter_map(|cap| {
        let score = tokens.iter().filter(|tok| cap.triggers.iter().any(|t|
            t.to_lowercase().contains(tok.as_str()) || tok.contains(&t.to_lowercase())
        )).count();
        if score == 0 { None } else { Some((score, cap)) }
    }).collect();
    ranked.sort_by(|a,b| b.0.cmp(&a.0).then_with(|| a.1.id.cmp(&b.1.id)));
    ranked.into_iter().take(limit.min(10)).map(|x| x.1).collect()
}
pub fn audit_catalog(root: &Path, catalog: &Catalog) -> Audit {
    let mut findings = Vec::new();
    let mut ids = HashSet::new();
    let mut paths = HashSet::new();
    for cap in &catalog.capabilities {
        let loc = cap.id.clone();
        if !is_slug(&cap.id) || !ids.insert(cap.id.clone()) {
            findings.push(Finding{severity:"error".into(),code:"invalid_or_duplicate_id".into(),
                location:loc.clone(),message:"Capability id invalid or duplicated".into()});
        }
        if !VALID_DOMAINS.contains(&cap.domain.as_str()) || cap.kind != "skill" || cap.triggers.is_empty() {
            findings.push(Finding{severity:"error".into(),code:"invalid_metadata".into(),
                location:loc.clone(),message:"Domain, kind or triggers invalid".into()});
        }
        if !paths.insert(cap.path.clone()) {
            findings.push(Finding{severity:"error".into(),code:"duplicate_path".into(),
                location:loc.clone(),message:"Two capabilities reference one Skill".into()});
        }
        let expected_skill = format!("skills/{}/SKILL.md", cap.id);
        let expected_docs = format!("docs/skills/{}.md", cap.id);
        if cap.path != expected_skill || cap.docs != expected_docs {
            findings.push(Finding{severity:"error".into(),code:"unexpected_location".into(),
                location:loc.clone(),message:"Path must match canonical id".into()});
        }
        match within_root(root, &cap.path).and_then(|p| read_bounded(&p,64*1024)) {
            Ok(text) => {
                if !text.starts_with("---\n") ||
                    !text.lines().any(|line| line.trim()==format!("name: {}",cap.id)) ||
                    !text.lines().any(|line| line.starts_with("description:")) {
                    findings.push(Finding{severity:"error".into(),code:"invalid_skill".into(),
                        location:loc.clone(),message:"SKILL.md missing frontmatter/name/description".into()});
                }
            }
            Err(error) => findings.push(Finding{severity:"error".into(),code:"skill_missing_or_unsafe".into(),
                location:loc.clone(),message:error}),
        }
        if let Err(error) = within_root(root, &cap.docs) {
            findings.push(Finding{severity:"error".into(),code:"docs_missing_or_unsafe".into(),
                location:loc.clone(),message:error});
        }
    }
    // Structural mismatch: old Python is still described as primary in per-agent manuals.
    let manual_dir = root.join("docs/agents");
    if let Ok(entries) = fs::read_dir(manual_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|x| x == "md") {
                if let Ok(text) = read_bounded(&path,64*1024) {
                    if text.contains("python -m harness.cli run") && !text.contains("cargo run --manifest-path") {
                        findings.push(Finding{severity:"warning".into(),code:"stale_python_primary".into(),
                            location:path.file_name().unwrap_or_default().to_string_lossy().into(),
                            message:"Agent manual uses only legacy Python CLI despite Rust-first architecture".into()});
                    }
                }
            }
        }
    }
    findings.sort_by(|a,b| a.location.cmp(&b.location).then_with(|| a.code.cmp(&b.code)));
    let status = if findings.iter().any(|f| f.severity == "error") {"failed"} else {"passed"};
    Audit {status:status.into(),checked:catalog.capabilities.len(),findings}
}
pub fn analyze(observations: &[Observation]) -> Insights {
    let mut counts: BTreeMap<(String,String),usize> = BTreeMap::new();
    for item in observations {
        if !is_slug(&item.component) || !matches!(item.kind.as_str(),
             "repeated_workflow" | "test_failure" | "stale_documentation" | "tool_denied") {continue;}
        *counts.entry((item.kind.clone(),item.component.clone())).or_default() += 1;
    }
    let suggestions = counts.into_iter().filter(|(_,count)| *count >= 2)
      .map(|((kind,component), count)| {
        let (proposal,recommendation) = match kind.as_str() {
            "repeated_workflow" => ("skill_candidate","Review whether repeated steps justify a reusable Skill"),
            "test_failure" => ("quality_gate_candidate","Review tests or a deterministic pre-commit hook"),
            "stale_documentation" => ("consistency_audit","Investigate documentation drift against code"),
            _ => ("permission_review","Review denied tool usage; never auto-grant access"),
        };
        Suggestion {proposal:proposal.into(),component,evidence_count:count,
                    recommendation:recommendation.into()}
      }).collect();
    Insights {observations: observations.len(),suggestions}
}
pub fn load_events(path: &Path) -> Result<Vec<Observation>, String> {
    let raw = read_bounded(path, MAX_EVENTS)?;
    let items: Vec<Observation> = serde_json::from_str(&raw).map_err(|_| "invalid_events_json")?;
    if items.len() > 10000 { return Err("too_many_events".into()); }
    Ok(items)
}
