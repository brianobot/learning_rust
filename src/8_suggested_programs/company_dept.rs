use std::collections::HashMap;

fn add_to_department(staff_name: String, department_name: String, mut roster: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let dept_data = roster.entry(department_name).or_insert(vec![]);
    dept_data.push(staff_name);
    roster
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    let staffs = [
        "Brian Obot",
        "Tonie Engine",
        "Joe Diamond",
        "Emeritus",
        "Deaconess",
    ];

    for staff in staffs {
        let staff = staff.to_string();
        departments = add_to_department(staff, "IT".to_string(), departments);
    }

    println!("DEPARTMENTS = {:?}", departments);

}