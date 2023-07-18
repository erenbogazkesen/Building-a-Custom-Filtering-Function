struct FilterCondition {
    desired_value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.desired_value
    }
}

fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_items = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            filtered_items.push(*item);
        }
    }

    filtered_items
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_condition = FilterCondition { desired_value: 7 };

    let filtered_result = custom_filter(&collection, &filter_condition);

    println!("Filtrelenmiş Sonuç: {:?}", filtered_result);
}
