extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);//Rng tritt - defined method
	
	println!("The secret number is: {}",secret_number);
    
	loop{
	    println!("Please input your guess.");

		let mut guess = String::new();  //let mut = let mutable 즉 가변 변수 선언 
	    //::는 연관함수임을 나타냄 new함수가 string인스턴스값을 받을수있는 공간을 만들어줌 
		io::stdin().read_line(&mut guess)//stdin함수는 instance값을 return해줌 
			.expect("Failed to read line");//io::result instance의 expect 메소드
		
	    let guess:u32 = match guess.trim().parse(){//guess String->u32/trim erase each side/parse change string to u32
		//	.expect("please type a number");
            Ok(num) => num,
		    Err(_) => continue,
		};
	    println!("You guessed: {}",guess);		
 
		match guess.cmp(&secret_number){ //cmp method return Ordering enum/ guess is String & sec_num is i32
		//i32-32bit integer, u32-32bit +integer/ default-i32
		    Ordering::Less     => println!("Too small!"),//match check the instance with each arms
		    Ordering::Greater  => println!("Too big!"),
		    Ordering::Equal    =>{	
				println!("You win!");
				break;
			}
		}
	}

}
