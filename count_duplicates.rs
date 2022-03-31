fn main() {
	//printer_error("aaabbbbhaijjjm"); // "0/14"
	// count_duplicates_v1("aaabbbbhaijjjm"); // "0/14"
	// count_duplicates_v2("aaabbbbhaijjjm"); // "0/14"
	count_duplicates_v1("aaaxbbbbyyhwawiwjjjwwm"); // 8/22
	count_duplicates_v2("aaaxbbbbyyhwawiwjjjwwm"); // 8/22
}

// FILTER
fn count_duplicates_v2(s: &str) -> String {
	format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
}

// ALGORYTHM
fn count_duplicates_v1(s: &str) -> String {
	let n = "nopqrstuvwxyz";
    let mut wrong_char = 0;

    if s.chars().any(|c| matches!(c, 'n'..='z')) == false { 
		return format!("{}/{}", wrong_char, s.len()).to_owned()
    }
	else {
// calculates an amount of wrong characters
		for i in 0..s.len() {
			for ii in 0..n.len() {
				if s[i..=i].find( &n[ii..=ii] ) == Some(0) {
					wrong_char += 1
				} else {}
			}
		}
		return format!("{}/{}", wrong_char, s.len()).to_owned()
	}
	let concat = format!("{}/{}", wrong_char, s.len());
	println!("concat: {}", concat);
	return "".to_owned()
}
