use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// タイムラインイベントの種別
#[derive(ValueEnum, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EventKind {
    /// 脆弱性を発見した初期状態
    Discovered,
    /// ベンダー/開発者へ報告済み (連絡先確認・初回連絡)
    Reported,
    /// ベンダーが受領を確認した
    Acknowledged,
    /// ベンダーが再現・存在を確認した
    Triaged,
    /// 修正作業が進行中
    InProgress,
    /// パッチが用意された
    Fixed,
    /// パッチがリリース・配布された
    Patched,
    /// 一般公開された
    Disclosed,
    /// 開示期限を過ぎた
    Overdue,
    /// ベンダーが応答しない / 対応拒否
    Unresponsive,
    /// 重複報告として扱われた
    Duplicate,
    /// 脆弱性ではないと判断された
    NotApplicable,
    /// 修正されずクローズ
    WontFix,
    /// 上記に当てはまらない自由記述の記録
    Note,
}
