use std::io::Write;

fn main() {
    check(Some((4,5,8)));
    check(Some((1,2,3)));
    check(None);
}

fn check(nums : Option<(i32, i32, i32)>) -> bool {

    let mut from_parameters = false;
    let mut is_triple = false;
    while !from_parameters{

        let (side_a, side_b, side_c) = match nums{
            Some(num) => {
                from_parameters = true;
                num
            }
            None => prompt()
        };

        // a^2 + b^2 = c^2
        if side_a.pow(2) + side_b.pow(2) == side_c.pow(2){
            println!("it is right");
            is_triple = true;
        } else {
            println!("it is not right");
        }
        
    }
    is_triple

}

fn prompt() -> (i32, i32, i32) {
    let mut side_a = String::new();
    let mut side_b = String::new();
    let mut side_c = String::new();

    print!("Side A: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut side_a).unwrap();

    let side_a: i32 = match side_a.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("not valid input");
            std::process::exit(1);
        }
    };

    print!("Side B: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut side_b).unwrap();

    let side_b: i32 = match side_b.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("not valid input");
            std::process::exit(1);
        }
    };

    print!("Side C: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut side_c).unwrap();

    let side_c: i32 = match side_c.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("not valid input");
            std::process::exit(1);
        }
    };

    (side_a, side_b, side_c)

}