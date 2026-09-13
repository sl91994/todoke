use serde::Serialize;

use crate::model::slug::Slug;

#[allow(dead_code)]
#[derive(Serialize)]
pub enum Severity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

/// case.toml のひな型
#[derive(Serialize)]
pub struct Case {
    pub meta: Meta,
    pub affected: Affected,
    pub ids: Ids,
    pub timeline: Vec<TimelineEntry>,
}

/// ケースの基本情報
#[derive(Serialize)]
pub struct Meta {
    pub slug: String,
    pub title: String,
    pub severity: Severity,
    pub summary: String, // 本文とは別のcaseの簡単な概要
    /// PoCが用意されているか
    pub poc: bool,
    pub reporter: String,
}

/// 影響を受けるソフトウェア
#[derive(Serialize)]
pub struct Affected {
    pub name: String,
    pub ecosystem: String,
    pub versions: String,
    pub patched: String,
}

/// 採番された脆弱性ID
#[derive(Serialize)]
pub struct Ids {
    pub cve: String,
    pub ghsa: String,
}

/// タイムラインの1件
#[derive(Serialize)]
pub struct TimelineEntry {
    pub date: String,
    pub kind: String,
    pub note: String,
}

impl Case {
    pub fn new(slug: &Slug, title: String, date: String) -> Self {
        Self {
            meta: Meta {
                slug: slug.as_str().to_string(),
                title,
                severity: Severity::Unknown,
                summary: String::new(),
                poc: false,
                reporter: String::new(),
            },
            affected: Affected {
                name: String::new(),
                ecosystem: String::new(),
                versions: String::new(),
                patched: String::new(),
            },
            ids: Ids {
                cve: String::new(),
                ghsa: String::new(),
            },
            timeline: vec![TimelineEntry {
                date, // newコマンドでの作成時はそのUTC時刻
                kind: String::new(),
                note: String::new(),
            }],
        }
    }
}
