# examples to debug sas to python conversion issues

#1
# Assuming he has a unique ID for each contract (e.g., 'contract_id')

import pandas as pd
import dataprof as dp

# Load your dataframes (replace with actual file paths or data loading methods)
mother_db = pd.read_csv("mother_db.csv")  # Load your main dataframe
broken_subset_db = pd.read_csv("broken_subset_db.csv")  # Load the subset that's missing rows

# 1. Get the IDs that SHOULD be in the perimeter (from the mother DB)
expected_ids = set(mother_db[mother_db['condition']]['contract_id'])

# 2. Get the IDs that actually MADE IT into his new sub-DB
actual_ids = set(broken_subset_db['contract_id'])

# 3. Find the missing ones!
missing_ids = expected_ids - actual_ids

# 4. Print the exact rows that vanished to see what they have in common
missing_rows = mother_db[mother_db['contract_id'].isin(missing_ids)]
print(missing_rows)

#2
# dataprof is a tool to profile your data and find out what's going on with it.
# You can use it to compare the mother DB and the broken subset to see if there are any glaring issues 
# (like missing values, unexpected data types, etc.) that could explain why rows are disappearing.


mother_db = pd.read_csv("mother_db.csv")  # Load your main dataframe
broken_subset_db = pd.read_csv("broken_subset_db.csv")  # Load the subset that's missing rows

# 1. Profile the Mother DB
report_mother = dp.profile(mother_db) # Pass your main dataframe here
print("--- MOTHER DB PROFILE ---")
print(f"Total rows: {report_mother.rows}")
print(f"Nulls in 'Sample' column: {report_mother.column_profiles['YOUR_SAMPLE_COLUMN'].null_count}")

# 2. Profile one of the broken subsets (e.g. where 40 rows are missing)
report_subset = dp.profile(broken_subset_db)
print("\n--- SUBSET DB PROFILE ---")
print(f"Total rows: {report_subset.rows}")

# 3. Export the reports to JSON so you can compare them easily
report_mother.save("profile_mother.json")
report_subset.save("profile_subset.json")