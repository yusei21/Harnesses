use harnesses_core::ops::{audit_catalog,analyze,load_catalog,suggest,Observation};
use std::path::PathBuf;
fn root()->PathBuf {PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")}
#[test] fn real_catalog_is_valid() {
    let c=load_catalog(&root()).unwrap();
    assert_eq!(c.capabilities.len(),12);
    let a=audit_catalog(&root(),&c);
    assert_eq!(a.status,"passed","{:?}",a.findings);
    assert!(a.findings.iter().all(|x|x.severity!="error"));
}
#[test] fn suggestion_is_selective() {
    let c=load_catalog(&root()).unwrap();
    let hits=suggest(&c,"auditoria de documentacao e contradicao",3);
    assert!(!hits.is_empty());
    assert_eq!(hits[0].id,"consistency-audit");
    assert!(suggest(&c,"xyz987unknown",5).is_empty());
}
#[test] fn insights_requires_recurrence() {
    let events=vec![
      Observation{kind:"test_failure".into(),component:"cargo-test".into()},
      Observation{kind:"test_failure".into(),component:"cargo-test".into()},
      Observation{kind:"tool_denied".into(),component:"github".into()},
      Observation{kind:"unknown".into(),component:"github".into()}
    ];
    let result=analyze(&events);
    assert_eq!(result.suggestions.len(),1);
    assert_eq!(result.suggestions[0].proposal,"quality_gate_candidate");
    assert_eq!(result.suggestions[0].evidence_count,2);
}
#[test] fn duplicated_catalog_is_rejected() {
    let mut c=load_catalog(&root()).unwrap();
    c.capabilities.push(c.capabilities[0].clone());
    let a=audit_catalog(&root(),&c);
    assert_eq!(a.status,"failed");
    assert!(a.findings.iter().any(|f|f.code=="invalid_or_duplicate_id"));
}
#[test] fn traversal_is_rejected() {
    let mut c=load_catalog(&root()).unwrap();
    c.capabilities[0].path="../secret".into();
    let a=audit_catalog(&root(),&c);
    assert_eq!(a.status,"failed");
}
