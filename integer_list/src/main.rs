//#[derive(Debug)]
use std::io;
use std::convert::TryInto;

const list_size:u32 = 10;

fn main() {
	let mut vect=Vec::new();
	let mut a=0;//for while
	//1. make input as vector list for 10 numbers
	while a!=list_size{
		let mut tmp = String::new();
		match io::stdin().read_line(&mut tmp){
			Ok(T)=>{
			let tmp:u32 = tmp.trim().parse().expect("fail to parse.");
			vect.push(tmp);
			}	   
			Err(E)=> println!("{}",E)	
		}
		a+=1;
	}
	// success to make vector list
	//2. sorting numbers
//	switch(0,1,&mut vect);
	sort(&mut vect);
	println!("{:?}",vect);
}
fn switch(n:u32,m:u32,list:&mut Vec<u32> ){
	let n: usize = n.try_into().unwrap();
	let m: usize = m.try_into().unwrap();
	let mut a=list[n];
	let mut b=list[m];
	list[n]=b;
	list[m]=a;
}
fn sort(list:&mut Vec<u32>){//sort by algorithm which find max num in list 

	let mut j=list_size;
	while j!=1 {
	let mut tmp =0;
	let mut i=0;
		while i!=j{
			if list[i as usize]>list[tmp as usize]{
				tmp=i;
			}	
//			if i==j-1{break;};// we have to break it before i become 5!!
			i+=1;
		}
		switch(tmp,i,list);
		println!("{:?}",list);// to show the process
		j-=1;
	}
}
