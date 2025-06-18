use std::io;

fn standard_io(){
    let a = [1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index.trim().parse().expect("Index entered was not a number");

    if index < 5{
        let element = a[index];
        println!("The value of the element at index {index} is: {element}");
    }else {
        println!("The entered index is greater than 5, please reduce it.")
    }
}

fn basic_loop(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("Result {result}")
}

fn main (){
    // standard_io();
    basic_loop();
}