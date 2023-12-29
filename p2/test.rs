use dict::{ Dict, DictIface };

fn main() {
    let s = "foo bar baz bar foo";

    match s.find("bar") {
        Some(i) => println!("found \"bar\" at {}", i),
        None => println!("no match"),
    }
    
    match s.rfind("barr") {
        Some(i) => println!("found last \"bar\" at {}", i),
        None => println!("no match"),
    }

    // dictionary of numbers 
    let mut d = Dict::new();

    d.add("zero".to_string(), 0);
    d.add("one".to_string(), 1);
    
    for o in &mut d {
        o.val += 1;
    }

    for o in &d {
        println!("{}: {}", o.key, o.val);
    }
}
