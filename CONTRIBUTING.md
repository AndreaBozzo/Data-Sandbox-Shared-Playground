# Contributing

Thanks for your interest in contributing! This is a community project and any help is welcome.

## How to Contribute

### Report bugs or suggest topics
Open a [GitHub Issue](https://github.com/AndreaBozzo/prac/issues) describing the problem or the topic you'd like to see covered.

### Add a new notebook
1. Fork the repo and create a branch.
2. Place your notebook under `notebooks/<topic>/` using the naming convention `NN_descriptive_name.ipynb` (e.g. `02_temporal_leakage.ipynb`).
3. Open a pull request with a short description of what the notebook teaches.

### Improve an existing notebook
Bug fixes, clearer explanations, and better examples are all welcome. Open a PR describing what you changed and why.

## Notebook Conventions

- **Self-contained** -- generate synthetic data inside the notebook so it runs without external files.
- **Colab-ready** -- the first code cell should install any dependencies with `!pip install ... --quiet`.
- **English throughout** -- variable names, comments, markdown, and print statements.
- **Progressive flow** -- problem statement, then solution, with markdown explanations between code cells.
- **Numbering** -- use `01_`, `02_`, etc. within each topic folder.

## Development Setup

```bash
git clone https://github.com/AndreaBozzo/prac.git
cd prac
pip install .
```

Or with [uv](https://github.com/astral-sh/uv):

```bash
uv sync
```

## Pull Request Process

- Keep PRs focused on one notebook or one fix.
- Make sure the notebook runs top-to-bottom without errors (test in Colab if possible).
- Update the README table if you're adding a new notebook.
