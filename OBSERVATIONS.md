# HW6 Observations

## Approach

- **SP1 `sort_by_distance`** — `sort_by` with `partial_cmp().unwrap()`, since `f64` is only `PartialOrd`. Taking `pairs` by value and returning it avoids a second allocation.
- **SP2 `k_nearest_labels` / `majority_vote`** — `iter().take(k)` handles `k = 0` and `k > len` for free without a bounds check. The vote is a `HashMap<i32, usize>` tally followed by `max_by_key`.
- **SP3 `predict_one`** — pure composition of the previous pieces: `distance_label_pairs` -> `sort_by_distance` -> `k_nearest_labels` -> `majority_vote`.
- **SP4 `predict_all` / `accuracy`** — `map` over the test set; accuracy zips predictions with truth, counts matches, and returns `0.0` on empty input to avoid a divide by zero.
- **SP5 `confusion_matrix_binary`** — one `fold` over the zipped pairs with a `match` on `(pred, truth)`. The catch-all arm drops non-binary labels, which is what makes the "ignore other labels" rule fall out of the pattern match instead of an `if` chain.

## Challenges and surprises

The `f64` ordering was the first snag: `sort` does not exist for floats, so `partial_cmp` and an explicit `unwrap` are required. That `unwrap` is a real panic path if a distance is ever `NaN`, which cannot happen here because the inputs are finite, but it is worth knowing it is there.

The bigger surprise was how little code k-NN actually is. There is no training step at all - the entire model is the training set, and all the work happens at prediction time. That makes it O(n) distance computations per query, so it scales badly with the dataset even though it is trivial to implement.

Ties in `majority_vote` are also undefined: with an even `k` and a 2-2 split, `max_by_key` returns whichever key `HashMap` iteration reaches first, which is not deterministic across runs. An odd `k` sidesteps this.

## Confusion matrix reflection

FP and FN are not interchangeable, and which one hurts depends entirely on the domain.

High **FP** means the classifier cries wolf - it flags negatives as positive. In spam filtering that is a real email lost to the junk folder; in fraud detection it is a legitimate transaction declined and an annoyed customer. The cost is wasted attention and eroded trust in the system.

High **FN** means the classifier misses real positives. In medical screening that is a missed diagnosis; in fraud detection it is money actually gone. The cost is the thing the classifier existed to catch.

Accuracy alone hides all of this: on a dataset that is 99% negative, a model that always predicts negative scores 99% while catching nothing. That is why the confusion matrix, and the precision/recall pair derived from it, matter more than a single number.
