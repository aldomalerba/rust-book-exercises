fn main() {
    if_conditions(false);

    loops_with_loop();
    loops_with_while();
    loops_with_for();
    loops_range_with_for();
}

fn if_conditions(condition: bool) {
    let number = if condition { 5 } else { 6 };

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loops_with_loop() {
    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

fn loops_with_while() {
    let numbers = [3,2,1];
    let mut index = 0;

    while index < 3 {
        println!("{}", numbers[index]);

        index += 1;
    }

    println!("LIFTOFF!!!");
}

fn loops_with_for() {
    let numbers = [3,2,1];

    for n in numbers {
        println!("{n}")
    }

    println!("LIFTOFF!!!");
}


fn loops_range_with_for() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}



