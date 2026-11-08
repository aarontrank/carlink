//! SQLite history of scans and actions, for trends over time.

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;
use serde_json::Value;
use std::path::Path;

pub struct Store {
    db: Connection,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoredReading {
    pub source: String,
    pub key: String,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoredDtc {
    pub code: String,
    pub kind: String,
    pub ecu: String,
}

pub struct NewSession {
    pub vehicle: String,
    pub kind: String,
    /// RFC 3339 UTC; `None` means now.
    pub at: Option<String>,
    pub odometer_mi: Option<f64>,
    pub note: Option<String>,
    pub report: Value,
    pub readings: Vec<StoredReading>,
    pub dtcs: Vec<StoredDtc>,
}

#[derive(Debug, Serialize)]
pub struct SessionSummary {
    pub id: i64,
    pub vehicle: String,
    pub kind: String,
    pub at: String,
    pub odometer_mi: Option<f64>,
    pub note: Option<String>,
    pub dtc_count: i64,
}

#[derive(Debug, Serialize)]
pub struct Session {
    pub id: i64,
    pub vehicle: String,
    pub kind: String,
    pub at: String,
    pub odometer_mi: Option<f64>,
    pub note: Option<String>,
    pub report: Value,
}

#[derive(Debug, Serialize)]
pub struct Point {
    pub session_id: i64,
    pub at: String,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Serialize)]
pub struct DtcHistory {
    pub code: String,
    pub kinds: String,
    pub first_seen: String,
    pub last_seen: String,
    pub sessions: i64,
    pub present_in_latest: bool,
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS sessions (
  id INTEGER PRIMARY KEY, vehicle TEXT NOT NULL, kind TEXT NOT NULL, at TEXT NOT NULL,
  odometer_mi REAL, note TEXT, report TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS readings (
  session_id INTEGER NOT NULL REFERENCES sessions(id), source TEXT NOT NULL, key TEXT NOT NULL,
  value REAL NOT NULL, unit TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS dtcs (
  session_id INTEGER NOT NULL REFERENCES sessions(id), code TEXT NOT NULL, kind TEXT NOT NULL, ecu TEXT NOT NULL);
CREATE INDEX IF NOT EXISTS readings_key ON readings(key, session_id);
";

impl Store {
    pub fn open(path: &Path) -> Result<Store> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        Self::init(Connection::open(path)?)
    }

    pub fn open_in_memory() -> Result<Store> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(db: Connection) -> Result<Store> {
        db.execute_batch(SCHEMA)?;
        Ok(Store { db })
    }

    pub fn save_session(&self, s: &NewSession) -> Result<i64> {
        let tx = self.db.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO sessions (vehicle, kind, at, odometer_mi, note, report)
             VALUES (?1, ?2, COALESCE(?3, strftime('%Y-%m-%dT%H:%M:%SZ','now')), ?4, ?5, ?6)",
            params![s.vehicle, s.kind, s.at, s.odometer_mi, s.note, s.report.to_string()],
        )?;
        let id = tx.last_insert_rowid();
        for r in &s.readings {
            tx.execute("INSERT INTO readings VALUES (?1, ?2, ?3, ?4, ?5)", params![id, r.source, r.key, r.value, r.unit])?;
        }
        for d in &s.dtcs {
            tx.execute("INSERT INTO dtcs VALUES (?1, ?2, ?3, ?4)", params![id, d.code, d.kind, d.ecu])?;
        }
        tx.commit()?;
        Ok(id)
    }

    pub fn session(&self, id: i64) -> Result<Option<Session>> {
        Ok(self
            .db
            .query_row("SELECT id, vehicle, kind, at, odometer_mi, note, report FROM sessions WHERE id = ?1", [id], |r| {
                Ok(Session {
                    id: r.get(0)?,
                    vehicle: r.get(1)?,
                    kind: r.get(2)?,
                    at: r.get(3)?,
                    odometer_mi: r.get(4)?,
                    note: r.get(5)?,
                    report: serde_json::from_str(&r.get::<_, String>(6)?).unwrap_or(Value::Null),
                })
            })
            .optional()?)
    }

    pub fn sessions(&self, vehicle: Option<&str>, limit: i64) -> Result<Vec<SessionSummary>> {
        let mut q = self.db.prepare(
            "SELECT s.id, s.vehicle, s.kind, s.at, s.odometer_mi, s.note,
                    (SELECT COUNT(*) FROM dtcs d WHERE d.session_id = s.id)
             FROM sessions s WHERE ?1 IS NULL OR s.vehicle = ?1 ORDER BY s.at DESC, s.id DESC LIMIT ?2",
        )?;
        let rows = q.query_map(params![vehicle, limit], |r| {
            Ok(SessionSummary { id: r.get(0)?, vehicle: r.get(1)?, kind: r.get(2)?, at: r.get(3)?, odometer_mi: r.get(4)?, note: r.get(5)?, dtc_count: r.get(6)? })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn trend(&self, vehicle: &str, key: &str) -> Result<Vec<Point>> {
        let mut q = self.db.prepare(
            "SELECT s.id, s.at, r.value, r.unit FROM readings r JOIN sessions s ON s.id = r.session_id
             WHERE s.vehicle = ?1 AND r.key = ?2 AND r.source != 'freeze' ORDER BY s.at, s.id",
        )?;
        let rows = q.query_map(params![vehicle, key], |r| Ok(Point { session_id: r.get(0)?, at: r.get(1)?, value: r.get(2)?, unit: r.get(3)? }))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn keys(&self, vehicle: &str) -> Result<Vec<String>> {
        let mut q = self.db.prepare(
            "SELECT DISTINCT r.key FROM readings r JOIN sessions s ON s.id = r.session_id WHERE s.vehicle = ?1 ORDER BY r.key",
        )?;
        let rows = q.query_map([vehicle], |r| r.get(0))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn dtc_history(&self, vehicle: &str) -> Result<Vec<DtcHistory>> {
        let mut q = self.db.prepare(
            "WITH latest AS (SELECT id FROM sessions WHERE vehicle = ?1 ORDER BY at DESC, id DESC LIMIT 1)
             SELECT d.code, GROUP_CONCAT(DISTINCT d.kind), MIN(s.at), MAX(s.at), COUNT(DISTINCT s.id),
                    MAX(s.id IN (SELECT id FROM latest))
             FROM dtcs d JOIN sessions s ON s.id = d.session_id WHERE s.vehicle = ?1
             GROUP BY d.code ORDER BY MAX(s.at) DESC, d.code",
        )?;
        let rows = q.query_map([vehicle], |r| {
            Ok(DtcHistory { code: r.get(0)?, kinds: r.get(1)?, first_seen: r.get(2)?, last_seen: r.get(3)?, sessions: r.get(4)?, present_in_latest: r.get(5)? })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn store() -> Store {
        Store::open_in_memory().unwrap()
    }

    fn reading(key: &str, value: f64) -> StoredReading {
        StoredReading { source: "live".into(), key: key.into(), value, unit: "V".into() }
    }

    fn dtc(code: &str, kind: &str) -> StoredDtc {
        StoredDtc { code: code.into(), kind: kind.into(), ecu: "7E8".into() }
    }

    fn save(s: &Store, vehicle: &str, at: &str, readings: Vec<StoredReading>, dtcs: Vec<StoredDtc>) -> i64 {
        s.save_session(&NewSession {
            vehicle: vehicle.into(),
            kind: "scan".into(),
            at: Some(at.into()),
            odometer_mi: None,
            note: None,
            report: json!({"ok": true}),
            readings,
            dtcs,
        })
        .unwrap()
    }

    #[test]
    fn session_round_trips_report() {
        let s = store();
        let id = save(&s, "VIN1", "2026-01-01T00:00:00Z", vec![], vec![]);
        let got = s.session(id).unwrap().unwrap();
        assert_eq!(got.vehicle, "VIN1");
        assert_eq!(got.report, json!({"ok": true}));
        assert!(s.session(id + 1).unwrap().is_none());
    }

    #[test]
    fn sessions_list_newest_first_filtered_by_vehicle() {
        let s = store();
        save(&s, "A", "2026-01-01T00:00:00Z", vec![], vec![dtc("P0420", "stored")]);
        save(&s, "B", "2026-01-02T00:00:00Z", vec![], vec![]);
        save(&s, "A", "2026-01-03T00:00:00Z", vec![], vec![]);
        let list = s.sessions(Some("A"), 10).unwrap();
        assert_eq!(list.iter().map(|x| x.at.as_str()).collect::<Vec<_>>(), vec!["2026-01-03T00:00:00Z", "2026-01-01T00:00:00Z"]);
        assert_eq!(list[1].dtc_count, 1);
        assert_eq!(s.sessions(None, 2).unwrap().len(), 2);
    }

    #[test]
    fn trend_returns_series_oldest_first() {
        let s = store();
        save(&s, "A", "2026-01-02T00:00:00Z", vec![reading("module_voltage", 14.1)], vec![]);
        save(&s, "A", "2026-01-01T00:00:00Z", vec![reading("module_voltage", 13.9), reading("rpm", 700.0)], vec![]);
        save(&s, "B", "2026-01-03T00:00:00Z", vec![reading("module_voltage", 12.0)], vec![]);
        let t = s.trend("A", "module_voltage").unwrap();
        assert_eq!(t.iter().map(|p| p.value).collect::<Vec<_>>(), vec![13.9, 14.1]);
        assert_eq!(s.keys("A").unwrap(), vec!["module_voltage", "rpm"]);
    }

    #[test]
    fn trend_excludes_freeze_frame_rows() {
        let s = store();
        let freeze = StoredReading { source: "freeze".into(), key: "rpm".into(), value: 3000.0, unit: "rpm".into() };
        save(&s, "A", "2026-01-01T00:00:00Z", vec![reading("rpm", 700.0), freeze], vec![]);
        assert_eq!(s.trend("A", "rpm").unwrap().iter().map(|p| p.value).collect::<Vec<_>>(), vec![700.0]);
    }

    #[test]
    fn cleared_code_is_not_present_after_clear_session() {
        let s = store();
        save(&s, "A", "2026-01-01T00:00:00Z", vec![], vec![dtc("P0420", "stored")]);
        s.save_session(&NewSession { vehicle: "A".into(), kind: "clear".into(), at: Some("2026-01-02T00:00:00Z".into()), odometer_mi: None, note: None, report: json!({}), readings: vec![], dtcs: vec![] }).unwrap();
        assert!(!s.dtc_history("A").unwrap()[0].present_in_latest);
    }

    #[test]
    fn dtc_history_tracks_first_last_seen_and_current() {
        let s = store();
        save(&s, "A", "2026-01-01T00:00:00Z", vec![], vec![dtc("P0420", "stored"), dtc("P0303", "pending")]);
        save(&s, "A", "2026-01-05T00:00:00Z", vec![], vec![dtc("P0420", "stored")]);
        let h = s.dtc_history("A").unwrap();
        let p0420 = h.iter().find(|d| d.code == "P0420").unwrap();
        assert_eq!((p0420.first_seen.as_str(), p0420.last_seen.as_str(), p0420.sessions, p0420.present_in_latest), ("2026-01-01T00:00:00Z", "2026-01-05T00:00:00Z", 2, true));
        assert!(!h.iter().find(|d| d.code == "P0303").unwrap().present_in_latest);
    }

    #[test]
    fn default_timestamp_is_utc_iso() {
        let s = store();
        let id = s.save_session(&NewSession { vehicle: "A".into(), kind: "clear".into(), at: None, odometer_mi: Some(120000.0), note: Some("n".into()), report: json!({}), readings: vec![], dtcs: vec![] }).unwrap();
        let got = s.session(id).unwrap().unwrap();
        assert!(got.at.ends_with('Z') && got.at.contains('T'), "{}", got.at);
        assert_eq!((got.kind.as_str(), got.odometer_mi, got.note.as_deref()), ("clear", Some(120000.0), Some("n")));
    }

    #[test]
    fn opens_file_database_and_creates_parent_dir() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sub/carlink.db");
        save(&Store::open(&path).unwrap(), "A", "2026-01-01T00:00:00Z", vec![], vec![]);
        assert_eq!(Store::open(&path).unwrap().sessions(None, 5).unwrap().len(), 1);
    }
}
