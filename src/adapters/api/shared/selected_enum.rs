pub fn select_enum_reward(enum_string :String)->String{
    let enum_reward =  vec!["zonk".to_string(),"product".to_string(),"cash".to_string()];
    let reward:Vec<_> = enum_reward.iter()
    .filter(|&s| s.contains(&enum_string.to_lowercase()))
    .cloned()
    .collect();
    if reward.get(0).is_some(){
        let choosed = reward.get(0).unwrap();
        return choosed.to_string();
    }
   return "".to_string();
}