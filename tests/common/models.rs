//! The model builders that match the standard's example documents.

use ops::{Status, Task};
use serde_json::{Map, Value, json};

/// `json!` yields a Value; Task.metadata is a Map.
pub fn meta(v: Value) -> Map<String, Value> {
    v.as_object().unwrap().clone()
}

/// The model behind the standard's example documents (tree.json,
/// yaml.yaml, jsonl.jsonl, csv.csv, markdown.md, xml.xml, ...).
pub fn example_model() -> Task {
    Task {
        title: "Acme product backlog".into(),
        status: Status::Open,
        version: Some("0".into()),
        id: Some("export-root".into()),
        metadata: Some(meta(json!({
            "created_at": "2026-01-01T09:00:00Z",
            "updated_at": "2026-07-31T18:00:00Z",
            "url": "https://acme.example/backlog",
        }))),
        subtasks: vec![Task {
            id: Some("t1".into()),
            title: "Fix memory leak in auth service".into(),
            status: Status::Open,
            metadata: Some(meta(json!({
                "priority": "high",
                "start_date": "2026-07-01",
                "due_date": "2026-08-15",
                "tags": ["infrastructure", "auth"],
                "status": "In review",
                "assignee": "ada",
            }))),
            subtasks: vec![
                Task {
                    id: Some("t1r".into()),
                    title: "Reproduce the leak".into(),
                    status: Status::Open,
                    metadata: Some(meta(json!({
                        "priority": "high",
                        "location": "Remote",
                    }))),
                    ..Default::default()
                },
                Task {
                    id: Some("t1a".into()),
                    title: "Ship the fix".into(),
                    status: Status::Done,
                    metadata: Some(meta(json!({
                        "priority": "high",
                        "estimates": 3,
                        "attachments": ["notes/fix-auth.pdf"],
                        "completed_at": "2026-08-15T17:00:00Z",
                    }))),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// The markdown and nested-xml mappings drop ids.
pub fn strip_ids(t: &mut Task) {
    t.id = None;
    for c in &mut t.subtasks {
        strip_ids(c);
    }
}

/// The CSV mapping has no version column.
pub fn strip_versions(t: &mut Task) {
    t.version = None;
    for c in &mut t.subtasks {
        strip_versions(c);
    }
}

pub fn example_no_ids() -> Task {
    let mut t = example_model();
    strip_ids(&mut t);
    t
}

/// The markdown mapping also carries the root notes paragraph.
pub fn example_markdown() -> Task {
    let mut t = example_no_ids();
    t.notes = Some(
        "The product backlog for the Acme platform, kept in the Open Productivity Standard (OPS)."
            .into(),
    );
    t
}

pub fn example_no_version() -> Task {
    let mut t = example_model();
    strip_versions(&mut t);
    t
}

/// Flat forms never attach an id to roots.
pub fn example_root_no_id() -> Task {
    let mut t = example_model();
    t.id = None;
    t
}

/// The CSV mapping: no versions, and flat forms never attach ids to
/// roots.
pub fn example_csv() -> Task {
    let mut t = example_root_no_id();
    strip_versions(&mut t);
    t
}

/// The `Work` root of the multi-root examples (t1's subtree is shared
/// with the single-root examples).
pub fn work_model() -> Task {
    let mut t = example_root_no_id();
    t.title = "Work".into();
    t
}

/// A leaf task with an id, as found in the flat examples.
pub fn chore(id: &str, title: &str, status: Status) -> Task {
    Task {
        id: Some(id.into()),
        title: title.into(),
        status,
        ..Default::default()
    }
}

/// The `Personal` root of the multi-root JSONL example.
pub fn personal_model() -> Task {
    Task {
        title: "Personal".into(),
        status: Status::Open,
        version: Some("0".into()),
        subtasks: vec![
            chore("p1", "Buy groceries", Status::Open),
            chore("p2", "Call dentist", Status::Done),
        ],
        ..Default::default()
    }
}

/// The `Personal` root of the multi-root CSV example: its row lists no
/// child ids (upstream quirk), so p1/p2 resolve as their own roots.
pub fn personal_csv_model() -> Task {
    Task {
        title: "Personal".into(),
        status: Status::Open,
        ..Default::default()
    }
}

/// The empty export: a root with no subtasks.
pub fn empty_model() -> Task {
    Task {
        title: "Empty export".into(),
        status: Status::Open,
        version: Some("0".into()),
        ..Default::default()
    }
}
