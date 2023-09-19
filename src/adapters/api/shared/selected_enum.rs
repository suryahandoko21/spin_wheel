fn filter_by_key<T>(vec: Vec<(String, T)>, key: &str) -> Vec<(String, T)>
where
    T: Clone,
{
    vec.into_iter()
        .filter(|(k, _)| k == key)
        .collect()
}

pub fn select_enum_reward(enum_string :String)->String{
    let enum_reward = vec![
        ("zonk".to_string(), "NONE".to_string()),
        ("product".to_string(), "PRODUCT".to_string()),
        ("cash".to_string(), "MONEY".to_string()),
        ];

   let filtered_vector = filter_by_key(enum_reward, &enum_string.to_lowercase());
   let category=&filtered_vector.get(0).unwrap().1;
   return category.to_string();

}