fn another_function(x : i32){
    println!("{}", x);

}
fn five()->i32{
    5
}

fn main() {
    
    another_function(32);
    let x = 34;
    // let y = 3;
    {
        let x = 45;
        println!("{}", x);
    }
    println!("{}", x);
    println!("{}",{let x = 3; x+1});
    println!("{}",five());


}
