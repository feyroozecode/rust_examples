use std::io;

pub fn calc_app() {

    let mut program_loop = true;
    let mut input = String::new();

    // boucle du programme START 
    if program_loop == true {
        println!("Enter an arithemetic operator (+, -, *, /) : ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        // convert this to a string
        let user_choice :&str = input.trim().parse().expect("Invalid input for integer");
    
        input.clear();
    
        // a generic functions that get user input and handle operation  based on choice
        if(!user_choice.is_empty()) {
            println!("You need to choice one operator");
        } else {
            println!("You need to choice one operator");
        }

        
    }

    // get arithemetic operator 
  

    

}

//fn startCalc(operaton , type)