const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let shadowing = 5;
    let mut shadowing = shadowing + 1;

    {
        let mut shadowing = shadowing * 2;
        println!("The value of shadowing in the inner scope is: {shadowing}");
    }

    println!("The value of shadowing is: {shadowing}");
}