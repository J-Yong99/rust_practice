//#[derive(Debug)]
/*
struct Rectangle{
    length:u32,
    width:u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.length*self.width
	}

}

impl Rectangle{
    fn Build_r(len:u32,wid:u32)->Rectangle{
	    Rectangle{length:len,width:wid}
	}

}

fn main(){
  let a = Rectangle::Build_r(3,5);
  println!("a is {}",a.length);


}
*/
/*
fn main() {
    let mut n = String::new();
	io::stdin().read_line(&mut n).expect("read line error");
    let n :u32 = n.trim().parse().expect("parse error ");
	
    let res = fibo_r(n);
	println!("fibo is {}",res);


}
*/
/*
fn fibo_i(n :u32){
    let mut a:u32 =1; 
    let mut b:u32 =1; 
	let mut c:u32 =a+b;
    let mut tmp:u32 = 0;
	if n==1||n==2 {
		println!("fibo is 1");
	}else{
		while tmp!=n-2{
			c=a+b;
			a=b;
			b=c;
			tmp=tmp+1;
		}
      println!("fibo is {}",c);
	}    
}
fn fibo_r(n :u32)->u32{
    if n==1||n==2{
		1
	}else{
        fibo_r(n-1)+fibo_r(n-2)
	}
}
*/
/*
fn main() {
	let mut a = Vec::new();
	a.push('a');	
//	a.to_string
	let mut a = vec![100,99,98,97];
	for i in &mut a{
		*i+=10;
		println!{"{}",i};
	}
	println!("{}",a[1]);
}
*/
/*
fn main() {
	let mut a=10;
	let b=add(&mut a);
	println!("{}",a);
//	println!("{}",b);

	
}

fn add(a:&mut i32)->& i32{
	*a+=10;
	a
}
*/
/*
fn main(){
	let v="hell".to_string();
	let u="o wo".to_string();
	let k="rld".to_string();
	let j = format!("{}{}{}",v,u,k);
	println!("{}{}{}",v,u,k);
	println!("{}",j);

	
	
}
*/

/*
//#![allow(unused)]

pub struct Guess {
	value: String,
}
impl Guess {
//	pub fn new(value: u32) -> Guess {
//		if value < 1 || value > 100 {
//			panic!("Guess value must be between 1 and 100, got {}.", value);
//		}
//		Guess {value}
//	}
	 fn value(self) ->String {
		self.value
	}
}


fn main() {
	
	let mut a=Guess{value:"jhhh".to_string()};
	println!("{}",a.value());
//	println!("{:?}",a);


}


*/
/*
fn main(){
	let mut a=String::from("10");
	{a.push('1');}
	println!("{}",a);
	
}

*/
//#[test]
fn main(){
    let mut v1=vec![1,2,3];
    let v1_iter=v1.into_iter();
//    println!("{:?}",v1_iter);
    for mut i in v1_iter{
        i+=1;
    }
//    assert_eq!(v1_iter,[2,3,4]);
//    println!("{:?}",v1_iter);
}
















