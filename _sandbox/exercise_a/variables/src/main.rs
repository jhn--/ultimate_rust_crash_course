// fn main() {
//     println!("Hello, world!");
//     let missiles = 8;
//     let ready = 2;
//     println!("Firing {} of my {} missiles...", ready, missiles);

//     let mut missiles = missiles;
//     missiles = missiles - ready;
//     println!("{} missiles left", missiles);
// }

/*
part 1
[root@Gilgamesh variables]# cargo run
   Compiling variables v2.3.4 (/mnt/d/Users/User/Dropbox/work/courses/udemy/ultimate_rust_crash_course/_sandbox/exercise_a/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 4.15s
     Running `target/debug/variables`
Hello, world!
Firing 2 of my 8 missiles...
*/

/* 
part 2
[root@Gilgamesh variables]# cargo run
   Compiling variables v2.3.4 (/mnt/d/Users/User/Dropbox/work/courses/udemy/ultimate_rust_crash_course/_sandbox/exercise_a/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 2.38s
     Running `target/debug/variables`
Hello, world!
Firing 2 of my 8 missiles...
6 missiles left
*/

// const STARTING_MISSILES: i32 = 8;
// const READY_AMOUNT: i32 = 2;

// fn main() {
//     let mut missiles = STARTING_MISSILES;
//     let ready = READY_AMOUNT;
//     println!("Firing {} of my {} missiles...", ready, missiles);

//     missiles = missiles - ready;
//     println!("{} missiles left", missiles);
// }


/*
part 2 cont
[root@Gilgamesh variables]# cargo run
warning: `variables` (bin "variables") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/variables`
Hello, world!
Firing 2 of my 8 missiles...
6 missiles left
*/

const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready);
}

/*
extra challenges

[root@Gilgamesh variables]# cargo run
   Compiling variables v2.3.4 (/mnt/d/Users/User/Dropbox/work/courses/udemy/ultimate_rust_crash_course/_sandbox/exercise_a/variables)
warning: variable does not need to be mutable
  --> src/main.rs:61:10
   |
61 |     let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
   |          ----^^^^^^^^
   |          |
   |          help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `variables` (bin "variables") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 2.37s
     Running `target/debug/variables`
Firing 2 of my 8 missiles...
6 missiles left
*/

// const STARTING_MISSILES: i32 = 8;
// const READY_AMOUNT: i32 = 2;

// fn main() {
//     let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
//     let unused = 1;
//     println!("Firing {} of my {} missiles...", ready, missiles);

//     println!("{} missiles left", missiles - ready);
// }

/*
[root@Gilgamesh variables]# cargo run
   Compiling variables v2.3.4 (/mnt/d/Users/User/Dropbox/work/courses/udemy/ultimate_rust_crash_course/_sandbox/exercise_a/variables)
warning: unused variable: `unused`
  --> src/main.rs:62:9
   |
62 |     let unused = 1;
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:61:10
   |
61 |     let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
   |          ----^^^^^^^^
   |          |
   |          help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `variables` (bin "variables") generated 2 warnings  
    Finished dev [unoptimized + debuginfo] target(s) in 2.46s
     Running `target/debug/variables`
Firing 2 of my 8 missiles...
6 missiles left
*/

// const STARTING_MISSILES: i32 = 8;
// const READY_AMOUNT: i32 = 2;

// fn main() {
//     READY_AMOUNT = 1;
//     let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
//     let unused = 1;
//     println!("Firing {} of my {} missiles...", ready, missiles);

//     println!("{} missiles left", missiles - ready);
// }

/*
[root@Gilgamesh variables]# cargo run
   Compiling variables v2.3.4 (/mnt/d/Users/User/Dropbox/work/courses/udemy/ultimate_rust_crash_course/_sandbox/exercise_a/variables)
error[E0070]: invalid left-hand side of assignment
   --> src/main.rs:135:18
    |
135 |     READY_AMOUNT = 1;
    |     ------------ ^
    |     |
    |     cannot assign to this expression

For more information about this error, try `rustc --explain E0070`.
error: could not compile `variables` due to previous error
[root@Gilgamesh variables]# 
*/