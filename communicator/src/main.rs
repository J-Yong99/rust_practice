use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;


/*
fn main(){
	let f=File::open("hello.txt");

	let f=match f{
		Ok(file)=>file,
		Err(error)=>match error.kind(){
			ErrorKind::NotFound=>match File::create("hello.txt"){
				Ok(fcc)=>fcc,
				Err(er)=>panic!("problem creating the file: {:?}",er)
			}
			other_error=>{
				panic!("problem opening the file:{:?}",other_error)
			}
		}
	};
}
*/

fn main(){
	let f=File::open("hello.txt");

}
