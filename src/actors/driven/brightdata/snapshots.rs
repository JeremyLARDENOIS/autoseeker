use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub created: String,
    pub status: String,
    pub dataset_id: String,
    pub dataset_size: u64,
}

pub struct SnapshotsResponse {
    pub snapshots: Vec<Snapshot>,
}

impl Display for SnapshotsResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.snapshots.is_empty() {
            writeln!(f, "No snapshots found.")?;
            return Ok(());
        }

        let headers = ["ID", "Created", "Status", "Dataset ID", "Size"];
        let mut widths = headers.map(|h| h.len());
        for s in &self.snapshots {
            widths[0] = widths[0].max(s.id.len());
            widths[1] = widths[1].max(s.created.len());
            widths[2] = widths[2].max(s.status.len());
            widths[3] = widths[3].max(s.dataset_id.len());
            widths[4] = widths[4].max(format!("{}", s.dataset_size).len());
        }

        let sep = |c: char| -> String {
            let mut out = String::from("+");
            for w in widths {
                out.push_str(&std::iter::repeat_n(c, w + 2).collect::<String>());
                out.push('+');
            }
            out
        };
        let pad = |s: &str, w: usize| -> String { format!(" {:<width$} ", s, width = w) };

        writeln!(f, "{}", sep('-'))?;
        write!(f, "|")?;
        for (i, h) in headers.iter().enumerate() {
            write!(f, "{}|", pad(h, widths[i]))?;
        }
        writeln!(f)?;
        writeln!(f, "{}", sep('='))?;

        for s in &self.snapshots {
            write!(f, "|")?;
            write!(f, "{}|", pad(&s.id, widths[0]))?;
            write!(f, "{}|", pad(&s.created, widths[1]))?;
            write!(f, "{}|", pad(&s.status, widths[2]))?;
            write!(f, "{}|", pad(&s.dataset_id, widths[3]))?;
            write!(f, "{}|", pad(&format!("{}", s.dataset_size), widths[4]))?;
            writeln!(f)?;
        }
        writeln!(f, "{}", sep('-'))?;
        Ok(())
    }
}
