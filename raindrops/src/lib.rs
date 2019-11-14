pub fn raindrops(n: u32) -> String {


	let mut owned_string: String = "".to_owned();
    let plong: String = "Plong".to_owned();
	let plang: String = "Plang".to_owned();
	let pling: String = "Pling".to_owned();

	if n % 3 == 0 {
		owned_string.push_str(&pling);
	}
	if n % 5 == 0 {
		owned_string.push_str(&plang);
	}
	if n % 7 == 0 {
		owned_string.push_str(&plong);
	}
	if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
		return n.to_string()
	}

	owned_string
}
