# 🛝 Data Sandbox & Shared Playground

A collaborative repository gathering real-world data engineering challenges, tricky migrations (like SAS to Python), and debugging exercises. This is a WIP and any help is welcome as we try to make it a useful resource for data people.

Whether you are losing rows in a Pandas merge, fighting with `NaN` values, or migrating legacy code, you'll find templates and interactive fixes here.

## 🚀 Interactive Colab Playgrounds

You don't need to install anything locally. Click the buttons below to open the interactive templates directly in Google Colab!

| Challenge / Exercise | Description | Open in Colab |
| :--- | :--- | :--- |
| **SAS to Python: Missing Rows** | How to track down dropped rows and debug `NaN` handling using Set Differences and `dataprof`. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/main/notebooks/sas_migrations/01_finding_missing_rows.ipynb) |
| **Profiling Messy CSVs** | Run profiling on diverse data types using `dataprof`. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/main/notebooks/data_quality/01_profiling_messy_csvs_with_dataprof.ipynb) |
| **Stop Putting DBs in Columns** | Example of why nesting complex structures in pandas columns is an anti-pattern. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/main/notebooks/pandas_antipatterns/01_stop_putting_dbs_in_columns.ipynb) |
| **Data Leakage (Startup Case)** | Exploring a complex data leakage issue similar to one experienced by a major startup. | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AndreaBozzo/prac/blob/main/notebooks/data_leakage/01_startup_data_leakage.ipynb) |

## 💻 Local Installation (For advanced users)

If you prefer to run this locally, the repository is managed via standard Python tooling.

```bash
git clone https://github.com/AndreaBozzo/prac.git
cd prac
pip install . 
```