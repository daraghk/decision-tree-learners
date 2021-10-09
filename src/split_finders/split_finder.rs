use crate::question::Question;
use crate::dataset::DataSet;

mod split_finder_gini;
mod split_finder_variance;

#[derive(Debug)]
struct BestThresholdResult {
    loss: f32,
    threshold_value: f32,
}

#[derive(Debug)]
pub struct BestSplitResult {
    pub gain: f32,
    pub question: Question,
}

#[derive(Clone, Copy)]
pub enum SplitMetric{
    Gini,
    Variance
}

pub struct SplitFinder{
    split_metric: SplitMetric,
    pub find_best_split: fn(&DataSet<i32, i32>, u32) -> BestSplitResult
}

impl SplitFinder{
    pub fn new(metric: SplitMetric) -> Self{
        Self{
            split_metric: metric,
            find_best_split: match metric{
                SplitMetric::Gini => use_gini::find_best_split,
                SplitMetric::Variance => use_variance::find_best_split
            }
        }
    }
}

mod use_gini{
    use super::*;
    pub fn find_best_split(data: &DataSet<i32, i32>, number_of_classes: u32)-> super::BestSplitResult {
        split_finder_gini::find_best_split(data, number_of_classes)
    }
}

mod use_variance{
    use super::*;
    pub fn find_best_split(data: &DataSet<i32, i32>, number_of_classes: u32)-> super::BestSplitResult {
        split_finder_variance::find_best_split(data, number_of_classes)
    }
}

fn get_sorted_feature_tuple_vector(features: &Vec<Vec<i32>>, column: u32) -> Vec<(i32, i32)> {
    let mut feature_tuple_vector = vec![];
    let mut row_index = 0;
    features.iter().for_each(|row| {
        let feature_value = row[column as usize];
        feature_tuple_vector.push((feature_value, row_index));
        row_index += 1;
    });
    feature_tuple_vector.sort_by_key(|tuple| tuple.0);
    feature_tuple_vector
}

#[cfg(test)]
mod tests {
    use crate::{dataset::DataSet, split_finder::get_sorted_feature_tuple_vector};
    #[test]
    fn test_get_sorted_feature_tuple_vector() {
        let features = vec![vec![10, 2, 1], vec![6, 2, 2], vec![1, 2, 3]];
        let labels = vec![1, 2, 3];
        let data = DataSet { features, labels };
        let column = 0;
        let sorted_feature_tuple_vector = get_sorted_feature_tuple_vector(&data.features, column);
        println!("{:?}", sorted_feature_tuple_vector);
        assert_eq!(sorted_feature_tuple_vector, vec![(1, 2), (6, 1), (10, 0)])
    }
}
