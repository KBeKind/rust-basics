


    // create a function that finds the average of several numers and returns it
   
    println!("The average is {}", average);
    
fn find_average(numbers: [i32; 5]) -> f32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum as f32 / numbers.len() as f32
}

fn main() {

    let numbers = [1, 2, 3, 4, 5];
    let average = find_average(numbers);
    println!("The average is {}", average);


    let item: &str = "mango";


    


    let a: i32 = 10;
    let b: i32 = 15;
    println!("Hello, world! , {} {}", a, b);


    let character: char = 'a';
    println!("Character => {}", character);

    let boolean: bool = true;
    println!("Boolean => {}", boolean);

    let tuple: (i32, i32, f64, i32, bool) = (1, -2, 3.0, 4, true);
    println!("Tuple => {:?}", tuple);


    // STRUCT IS LIKE A CLASS
    struct Point { 

        x: f64, 
        
        y: f64, 
        
        }

    // ENUM GIVES A TYPE
    enum Color {
        Red,
        Green,
        Blue,
    }

    // LOOP ENDS ONLY ON BREAK
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    // WHILE LOOP IS STANDARD
    let mut i: i32 = 0;
    while i < 10 {
        i += 1;
    }

    // FOR LOOP IS STANDARD
    for i: i32 in 0..10 {
        println!("Loop stuff")
    }

    // MATCH IS SIMILAR TO SWITCH
    let number = 3;

match number { 

1 => println!("One"), 

2 => println!("Two"), 

3 => println!("Three"), 

4 => println!("Four"), 

5 => println!("Five"), 

_ => println!("It's something else"),

}

//MATCH WITH RANGE PATTERN
let number = 5;

match number { 

1..=5 => println!("one through five"),

 _ => println!("something else"),

}


// FUNCTIONS ARE NORMAL
fn function_name(parameter_list) -> return_type {

    // code
    
    }

fn print_number(n: i32) {

    println!("The number is {}", n);
        
    }
        
// Call the function
        
print_number(5);


fn subtract(x: i32, y: i32) -> i32 {

    return x - y;
    
    }
    
// Call the function
    
let difference = subtract(10, 5);
    
println!("The difference is {}", difference);


// IMPLEMENTATION impl GIVES METHODS TO A STRUCT OR ENUM

struct StructName {

    // fields
    
}
    
impl StructName {
    
    // methods
    
}


struct Rectangle { width: u32, height: u32,

}

impl Rectangle { fn area(&self) -> u32 { self.width * self.height }

}


// IMPLEMENTS CAN BE USED FOR TRAITS
//TRAITS DFINE BEHAVIOR SHARE ACROSS MULTIPLE TYPES

struct Rectangle { width: u32, height: u32,

}

impl Rectangle { fn area(&self) -> u32 { self.width * self.height }

}

}
