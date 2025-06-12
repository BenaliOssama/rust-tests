use std::io ;

fn main(){

    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

    let mut trials : u32 = 0 ; 

    let answer  = "The letter e\n";

    loop {
        println!("{}", riddle);
        let mut guess  = String::new();

        io::stdin().read_line(&mut guess).expect("error reading the line!");

        trials += 1 ; 


        if guess == answer {
            println!("Number of trials: {}", trials );
                break;
        }
    }
}
