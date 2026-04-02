# 🛝 Data Sandbox & Shared Playground

A collaborative repository gathering real-world data challenges, tricky migrations (like SAS to Python), and debugging exercises. This is a WIP and any help is welcome as we try to make it a useful resource for data people.

Whether you are losing rows in a Pandas merge, fighting with `NaN` values, or migrating legacy code, you'll find templates and interactive fixes here.

## 🚀 Interactive Colab Playgrounds

You don't need to install anything locally. Click the buttons below to open the interactive templates directly in Google Colab!

| Challenge / Exercise | Description | Open in Colab |
| :--- | :--- | :--- |
| **SAS to Python: Missing Rows** | How to track down dropped rows and debug `NaN` handling using Set Differences and `dataprof`. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/sas_migrations/01_finding_missing_rows.ipynb) |
| **Profiling Messy CSVs** | Run profiling on diverse data types using `dataprof`. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/data_quality/01_profiling_messy_csvs_with_dataprof.ipynb) |
| **Stop Putting DBs in Columns** | Example of why nesting complex structures in pandas columns is an anti-pattern. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/pandas_antipatterns/01_stop_putting_dbs_in_columns.ipynb) |
| **Data Leakage (Startup Case)** | Exploring a complex data leakage issue similar to one experienced by a major startup. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/data_leakage/01_startup_data_leakage.ipynb) |
| **Leakage at Scale (Polars + DataFusion)** | Advanced leakage detection using Polars LazyFrames, Arrow C Interface, and DataFusion SQL. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/data_leakage/02_leakage_detection_at_scale.ipynb) |
| **Salting Skewed GroupBy (PySpark)** | Fixing aggregation bottlenecks in Databricks/Spark using two-phase salted aggregation. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/spark_performance/01_salting_skewed_groupby.ipynb) |
| **Sensor Drift & Stuck-At Faults** | Detecting silent IoT sensor failures with rolling z-scores and variance checks. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/iot_anomalies/01_sensor_drift_and_stuck_faults.ipynb) |
| **Merge Pitfalls** | Silent row explosion from duplicate keys, lost rows from wrong join type, and dtype mismatches. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/pandas_antipatterns/02_merge_pitfalls.ipynb) |
| **DateTime Hell** | Timezone-naive vs aware mixing, ambiguous date formats, and off-by-one resampling bugs. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/pandas_antipatterns/03_datetime_hell.ipynb) |
| **PII Anonymization Leakage** | Why hashing emails isn't enough when quasi-identifiers (ZIP, DOB, gender) allow re-identification. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/data_privacy/01_pii_anonymization_leakage.ipynb) |
| **Spark Memory Spill & Window OOM** | Unbounded window functions and forced SortMergeJoin vs proper partitioning and broadcast. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/spark_performance/02_memory_spill_window_vs_broadcast.ipynb) |
| **SCD2 Idempotent Upsert** | Non-idempotent SCD2 scripts that duplicate rows on re-run vs correct upsert with DuckDB. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/data_modeling/01_scd2_idempotent_upsert.ipynb) |
| **Late Arriving Events & Windowing** | Processing-time vs event-time windowing and the watermark trade-off for streaming pipelines. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/streaming_patterns/01_late_arriving_events_windowing.ipynb) |
| **Schema Evolution & Breaking Changes** | Detecting breaking schema changes with PyArrow contracts before downstream pipelines fail. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/data_contracts/01_schema_evolution_breaking_changes.ipynb) |
| **DAG Coupling & Retry Safety** | Tightly coupled tasks and non-idempotent retries vs atomic checkpointed pipelines. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/master/notebooks/orchestration_antipatterns/01_dag_coupling_retry_safety.ipynb) |

### Rust

| Example | Description | Run |
| :--- | :--- | :--- |
| **HashMap vs Linear Merge** | When does O(n+m) actually beat O(n log n)? Benchmark of a real optimization that was correctly rejected. | `cargo run --release --example merge_benchmark` ([README](notebooks/rust_performance/README.md)) |

## 💻 Local Installation (For advanced users)

If you prefer to run this locally, the repository is managed via standard Python tooling.

```bash
git clone https://github.com/AndreaBozzo/prac.git
cd prac
pip install . 

# or use uv
uv sync
```