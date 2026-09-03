mod q1;
use q1::{
    DataPoint, accuracy, confusion_matrix_binary, distance_label_pairs, k_nearest_labels,
    majority_vote, predict_all, sort_by_distance,
};

fn main() {
    // Small 2D training set with binary labels.
    let train = vec![
        DataPoint::new(vec![1.0, 2.0], 0),
        DataPoint::new(vec![1.5, 1.8], 0),
        DataPoint::new(vec![2.0, 2.5], 0),
        DataPoint::new(vec![7.0, 8.0], 1),
        DataPoint::new(vec![8.0, 7.5], 1),
        DataPoint::new(vec![7.5, 9.0], 1),
        DataPoint::new(vec![6.5, 7.0], 1),
    ];

    let test = vec![
        DataPoint::new(vec![1.8, 2.0], -1),
        DataPoint::new(vec![7.2, 8.1], -1),
        DataPoint::new(vec![4.0, 5.0], -1),
    ];

    println!("=== k-NN Classifier (k=3) ===\n");

    // Walk through the pipeline for the first query.
    let q = &test[0];
    println!("--- Pipeline walkthrough for query {:?} ---", q.features);
    let pairs = distance_label_pairs(q, &train);
    println!("distance_label_pairs: {:?}", pairs);
    let sorted = sort_by_distance(pairs);
    println!("sort_by_distance:     {:?}", sorted);
    let labels = k_nearest_labels(&sorted, 3);
    println!("k_nearest_labels(3):  {:?}", labels);
    let vote = majority_vote(&labels);
    println!("majority_vote:        {vote}");
    println!();

    let k = 3;
    let preds = predict_all(&test, &train, k);
    println!("predict_all (k={k}):");
    for (i, (point, pred)) in test.iter().zip(preds.iter()).enumerate() {
        println!(
            "  test[{i}] features={:?} -> predicted label={pred}",
            point.features
        );
    }
    println!();

    // Demo-only reference labels for these sample test points.
    let truth = vec![0, 1, 0];
    let acc = accuracy(&preds, &truth);
    println!("Accuracy: {:.2}", acc);

    let (tp, tn, fp, fn_count) = confusion_matrix_binary(&preds, &truth);
    println!("Confusion matrix (binary):");
    println!("  TP={tp}  FP={fp}");
    println!("  FN={fn_count}  TN={tn}");
}
