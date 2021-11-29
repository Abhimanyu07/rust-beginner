


fn update_the_vector(data: Vec){
    for num in data.iter_mut(){
        *num = *num + 20; 
    }
    printTheVector(&data);
}

fn print_the_vector(data : Vec){
    for num in data.iter(){
        println!("{}", num);
    }
}

fn print_the_number(num: i64){
    println!("{}", num);
}



fn main(){
    let mut data = vec![1,2,3,4,5,6,7];
//    updateTheVector(data);
    update_the_vector(data);
    // print_the_number(12);

}