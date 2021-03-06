use std::collections::HashMap;
fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key is not there");
    let value = args.next().unwrap();
    println!("The key is {} and the value is {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    //std::fs::write("kv.db", contents).unwrap();
    
    let database = Database::new().expect("Database::new() crashed");
}

struct Database {
    map: HashMap<String,String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        /*let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Err(error);
            }
        };*/

        let contents = std::fs::read_to_string("kv.db")?;
    
        Ok(Database {
            map: HashMap::new(),
        })
    }
}