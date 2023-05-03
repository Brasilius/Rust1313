use std::io;

struct PrimaryBody{
    mass: u64,
    gravity_parameter: u64,
    radius: u64,
}
fn gravity_mass_relation(mass:u64, gravity_parameter:u64 ) -> u64{
    let relation;
    relation = mass/gravity_parameter;
    return relation
}


fn main() {
    println!("Hello, I will be calculating the Hohmann Transfer of your values!");
    //lets begin with creating an instance for the primary body and doing some simple math with it!
    println!("Lets start with the relation between a mass in earths and the gravity parameter of said mass!");
    println!("please input the mass in earths: ");
    let mut mass_input = String::new();
    io::stdin()
        .read_line(&mut mass_input)
        .expect("failed to read input L ");
    let mass_number: u64 = mass_input.trim().parse().expect("failed to parse input L ");
    println!("please input the gravitational parameter the system: ");
    let mut gravity_input = String::new();
    io::stdin()
        .read_line(&mut gravity_input)
        .expect("failed to read input");
    let gravity_number:u64 = gravity_input.trim().parse().expect("failed to parse line l");



    let body1 = PrimaryBody{
        mass: mass_number,
        gravity_parameter: gravity_number,
        radius: 1,
    };
    let relation = gravity_mass_relation(body1.mass, body1.gravity_parameter);
    println!("The relation is {} ", relation);
    return

}
