use std::env;
use std::io;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use std::path::Path;


fn main() {
    // get cli arguments
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];

    match command.as_str() {
        "budget" => show_budget(),
        "goal" => set_goal(&args),
        "purchase" => add_purchase(&args),
        _ => println!("{:?} is invalid. Try help option for available options", command)
    }
}

fn add_purchase(args: &Vec<String>) {
    // check that arguments are present
    if args.len() < 6 {
        io::Error::new(io::ErrorKind::Other, "Insufficient arguments were given for adding purchase");
    } 

    let amount: i32 = args[2].parse::<i32>().unwrap();
    let category: String = args[3].clone();
    let date: String = args[4].clone();
    let description: String = args[5].clone();
    let purchase = Purchase::new(amount, category, date, description);

    // add purchase to budget
    add_to_record(purchase).expect("Failed to add purchase to record");
}

fn add_to_record(purchase: Purchase) -> Result<(), io::Error> {
    let path = Path::new("/home/lee/.budget/current");

    // if using this please specify where you would like the file to be
    let mut db = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to get file");

    let mut content = String::new();
    db.read_to_string(&mut content).expect("Failed to read contents of file");

    let amount = purchase.amount.to_string();
    let entry: String = format!("\n{} {} {} {}", amount, purchase.category, purchase.date, purchase.description);
    
    use std::io::Write;

    db.write_all(&entry.as_bytes())
}

fn set_goal(args: &Vec<String>) {
    let path = Path::new("/home/lee/.budget/goals");
    let mut db = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to get file");

    let mut content = String::new();
    db.read_to_string(&mut content).expect("Failed to read file");


    let mut goals: HashMap<&str, i32> = HashMap::new();
    for line in content.split('\n') {
        // parse content of lines. should be " category amount"
        if line != "" {
            let items: Vec<&str> = line.split(' ').collect();
            let amount: i32 = items[0].parse::<i32>().unwrap();
            let category: &str = items[1];
            goals.insert(category, amount);
        }
    }

    let category = args[1].clone();
    let amount = args[2].parse::<i32>().unwrap();
    goals.insert(&category, amount);

    for key in goals.keys() {
        use std::io::Write;

        let item = (key, goals.get(key).unwrap());
        let entry = format!("{} {}", item.0, item.1);    
    
        db.write(&entry.as_bytes()).expect("Failed to write to file");
    }
}

fn get_goals<'a>() -> HashMap<String, i32> {
    let path = Path::new("/home/lee/.budget/goals");
    let mut db = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to get file");

    let mut content = String::new();
    db.read_to_string(&mut content).expect("Failed to read file");


    let mut goals: HashMap<String, i32> = HashMap::new();
    for line in content.split('\n') {
        // parse content of lines. should be " category amount"
        if line != "" {
            let items: Vec<&str> = line.split(' ').collect();
            let amount: i32 = items[1].parse::<i32>().unwrap();
            let category: String = String::from(items[0]);
            goals.entry(category).or_insert(amount);
        }
    }

    goals
}
fn show_budget() {
    let path = Path::new("/home/lee/.budget/current");
    let mut db = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Failed to open file");
    
    let mut content = String::new();
    db.read_to_string(&mut content).expect("Failed to read file");

    // parse contents
    let lines: Vec<&str> = content.split('\n').collect();
    let mut purchases: HashMap<&str, Vec<Purchase>> = HashMap::new();
    let mut totals: HashMap<&str, i32> = HashMap::new();
    let goals: HashMap<String, i32> = get_goals();
    for line in lines {
        if line == "" {
            // no-op
        } else {
            let mut item: Vec<&str> = line.split(" ").collect(); 
            let amount: i32 = item[0].parse::<i32>().unwrap();
            let category = String::from(item[1]);
            // update the totals so we can show that later
            *totals.entry(item[1]).or_insert(0) += amount;
            let date = String::from(item[2]);
            let description: String = item.drain(3..).into_iter().map(|word| String::from(word)).collect();

            let purchase = Purchase::new(amount, category, date, description);
            purchases.entry(&item[1]).or_insert_with(Vec::new).push(purchase);
        }
    }

    // display retrived values
    for category in purchases.keys() {
        if goals.contains_key(category.clone()) {
            println!("{}    Total: ${}    Planned: ${}", category, totals.get(category).unwrap(), goals.get(category.clone()).unwrap());
            let items = purchases.get(category).unwrap().into_iter();
            for purchase in items {
                println!("{}    {}    {}", purchase.amount, purchase.date, purchase.description)
            }
        } else {
            println!("{}    Total: ${}", category, totals.get(category).unwrap());
            let items = purchases.get(category).unwrap().into_iter();
            for purchase in items {
                println!("{}    {}    {}", purchase.amount, purchase.date, purchase.description)
            }
        }
    }
}


struct Purchase {
    amount: i32,
    category: String,
    date: String,
    description: String,
}

impl Purchase {
    fn new(amount: i32, category: String, date: String, description: String) -> Purchase {
        Purchase {
            amount: amount, 
            category: category,
            date: date, 
            description: description,
        }
    }
}
