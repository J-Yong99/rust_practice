fn main() {
    let mut v1=vec![1,2,3];
    let mut v1_iter=v1.iter();
    println!("{:?}",v1_iter);
    v1_iter.next(); 
    println!("{:?}",v1_iter);
    let total:i32 = v1_iter.sum();
    println!("{:?}",total);

    println!("{:?}",v1_iter);
            
//    println!("mmaa127371{:?}",v1_iter);
        
   
}
