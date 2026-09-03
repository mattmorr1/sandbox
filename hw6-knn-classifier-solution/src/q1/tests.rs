use super::*;

#[test]
fn sort_by_distance_test_cases() {
    let pairs = vec![(3.0, 1), (1.0, 0), (2.0, 1), (0.5, 0)];
    let sorted = sort_by_distance(pairs);
    assert_eq!(sorted[0].0, 0.5);
    assert_eq!(sorted[1].0, 1.0);
    assert_eq!(sorted[2].0, 2.0);
    assert_eq!(sorted[3].0, 3.0);

    // Labels travel with distances
    assert_eq!(sorted[0].1, 0);
    assert_eq!(sorted[1].1, 0);

    // Already sorted
    let already = vec![(1.0, 0), (2.0, 1), (3.0, 0)];
    let result = sort_by_distance(already);
    assert_eq!(result[0].0, 1.0);
    assert_eq!(result[2].0, 3.0);

    // Empty
    let empty: Vec<(f64, i32)> = vec![];
    assert_eq!(sort_by_distance(empty), vec![]);
}

#[test]
fn k_nearest_labels_test_cases() {
    let pairs = vec![(0.5, 0), (1.0, 1), (2.0, 0), (3.0, 1)];
    assert_eq!(k_nearest_labels(&pairs, 1), vec![0]);
    assert_eq!(k_nearest_labels(&pairs, 3), vec![0, 1, 0]);
    assert_eq!(k_nearest_labels(&pairs, 4), vec![0, 1, 0, 1]);

    // k larger than length
    assert_eq!(k_nearest_labels(&pairs, 10), vec![0, 1, 0, 1]);

    // k = 0
    let empty: Vec<i32> = vec![];
    assert_eq!(k_nearest_labels(&pairs, 0), empty);
}

#[test]
fn majority_vote_test_cases() {
    assert_eq!(majority_vote(&[0, 0, 1]), 0);
    assert_eq!(majority_vote(&[1, 1, 0]), 1);
    assert_eq!(majority_vote(&[1]), 1);
    assert_eq!(majority_vote(&[0, 0, 0, 1, 1]), 0);
    assert_eq!(majority_vote(&[1, 1, 1, 1, 0]), 1);
}

#[test]
fn predict_one_test_cases() {
    let train = vec![
        DataPoint::new(vec![1.0, 1.0], 0),
        DataPoint::new(vec![1.5, 1.5], 0),
        DataPoint::new(vec![8.0, 8.0], 1),
        DataPoint::new(vec![8.5, 8.5], 1),
        DataPoint::new(vec![9.0, 9.0], 1),
    ];

    // Query near label-0 cluster
    let q0 = DataPoint::new(vec![1.2, 1.2], -1);
    assert_eq!(predict_one(&q0, &train, 3), 0);

    // Query near label-1 cluster
    let q1 = DataPoint::new(vec![8.2, 8.2], -1);
    assert_eq!(predict_one(&q1, &train, 3), 1);
}

#[test]
fn predict_all_test_cases() {
    let train = vec![
        DataPoint::new(vec![0.0, 0.0], 0),
        DataPoint::new(vec![0.5, 0.5], 0),
        DataPoint::new(vec![9.0, 9.0], 1),
        DataPoint::new(vec![9.5, 9.5], 1),
    ];
    let test = vec![
        DataPoint::new(vec![0.2, 0.2], -1),
        DataPoint::new(vec![9.2, 9.2], -1),
    ];
    let preds = predict_all(&test, &train, 2);
    assert_eq!(preds.len(), 2);
    assert_eq!(preds[0], 0);
    assert_eq!(preds[1], 1);

    // Empty test set
    let empty: Vec<DataPoint> = vec![];
    assert_eq!(predict_all(&empty, &train, 3), vec![]);
}

#[test]
fn accuracy_test_cases() {
    let preds = vec![0, 1, 0, 1, 1];
    let truth = vec![0, 1, 0, 0, 1];
    let acc = accuracy(&preds, &truth);
    assert!((acc - 0.8).abs() < 1e-9);

    // Perfect accuracy
    let acc2 = accuracy(&[1, 0, 1], &[1, 0, 1]);
    assert!((acc2 - 1.0).abs() < 1e-9);

    // Zero accuracy
    let acc3 = accuracy(&[0, 0, 0], &[1, 1, 1]);
    assert!((acc3 - 0.0).abs() < 1e-9);

    // Empty
    let acc4 = accuracy(&[], &[]);
    assert!((acc4 - 0.0).abs() < 1e-9);
}

#[test]
fn confusion_matrix_binary_test_cases() {
    let preds = vec![1, 0, 1, 1, 0, 0];
    let truth = vec![1, 0, 0, 1, 1, 0];
    // TP=2, TN=2, FP=1, FN=1
    let (tp, tn, fp, fn_count) = confusion_matrix_binary(&preds, &truth);
    assert_eq!(tp, 2);
    assert_eq!(tn, 2);
    assert_eq!(fp, 1);
    assert_eq!(fn_count, 1);

    // All correct
    let (tp2, tn2, fp2, fn2) = confusion_matrix_binary(&[1, 0, 1], &[1, 0, 1]);
    assert_eq!(tp2, 2);
    assert_eq!(tn2, 1);
    assert_eq!(fp2, 0);
    assert_eq!(fn2, 0);

    // All wrong
    let (tp3, tn3, fp3, fn3) = confusion_matrix_binary(&[1, 1, 0, 0], &[0, 0, 1, 1]);
    assert_eq!(tp3, 0);
    assert_eq!(tn3, 0);
    assert_eq!(fp3, 2);
    assert_eq!(fn3, 2);
}
