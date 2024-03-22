

pub mod zz{

// x:i32,y:i16

	pub fn echo(strr: &str){
		println!("{0}", strr);
	}


pub fn get_full_name(first: & str,  last: &str) -> String{
	let full_name = format!("{0} {1}", first, last);
	return full_name;
}







}


pub mod pair_test{


		//Tuples
	  static pair : (&str, i32) = ("Waqar", -31245);
	 //let pair : (i32, &str) = (433, "Char soo bees");

	 fn print_pair() ->(){
	//println!("first item: {}, second item: {}, 3rd item: {}",pair.0, pair.1, pair.2);
	}
}