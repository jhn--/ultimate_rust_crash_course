// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // println!("{:?}", args);

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // 1a. Your task: handle the command-line arguments!
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.
        if arg == "sum" {
            sum()
        } else if arg == "double" {
            double()
        } else {
            count(arg)
        }

        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"

        /*
        [root@Gilgamesh d_control_flow_strings]# cargo run sum
        Compiling d_control_flow_strings v0.1.0 ((((o(*°▽°*)o))))
            Finished dev [unoptimized + debuginfo] target(s) in 2.44s
            Running `target/debug/d_control_flow_strings sum`
        The sum is 0
        
        [root@Gilgamesh d_control_flow_strings]# cargo run double
            Finished dev [unoptimized + debuginfo] target(s) in 0.01s
            Running `target/debug/d_control_flow_strings double`
        You can double x 0 times until x is larger than 500

        [root@Gilgamesh d_control_flow_strings]# cargo run bananas
            Finished dev [unoptimized + debuginfo] target(s) in 0.01s
            Running `target/debug/d_control_flow_strings bananas`
     */
    }
}

fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`
    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
    /*
    [root@Gilgamesh d_control_flow_strings]# cargo run sum
    Compiling d_control_flow_strings v0.1.0 (☆*:.｡.o(≧▽≦)o.｡.:*☆)
        Finished dev [unoptimized + debuginfo] target(s) in 2.40s
        Running `target/debug/d_control_flow_strings sum`
        The sum is 255
    */
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.
    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
    /*
    [root@Gilgamesh d_control_flow_strings]# cargo run double
    Compiling d_control_flow_strings v0.1.0 (ヽ(o＾▽＾o)ノ)
        Finished dev [unoptimized + debuginfo] target(s) in 2.37s
        Running `target/debug/d_control_flow_strings double`
    You can double x 9 times until x is larger than 500
    */
}

fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
    let mut count = 0;
    loop {
        if count < 8 {
            print!("{}", arg);
            count += 1;
        } else {
            break;
        }
    }

    println!(); // This will output just a newline at the end for cleanliness.
    /*
    [root@Gilgamesh d_control_flow_strings]# cargo run bananas
    Compiling d_control_flow_strings v0.1.0 (/mnt/d/Users/User/Dropbox/work/courses/udemy/ultimate_rust_crash_course/_sandbox/exercise_d/d_control_flow_strings)
        Finished dev [unoptimized + debuginfo] target(s) in 2.41s
        Running `target/debug/d_control_flow_strings bananas`
    bananasbananasbananasbananasbananasbananasbananasbananas
    */
}
