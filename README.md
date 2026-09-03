# HW6 — k-NN Classifier

**DS 210 · Spring 2026 · Due: April 25, 2026**

In this assignment you will complete a full k-Nearest Neighbors classifier — one of the most intuitive machine learning algorithms. This is the culmination of the HW4 → HW5 → HW6 arc.

---

## Learning Objectives

- Use `HashMap` for label counting
- Use `sort_by` with closures for custom ordering
- Chain iterator operations to implement a complete ML pipeline
- Understand and compute classifier evaluation metrics (accuracy, confusion matrix)

---

## Provided Code

The following are given in `src/q1/mod.rs` — you do **not** implement these:

```rust
pub fn euclidean_distance(a: &DataPoint, b: &DataPoint) -> f64 { ... }
pub fn distance_label_pairs(query: &DataPoint, data: &[DataPoint]) -> Vec<(f64, i32)> { ... }
```

Use these helpers to build your classifier functions.

---

## Subproblems

Complete the functions in `src/q1/mod.rs`. Each subproblem corresponds to a feature branch.

| Branch | Functions |
|--------|-----------|
| `sp1-sort` | `sort_by_distance` |
| `sp2-vote` | `k_nearest_labels`, `majority_vote` |
| `sp3-predict-one` | `predict_one` |
| `sp4-predict-all` | `predict_all`, `accuracy` |
| `sp5-confusion` | `confusion_matrix_binary` |

### SP1 — Sort by Distance

```rust
pub fn sort_by_distance(mut pairs: Vec<(f64, i32)>) -> Vec<(f64, i32)>
```

Sort the `(distance, label)` pairs in ascending order by distance. Return the sorted `Vec`.

> Hint: use `.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap())`

### SP2 — Voting

```rust
pub fn k_nearest_labels(pairs: &[(f64, i32)], k: usize) -> Vec<i32>
pub fn majority_vote(labels: &[i32]) -> i32
```

- `k_nearest_labels`: take the first `k` pairs and collect their labels
- `majority_vote`: return the label with the highest count (use `HashMap<i32, usize>`)

### SP3 — Predict One

```rust
pub fn predict_one(query: &DataPoint, data: &[DataPoint], k: usize) -> i32
```

Chain: `distance_label_pairs` → `sort_by_distance` → `k_nearest_labels` → `majority_vote`

### SP4 — Predict All + Accuracy

```rust
pub fn predict_all(test: &[DataPoint], train: &[DataPoint], k: usize) -> Vec<i32>
pub fn accuracy(predictions: &[i32], truth: &[i32]) -> f64
```

- `predict_all`: apply `predict_one` to every test point
- `accuracy`: fraction of predictions that match ground truth (return `0.0` for empty input)

### SP5 — Confusion Matrix

```rust
pub fn confusion_matrix_binary(predictions: &[i32], truth: &[i32]) -> (usize, usize, usize, usize)
```

Return `(TP, TN, FP, FN)` for binary classification (labels `0` and `1`). Ignore pairs where neither label is 0 or 1.

---

## Git Requirements

- **≥ 10 commits** (excluding instructor/bot commits)
- **≥ 6 branches** (main + one per subproblem)

---

## `main.rs`

Update `src/main.rs` to demonstrate the full k-NN pipeline. Use a training set of ≥7 points with binary labels, define ≥3 test points, and:

1. Show the step-by-step pipeline for one query
2. Run `predict_all` and print results
3. Print accuracy and the confusion matrix

This is manually graded (~2 points).

---

## `OBSERVATIONS.md`

Create a file named `OBSERVATIONS.md` in the root of your repo. It should include:

1. A description of your solution approach for each subproblem
2. What you found challenging or surprising about implementing k-NN
3. Reflect on the confusion matrix: what does high FP vs. high FN mean in a real classification task?

This is manually graded (~3 points).

---

## Grading Summary

| Component | Points |
|-----------|--------|
| Unit tests (7 functions × 1 pt each) | 7 |
| Code quality (rustfmt + clippy) | 2 |
| Git hygiene (commits + branches) | 2 |
| `main.rs` output (manual) | 2 |
| `OBSERVATIONS.md` (manual) | 3 |
| **Total** | **16** |
