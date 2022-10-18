mod old;

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    let emp = Employee {
        name: String::from("John"),
        age: 35
    };
    println!("{:#?}", emp);
    println!("{:#?}", emp.age);
    println!("{:#?}", emp.details());
    println!("{:#?}", Employee::static_details());
}

#[allow(dead_code)]
#[derive(Debug)]
struct Employee {
    name: String,
    age: i32
}

impl Employee {
    fn details(&self) -> String {
        format!("name: {}, age: {}", &self.name, &self.age)
    }

    fn static_details() -> String {
        String::from("Details static")
    }
}