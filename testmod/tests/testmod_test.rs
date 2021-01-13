extern crate testmod;

#[test]
fn a(){
	assert_eq!(testmod::prints_return(1),2);
}
