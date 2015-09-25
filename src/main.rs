extern crate time;

use std::io;
use std::rand;

fn main() {
    println!("Please input 'x'.");
    let start = time::get_time();

	let mut x = 0u;
    loop {

	    x += 1;
	    if x == 11 { 

			let now = time::get_time();
			println!("time:   {}", (now - start) / 60);
	    	println!("Complete!!");
	    	break;
	    }
    	println!("No.{}", x);

    	let secret_number = (rand::random::<uint>() % 1000u) + 1u;
	    loop {

		    println!("{} + x = 1000", secret_number);

		    let input = io::stdin().read_line()
		    					  .ok().expect("Failed to read line");
			
			let input_num: Option<uint> = from_str(input.as_slice().trim());

		    let num = match input_num {
		        Some(num) => num,
		        None      => {
		            println!("Please input a number!");
		            continue;
		        }
		    };

		    println!("{} + {} = {}", secret_number , num, (secret_number + num));


		    match cmp(secret_number + num, 1000) {
		        Less    => println!("Too small!"),
		        Greater => println!("Too big!"),
		        Equal   => {println!("Success!"); break;},
		    }
	    }
	}


}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}
