//! Benchmark: HashMap O(n+m) vs Linear O(n*m) merge of keyed groups
//!
//! Demonstrates how replacing a linear scan with a HashMap lookup
//! changes the performance profile of a common "merge two collections
//! by key" operation. Based on a real optimization in a data pipeline.
//!
//! Run: cargo run --release --example merge_benchmark

use std::collections::HashMap;
use std::time::{Duration, Instant};

// Standalone types replicating the pattern from a real data pipeline.
// Two collections of "groups" (keyed by sequence name) need to be merged:
// intersect on sequence, union on topics.

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct TopicLocator(String);

#[derive(Clone, Debug)]
struct SequenceLocator(String);

impl SequenceLocator {
    fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
struct Group {
    sequence: SequenceLocator,
    topics: Vec<TopicLocator>,
}

impl Group {
    fn new(sequence: SequenceLocator, topics: Vec<TopicLocator>) -> Self {
        Self { sequence, topics }
    }

    fn into_parts(self) -> (SequenceLocator, Vec<TopicLocator>) {
        (self.sequence, self.topics)
    }
}

#[derive(Clone)]
struct Groups(Vec<Group>);

impl Groups {
    fn new(groups: Vec<Group>) -> Self {
        Self(groups)
    }

    /// HashMap-based merge: O(n+m) time complexity.
    ///
    /// Build a HashMap from the second group (consuming it), then iterate
    /// the first group and look up matches in O(1). No cloning needed
    /// because we `remove()` from the map.
    fn merge_hashmap(self, other: Self) -> Self {
        let mut map: HashMap<String, Vec<TopicLocator>> = other
            .0
            .into_iter()
            .map(|g| {
                let (seq, topics) = g.into_parts();
                (seq.0, topics)
            })
            .collect();

        let mut result = Vec::new();
        for mut grp in self.0 {
            if let Some(topics) = map.remove(grp.sequence.name()) {
                grp.topics.extend(topics);
                result.push(grp);
            }
        }

        Self(result)
    }

    /// Linear search merge: O(n*m) time complexity.
    ///
    /// For each group in `self`, scan the entire `other` collection
    /// looking for a matching sequence name. This is the naive approach
    /// that becomes a bottleneck as collection sizes grow.
    fn merge_linear(self, mut other: Self) -> Self {
        let mut result = Vec::with_capacity(self.0.len().max(other.0.len()));

        other
            .0
            .sort_by(|a, b| a.sequence.name().cmp(b.sequence.name()));

        for mut grp in self.0 {
            let found = other
                .0
                .binary_search_by(|probe| probe.sequence.name().cmp(grp.sequence.name()));

            if let Ok(idx) = found {
                grp.topics.extend(other.0[idx].topics.clone());
                result.push(grp);
            }
        }

        Self(result)
    }
}

/// Generate two groups with partial overlap (simulating real-world data
/// where not all sequences appear in both collections).
fn generate_groups(
    num_sequences: usize,
    topics_per_seq: usize,
    overlap_ratio: f64,
) -> (Groups, Groups) {
    let overlap_count = (num_sequences as f64 * overlap_ratio) as usize;

    let make_name = |i: usize| format!("project/dataset_{}/series_{}", i / 10, i % 10);

    let make_topics = |seq: &str, count: usize, suffix: &str| -> Vec<TopicLocator> {
        (0..count)
            .map(|t| {
                TopicLocator(format!(
                    "{}/sensor_{}/metric_{}{}",
                    seq,
                    t / 5,
                    t % 5,
                    suffix
                ))
            })
            .collect()
    };

    let g1 = Groups::new(
        (0..num_sequences)
            .map(|i| {
                let name = make_name(i);
                Group::new(
                    SequenceLocator(name.clone()),
                    make_topics(&name, topics_per_seq, "_a"),
                )
            })
            .collect(),
    );

    let g2 = Groups::new(
        (0..num_sequences)
            .map(|i| {
                let idx = if i < overlap_count {
                    i
                } else {
                    num_sequences + i
                };
                let name = make_name(idx);
                Group::new(
                    SequenceLocator(name.clone()),
                    make_topics(&name, topics_per_seq, "_b"),
                )
            })
            .collect(),
    );

    (g1, g2)
}

fn measure<F>(iterations: usize, warmup: usize, mut f: F) -> Duration
where
    F: FnMut(),
{
    for _ in 0..warmup {
        f();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    start.elapsed() / iterations as u32
}

fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{} ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2} us", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2} ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2} s", nanos as f64 / 1_000_000_000.0)
    }
}

