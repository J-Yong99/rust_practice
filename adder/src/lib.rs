#[cfg(test)]


fn add_two(a:i32)->i32{
	a+2
}

mod tests {
    
	use super::*;
    #[test]
	fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn another(){
		assert_eq!(5,add_two(2));
		//	panic!("make this test failure");
	}

}
