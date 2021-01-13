pub fn prints_return(a:i32)->i32{
	println!("I got the value{}",a);
	a
}


#[cfg(test)]
mod tests {
	use crate::*;

    #[test]
    fn it_works() {
		let value=prints_return(10);
        assert_eq!(10, value);
    }
	#[test]
    fn it_fails() {
		let value=prints_return(10);
        assert_eq!(10, value);
    }
	
}