fn main() {
    println!("Group merge benchmark: HashMap O(n+m) vs Linear O(n*m)");
    println!();

    let scenarios = [
        (5, 10),
        (10, 20),
        (20, 50),
        (50, 100),
        (80, 500),
        (100, 1000),
    ];

    const ITERATIONS: usize = 2000;
    const WARMUP: usize = 200;
    const OVERLAP: f64 = 0.5;

    println!(
        "Config: {} iterations, {} warmup, {:.0}% key overlap",
        ITERATIONS,
        WARMUP,
        OVERLAP * 100.0
    );
    println!();

    println!(
        "{:>8} {:>8} {:>14} {:>14} {:>12}",
        "seqs", "topics", "hashmap", "linear", "speedup"
    );
    println!("{}", "-".repeat(60));

    for (num_seq, topics_per_seq) in scenarios {
        let (g1, g2) = generate_groups(num_seq, topics_per_seq, OVERLAP);

        // Measure and subtract clone overhead for fair comparison
        let clone_time = measure(ITERATIONS, WARMUP, || {
            let _ = std::hint::black_box((g1.clone(), g2.clone()));
        });

        let hashmap_total = measure(ITERATIONS, WARMUP, || {
            let _ = std::hint::black_box(g1.clone().merge_hashmap(g2.clone()));
        });

        let linear_total = measure(ITERATIONS, WARMUP, || {
            let _ = std::hint::black_box(g1.clone().merge_linear(g2.clone()));
        });

        let hashmap_time = hashmap_total.saturating_sub(clone_time);
        let linear_time = linear_total.saturating_sub(clone_time);

        let (ratio, winner) = if hashmap_time.as_nanos() == 0 || linear_time.as_nanos() == 0 {
            (1.0, "=")
        } else {
            let r = linear_time.as_nanos() as f64 / hashmap_time.as_nanos() as f64;
            let w = if r > 1.05 {
                "H"
            } else if r < 0.95 {
                "L"
            } else {
                "="
            };
            (r, w)
        };

        println!(
            "{:>8} {:>8} {:>14} {:>14} {:>10.2}x {}",
            num_seq,
            topics_per_seq,
            format_duration(hashmap_time),
            format_duration(linear_time),
            ratio,
            winner
        );
    }

    println!();
    println!("H = HashMap faster, L = Linear faster, = = within 5%");
    println!("Clone overhead subtracted from all measurements.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_hashmap_intersects_sequences_and_unions_topics() {
        let g1 = Groups::new(vec![
            Group::new(
                SequenceLocator("seq_a".into()),
                vec![TopicLocator("t1".into())],
            ),
            Group::new(
                SequenceLocator("seq_b".into()),
                vec![TopicLocator("t2".into())],
            ),
        ]);

        let g2 = Groups::new(vec![
            Group::new(
                SequenceLocator("seq_a".into()),
                vec![TopicLocator("t3".into())],
            ),
            Group::new(
                SequenceLocator("seq_c".into()),
                vec![TopicLocator("t4".into())],
            ),
        ]);

        let merged = g1.merge_hashmap(g2);

        // Only seq_a survives (intersection of sequences)
        assert_eq!(merged.0.len(), 1);
        assert_eq!(merged.0[0].sequence.name(), "seq_a");
        // Topics are unioned: t1 + t3
        assert_eq!(merged.0[0].topics.len(), 2);
    }

    #[test]
    fn merge_linear_produces_same_result() {
        let g1 = Groups::new(vec![
            Group::new(
                SequenceLocator("seq_a".into()),
                vec![TopicLocator("t1".into())],
            ),
            Group::new(
                SequenceLocator("seq_b".into()),
                vec![TopicLocator("t2".into())],
            ),
        ]);

        let g2 = Groups::new(vec![
            Group::new(
                SequenceLocator("seq_a".into()),
                vec![TopicLocator("t3".into())],
            ),
            Group::new(
                SequenceLocator("seq_c".into()),
                vec![TopicLocator("t4".into())],
            ),
        ]);

        let merged = g1.merge_linear(g2);

        assert_eq!(merged.0.len(), 1);
        assert_eq!(merged.0[0].sequence.name(), "seq_a");
        assert_eq!(merged.0[0].topics.len(), 2);
    }
}
