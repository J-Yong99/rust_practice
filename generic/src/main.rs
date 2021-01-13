use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
	    type_name::<T>()
}
/*
extern crate generic;
use generic::Tweet;
use generic::Summarizable;

fn main(){
	let tweet = Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("of course, as you probably already know, people"),
		reply: false,
		retweet: false,
	};

	println!("1 new tweet: {}", tweet.summary());
}
*/
// life time

fn main(){
	let string1=String::from("adcd");
	let result;

	{let string2=String::from("xyz");
	result = longest(string1.as_str(),string2.as_str());
	println!("the longest string is {}",result);
	}
//	println!("{}",string2);
	println!("the longest string is {}",result);
}

fn longest<'a> (x: &'a str, y:&'a str)-> &'a str{
	if x.len()>y.len(){
	println!("{}", type_of(x));

		x
	}else{
		println!("{:?}", y);
		y
	}

}

