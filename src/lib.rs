pub mod strops {
#![allow(dead_code)]
use std::convert::TryInto;
pub fn strin(s: &String, n: u32) -> char {
	let mut x = 0;
	let mut ret = '\0';
	for c in s.chars() {
		if x == n {
			ret = c;
			break;
		}
		x+=1;
	}
	ret
}

pub fn strcmp(s1: &String,s2: &String) -> i8{
	// Return 0:  Strings match
	// Return 1:  Unmatched char found
	// Return -1: Lengths different
	let t1 = s1;
	let t2 = s2;
	let mut ret = 0;

	let l = t1.len();
	if l != t2.len() {
		ret = -1;
	}
	if ret == 0 {
		let mut x: u32 = 0;
		while x < l.try_into().unwrap() {
			if strin(t1,x) != strin(t2,x) {
				ret = 1;
				break;
			}
			x += 1;
		}
	}
	ret
}

pub fn strrev(s: &String) -> String{
	let mut r = String::new();

	for c in s.chars() {
		r = String::from(format!("{}",c))+&r
	}

	r
}

pub fn strsplit(s : &String, w:char) -> Vec<String> {
	let mut ret = vec![];
	let mut tmp = String::new();
	for c in s.chars() {
		if c == w {
			ret.push(tmp);
			tmp = String::new();
			continue;
		}
		tmp = tmp + &format!("{}", c)
	}
	if tmp != String::from("") {
		ret.push(tmp);
	}
	ret
}
}