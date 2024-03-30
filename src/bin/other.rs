fn main(){

    println!("i am other file!");

    let float_val: f64 = 5.5f64;
    println!("flaot val is {}", float_val);

    {
        const AGE:i32= 37;

        let  float_val = 3.4f32;
        println!("in the block! {}", float_val);

        println!("age const is  {}", AGE);

    }
}