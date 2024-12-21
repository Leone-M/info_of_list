use std::collections::HashMap;

pub fn mean(vect: &Vec<i32>) -> f32 {
	let mut mean: f32 = 0.0;
	for el in vect {
		mean += *el as f32;
	}
	mean = mean / vect.len() as f32;
	return mean;
}

pub fn median(vect: &Vec<i32>) -> i32 {
	let mut srtd_vec: Vec<i32> = vect.clone();
	srtd_vec.sort();
	let mode: i32 = srtd_vec[(srtd_vec.len()/2) as usize];
	return mode;
}

pub fn mode(vect: &Vec<i32>) -> i32 {
	let mut freq: HashMap<i32, i32> = HashMap::new();
	for el in vect {
		freq.insert(*el, 0);
	}
	for el in vect {
		freq.insert(*el, *freq.get(&el).unwrap()+1);
	}
	let mut max_fr: i32 = 0;
	let mut max_el: i32 = 0;
	let keys: Vec<i32> = freq.clone().into_keys().collect();
	for key in keys {
		if freq.get(&key).unwrap() >= &max_fr{
			max_el = key;
			max_fr = *freq.get(&key).unwrap();
		}
	}
	return max_el;
}