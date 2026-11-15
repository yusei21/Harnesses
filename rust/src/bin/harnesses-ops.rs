//! Read-only operations. No side-effect tools and no auto-generation of Skills.
use harnesses_core::ops::{audit_catalog, analyze, load_catalog, load_events, suggest};
use serde_json::json;
use std::path::PathBuf;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let result = match args.first().map(String::as_str) {
        Some("catalog") if args.len()==1 => {
            load_catalog(&root).and_then(|c| serde_json::to_value(&c.capabilities).map_err(|_|"serialize_error".into()))
        }
        Some("suggest") if args.len()>=2 => {
            load_catalog(&root).and_then(|c| serde_json::to_value(suggest(&c,&args[1..].join(" "),5)).map_err(|_|"serialize_error".into()))
        }
        Some("audit") if args.len()==1 => load_catalog(&root)
            .and_then(|c|serde_json::to_value(audit_catalog(&root,&c)).map_err(|_|"serialize_error".into())),
        Some("insights") if args.len()==2 => load_events(&PathBuf::from(&args[1]))
            .and_then(|events|serde_json::to_value(analyze(&events)).map_err(|_|"serialize_error".into())),
        _ => Err("Usage: harnesses-ops catalog|suggest QUERY|audit|insights EVENTS.json".into())
    };
    match result {
        Ok(value) => {
            println!("{}",serde_json::to_string_pretty(&value).unwrap_or_default());
            if value.get("status").and_then(|v|v.as_str())==Some("failed") {std::process::exit(1);}
        }
        Err(e)=>{eprintln!("{}",json!({"error":e}));std::process::exit(2);}
    }
}
