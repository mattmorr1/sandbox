use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub features: Vec<f64>,
    pub label: i32,
}

impl DataPoint {
    pub fn new(features: Vec<f64>, label: i32) -> Self {
        DataPoint { features, label }
    }
}

// These two helpers are provided - students do NOT implement them.
pub fn euclidean_distance(a: &DataPoint, b: &DataPoint) -> f64 {
    a.features
        .iter()
        .zip(b.features.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn distance_label_pairs(query: &DataPoint, data: &[DataPoint]) -> Vec<(f64, i32)> {
    data.iter()
        .map(|p| (euclidean_distance(query, p), p.label))
        .collect()
}

// --- Student-implemented functions below ---

pub fn sort_by_distance(mut pairs: Vec<(f64, i32)>) -> Vec<(f64, i32)> {
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    pairs
}

pub fn k_nearest_labels(pairs: &[(f64, i32)], k: usize) -> Vec<i32> {
    pairs.iter().take(k).map(|&(_, label)| label).collect()
}

pub fn majority_vote(labels: &[i32]) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &label in labels {
        *counts.entry(label).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(label, _)| label)
        .unwrap_or(-1)
}

pub fn predict_one(query: &DataPoint, data: &[DataPoint], k: usize) -> i32 {
    let sorted = sort_by_distance(distance_label_pairs(query, data));
    majority_vote(&k_nearest_labels(&sorted, k))
}

pub fn predict_all(test: &[DataPoint], train: &[DataPoint], k: usize) -> Vec<i32> {
    test.iter().map(|q| predict_one(q, train, k)).collect()
}

pub fn accuracy(predictions: &[i32], truth: &[i32]) -> f64 {
    if predictions.is_empty() {
        return 0.0;
    }
    let correct = predictions
        .iter()
        .zip(truth)
        .filter(|(p, t)| p == t)
        .count();
    correct as f64 / predictions.len() as f64
}

pub fn confusion_matrix_binary(predictions: &[i32], truth: &[i32]) -> (usize, usize, usize, usize) {
    let (mut tp, mut tn, mut fp, mut fn_count) = (0, 0, 0, 0);
    for (&p, &t) in predictions.iter().zip(truth) {
        match (p, t) {
            (1, 1) => tp += 1,
            (0, 0) => tn += 1,
            (1, 0) => fp += 1,
            (0, 1) => fn_count += 1,
            _ => {}
        }
    }
    (tp, tn, fp, fn_count)
}

#[cfg(test)]
mod tests;
