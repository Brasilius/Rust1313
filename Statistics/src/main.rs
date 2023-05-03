
fn sum(array1: &[u32], array2: &[u32]) -> Vec<u32> {
    array1.iter().zip(array2.iter()).map(|(a, b)| a + b).collect()
}

fn difference(array1: &[u32], array2: &[u32]) -> Vec<u32> {
    array1.iter().zip(array2.iter()).map(|(a, b)| a - b).collect()
}

fn product(array1: &[u32], array2: &[u32]) -> Vec<u32> {
    array1.iter().zip(array2.iter()).map(|(a, b)| a * b).collect()
}

fn quotient(array1: &[u32], array2: &[u32]) -> Vec<u32> {
    array1.iter().zip(array2.iter()).filter_map(|(a, b)| {
        if *b != 0 {
            Some(a / b)
        } else {
            None
        }
    }).collect()
}
fn main() {
    println!("Hello! I will be doing numerous calculations using the Rust programming language! ");
    println!("To start please enter the length of the first array: ");
    let mut length = String::new();
    std::io::stdin().read_line(&mut length).expect("Failed to read line");
    let length: u32 = length.trim().parse().expect("Please type a number!");
    println!("Please enter the length of the second array: ");
    let mut length2 = String::new();
    std::io::stdin().read_line(&mut length2).expect("Failed to read line");
    let length2: u32 = length2.trim().parse().expect("Please type a number!");
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();
    // set the length of the arrays
    array1.resize(length as usize, 0);
    array2.resize(length2 as usize, 0);
    //please enter the length of the first array
    println!("Please enter the values of the first array: ");
    for i in 0..length {
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).expect("Failed to read line");
        let value: u32 = value.trim().parse().expect("Please type a number!");
        array1[i as usize] = value;
    }
    //please enter the length of the second array
    println!("Please enter the values of the second array: ");
    for i in 0..length2 {
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).expect("Failed to read line");
        let value: u32 = value.trim().parse().expect("Please type a number!");
        array2[i as usize] = value;
    }
    //print the arrays
    println!("The first array is: {:?}", array1);
    println!("The second array is: {:?}", array2);
    //print the sum of the arrays
    println!("The sum of the arrays is: {:?}", sum(&array1, &array2));
    //print the difference of the arrays
    println!("The difference of the arrays is: {:?}", difference(&array1, &array2));
    //print the product of the arrays
    println!("The product of the arrays is: {:?}", product(&array1, &array2));
    //print the quotient of the arrays
    println!("The quotient of the arrays is: {:?}", quotient(&array1, &array2));
    //end the program
    println!("Thank you for using my program!");
}
