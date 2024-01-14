use std::collections::HashMap;

struct HumanResources {
    employees: HashMap<String, String>,
    count: u32,
}

impl HumanResources {
    fn new() -> HumanResources {
        HumanResources {
            employees: HashMap::<String, String>::new(),
            count: 0,
        }
    }

    fn add(&mut self, s: &str) {
        let cmd: Vec<&str> = s.splitn(4, ' ').collect();
        let (name, dep) = (cmd.get(1), cmd.get(3));

        match (name, dep) {
            (Some(x), Some(y)) => {
                if self.employees.contains_key(*x) {
                    eprintln!("Error: employee {} is already at {}", x, y)
                } else {
                    self.employees.insert(x.to_string(), y.to_string());
                    self.count += 1;
                }
            }
            (None, Some(_)) => eprintln!("Error: failed to parse name"),
            (Some(_), None) => eprintln!("Error: failed to parse dep"),
            (None, None) => eprintln!("Error: failed to parse name and dep"),
        }
    }

    fn list(&self) {
        if self.count == 0 {
            println!("No-one has been added yet")
        }
        for (k, v) in self.employees.iter() {
            println!("{} is at {}.", k, v)
        }
    }
}

fn main() {
    let mut hr = HumanResources::new();
    hr.list();

    hr.add("add Sally to Sales");
    hr.add("add Jim to Marking");
    hr.list();

    hr.add("");
    hr.add("add");
    hr.add("add Sally");
    hr.add("add Sally to");
    hr.add("add Sally to Marketing");
}
