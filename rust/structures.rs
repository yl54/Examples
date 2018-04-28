// This rust application practices more complex structures.

use std::collections::VecDeque;

fn main() {
    println!("test_tuple_struct()");
    test_tuple_struct();
    println!();

    println!("test_struct_game()");
    test_struct_game();
    println!();

    println!("test_enum()");
    test_enum();
    println!();

    println!("test_vecdeque()");
    test_vecdeque();
    println!();
}

// This is an example tuple struct.
// A person has a name and age.
// Note: fields of structs, pairs, whatever cannot have unknown length
//         at compile time. a str is unknown, while a &'static is known
//         to have whatever is declared in code.
struct Person(&'static str, i16);


// String information: 
// Viewing a String as a &str is cheap, but converting the &str to a 
//   String involves allocating memory. No reason to do that unless 
//   you have to!

// look up encoding, unicode
// 8 bit encoding
// if 1 byte: top bit is 0, everything can be whatever
// if 2 byte: top bit of first byte is 1, get the next byte ready
// the most common characters can be sent in one byte, lower amt to transfer
// unicode is 4 bytes

// Note: str is an unsized type at compile time.
//       &str is a reference to a string.
//       "Hello there." is a string literal and its type is &'static str.
//       string literals are slices that are statically allocated, which 
//         means that its saved inside the compiled program and runs for
//         the entire duration of the program.
//       String is a heap allocated string. It is growable and UTF-8. Be
//         careful about too much string manipulation.
fn test_tuple_struct() -> () {
    // Set up the data to pass in.
    // Note: Make sure that your strings are statically typed.
    let name_1: &'static str = "QW";
    let name_2: &'static str = "QW";
    let name_3: &'static str = "ERT";

    // let name_1: String = "QW";
    // let name_2: String = "QW";
    // let name_3: String = "ER";

    let age_1 = 10;
    let age_2 = 30;
    let age_3 = 10;

    // Create Person objects from the data.
    let person_1: Person = Person(name_1, age_1);
    let person_2: Person = Person(name_2, age_2);
    let person_3: Person = Person(name_3, age_3);

    // let person_1: Person = Person(age_1, name_1);
    // let person_2: Person = Person(age_2, name_2);
    // let person_3: Person = Person(age_3, name_3);
    
    // Place Person objects into an array.
    let people: [Person; 3] = [person_1, person_2, person_3];
    let people_slice: &[Person] = &people;

    // Print out the struct data.
    let mut count: i16 = 1;

    // Slice does not need iterator explicitly declared.
    println!("Slice iteration:");
    for person in people_slice {
        println!("{}: {}, {}", count, person.0, person.1);
        count += 1;
    }

    // Arrays need an iterator declared for them.
    println!("Array iteration:");
    for person in people.iter() {
        println!("{}: {}, {}", count, person.0, person.1);
        count += 1;
    }


    // Note: do not declare variables and then do nothing with the variables.
    // count = 0;

    // Compare the struct data.
    // Lesson: str objects with the same value are regarded as the same.
    //         i16 objects with the same value are regarded as the same.
    for p1 in people.iter() {
        for p2 in people.iter() {
            if p1.0 == p2.0 {
                println!("{} and {} are the same", p1.0, p2.0);
            } else {
                println!("{} and {} are not the same", p1.0, p2.0);
            }

            if p1.1 == p2.1 {
                println!("{} and {} are the same", p1.1, p2.1);
            } else {
                println!("{} and {} are not the same", p1.1, p2.1);
            }
        }
    } 
}

struct Game {
    name: &'static str,
    age: u16,
    rules: [&'static str; 4],
}

fn test_struct_game() {
    // Set up the data to pass into.
    let name: &'static str = "basketball";
    let age: u16 = 60;
    let rules: [&'static str; 4] = ["no double dribble", "pump fake", 
                                    "always shoot", "never pass"];

    // Make the game object.
    let g: Game = Game {
        name: name,
        age: age,
        rules: rules,
    };

    // Pass into another function to do stuff.
    print_struct_game(g);
}

fn print_struct_game(g: Game) -> () {
    // Print out the game value.
    println!("name: {}", g.name);

    // Print out the game age.
    println!("age: {}", g.age);

    // Print out the game rules.
    println!("rules:");
    for r in g.rules.iter() {
        println!("{}", r);
    }
}

// Either do stuff with "left" or "right".
// Easier to use static strings b/c most of the time don't need variable string length.
// I don't think Flume internals will need heap allocated strings.
enum Event {
    Pass(&'static str),
    Shot(&'static str),
    Screen(&'static str),
    Dribble(&'static str),
    Catch(&'static str),
}

fn test_enum() {
    let m1 = Event::Pass("left");
    let m2 = Event::Screen("right");
    let m3 = Event::Catch("both");
    let m4 = Event::Dribble("right");
    let m5 = Event::Shot("right");
    let mset: [Event; 5] = [m1, m2, m3, m4, m5];
    for m in mset.iter() {
        show_event(m);
    }
}

// Note: Not sure why the address needs to be referenced.
fn show_event(event: &Event) {
    match event {
        &Event::Pass(h) => println!("passed the ball with {}", h),
        &Event::Shot(h) => println!("shot the ball with {}", h),
        &Event::Screen(h) => println!("screened the ball with {}", h),
        &Event::Dribble(h) => println!("dribbled the ball with {}", h),
        &Event::Catch(h) => println!("caught the ball with {}", h),
    }
}

fn test_vecdeque() {
    // Make a Vecdeque object.
    let mut q = VecDeque::new();

    // Add a few objects into it.
    add_to_vecdeque(&mut q);

    // Read all of the objects that have been placed into it.
    read_from_vecdeque(&mut q);
}

// Note: Since we do not want to take ownership here and subsequently deallocate
//         the queue, we only allow this function to borrow the object. 
fn add_to_vecdeque(q: &mut VecDeque<i16>) {
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    q.push_back(4);
    q.push_back(5);
    q.push_back(6);
}

// Note: Since this is the last function called in test_vecdeque(), it is ok
//         for this function to either take ownership (and deallocate at the
//         end of the function) or borrow (and not deallocate at the end of
//         this function).
fn read_from_vecdeque(q: &mut VecDeque<i16>) {
    // Check if there are 0 values in the queue.

    // Check if there are any values left in the queue.
    println!("Start pulling out from queue:");
    while !q.is_empty() {
        // Remove from the queue.
        let obj = q.pop_front();
        match obj {
            Some(num) => println!("obj: {}", num),
            None => println!("obj does not match anything"),
        }
    }
}

