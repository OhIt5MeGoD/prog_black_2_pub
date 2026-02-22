fn main() {
    let ollie: String = "Ollie".to_string();
    let jed: String = "Jed".to_string();
    print_greeting(&jed, &ollie);
    println!();
    print_greeting(&ollie, &jed);
    println!();

    let mut names: Vec<Value> = vec![Value::Text("Ollie".to_string()), Value::Text("Jed".to_string()), Value::Text("Neshesh".to_string()), Value::Int(7)];
    print_pairs_greeting(&names);
    println!();

    names[0] = Value::Text("Ollie2".to_string());
    print_pairs_greeting(&names);
    println!();
}

enum Value {
    Int(i32),
    Text(String)
}

fn as_string(v: &Value) -> String {
    match v {
        Value::Int(i) => i.to_string(),
        Value::Text(s) => s.clone(),
    }
}

fn print_greeting(name1: &str, name2: &str) {
    println!("Hello {}, from {}", name1, name2);
}

fn print_pairs_greeting(names: &Vec<Value>) {
    for i in 0..names.len() {
        for j in 0..names.len() {
            if i != j {
                println!("Hello {}, from {}", as_string(&names[j]), as_string(&names[i]));
            }
        }
    }
}