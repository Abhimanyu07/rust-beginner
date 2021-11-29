// getting started with the RUST


// a function to print hello world and 
// other messages using iterables

fn greet_world(){
    println!("hello world");
    let greet1 = "hello";
    let greet2 = "hi";
    let greet3 = "heyy";
    let greets = [greet1, greet2,greet3];
    for greet in greets.iter(){
        println!("{}",&greet);

    }

}

// the main function where everything begins 

fn main(){
    greet_world();
}