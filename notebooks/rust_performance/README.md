# Rust Performance: HashMap vs Linear Merge

## The Problem

You have two collections of groups, keyed by name. You need to **intersect** on the key and **union** the values — a common pattern in data pipelines when merging metadata from different sources.

The naive approach: sort + binary search, giving O(n log n + m log m). The alternative: build a HashMap from one side (O(m)), then look up each item in O(1). Total: O(n+m).

## The Tradeoff

The HashMap approach has better asymptotic complexity, but hashing has a higher constant factor than comparison. At small collection sizes (< ~50 items), the difference is negligible or even negative. The speedup only materializes at scale.

This example is adapted from [mosaico-labs/mosaico#52](https://github.com/mosaico-labs/mosaico/pull/52), where this optimization was proposed for `SequenceTopicGroups::merge`. The PR was closed — correctly — because the actual workload involved tens of sequences, not hundreds. The sort + binary search was already fast enough, and the HashMap added complexity without practical gain.

**The lesson:** asymptotic complexity matters, but so does your actual N. Profile before you optimize.

## Run

```bash
cargo run --release --example merge_benchmark
```

### Run tests

```bash
cargo test --example merge_benchmark
```

## Expected Output

```
Group merge benchmark: HashMap O(n+m) vs Linear O(n log n)

Config: 2000 iterations, 200 warmup, 50% key overlap

    seqs   topics        hashmap         linear      speedup
------------------------------------------------------------
       5       10         ...             ...          ~1.0x
      50      100         ...             ...          ~2-3x
     100     1000         ...             ...          ~4-8x
```

Exact numbers depend on hardware. The crossover point where HashMap wins is typically around 30-50 items.

## Key Takeaway

When merging two keyed collections and you're reaching for `.binary_search()` inside a loop, a `HashMap` *might* be faster — but only if your collections are large enough for O(1) lookup to beat the hashing overhead. Benchmark your actual workload, not a synthetic one.
