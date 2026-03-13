// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then, let the user retrieve a list of all people in a department or all people
// in the company by department, sorted alphabetically.

use std::collections::HashMap;

pub struct Directory {
    departments: HashMap<String, Department>,
}

struct Department {
    employees: Vec<Employee>,
}

struct Employee {
    name: String,
}

impl Directory {
    pub fn new() -> Self {
        Directory {
            departments: HashMap::new(),
        }
    }

    pub fn send_text_command(&mut self, text_command: &str) {
        // split and collect words into a Vector
        let words: Vec<&str> = text_command.split_ascii_whitespace().collect();

        // println!("COMMAND:");
        // for word in words {
        //     println!("- {word}");
        // }

        let Some(employee_name) = words.get(1) else {
            println!("Employee name is not valid.");
            return;
        };

        let Some(department_name) = words.get(3) else {
            println!("Department name is not valid.");
            return;
        };

        println!("E: {employee_name}, D: {department_name}");

        self.add_employee_to_department(employee_name, department_name);
    }

    fn add_employee_to_department(&mut self, employee_name: &str, department_name: &str) {
        let dept = self.departments
            .entry(String::from(department_name))
            .or_insert(Department { employees: Vec::new() });

        // could use BTreeMap for even faster shift-less inserts in the future here
        // But this is a Vector exercise
        // Approach	                Search	    Insert/Shift	Total
        // push + sort	            —	        O(n log n)	    O(n log n)
        // binary search + insert	O(log n)	O(n) shift	    O(n)
        // BTreeMap (future)	    O(log n)	O(log n)	    O(log n)
        let position = dept.employees
            .binary_search_by_key(&employee_name, |e| e.name.as_str());

        //determine the position in the Vector we can insert this Employee at to maintain sorting
        let insert_at = match position {
            Ok(i) | Err(i) => i,
        };

        // Insert the employee at the proper position
        dept.employees.insert(insert_at, Employee { name: String::from(employee_name) });
    }

    // list all people in a department, sorted alphabetically
    pub fn list_employees_in_department(&self, department_name: &str) {
        let Some(department) = self.departments.get(department_name) else {
            println!("{department_name} department not found.");
            return;
        };

        println!("{department_name} Department:");

        for employee in &department.employees {
            let employee_name: &String = &employee.name;
            println!("- {employee_name}");
        }
    }

    // list all people in the company by department, sorted alphabetically
    pub fn list_all_employees(&self) {

        println!("=======================");
        println!("ALL DEPARTMENTS REPORT:");
        println!("----------------------");

        for department in &self.departments {
            self.list_employees_in_department(department.0);
        }

        println!("=======================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. A new directory starts with no departments
    #[test]
    fn new_creates_empty_directory() {
        let dir = Directory::new();
        assert!(dir.departments.is_empty());
    }

    // 2. Adding an employee to a new department creates that department
    #[test]
    fn add_employee_creates_new_department() {
        let mut dir = Directory::new();
        dir.add_employee_to_department("Alice", "Engineering");
        assert!(dir.departments.contains_key("Engineering"));
    }

    // 3. The employee's name is stored correctly
    #[test]
    fn add_employee_stores_correct_name() {
        let mut dir = Directory::new();
        dir.add_employee_to_department("Alice", "Engineering");
        assert_eq!(dir.departments["Engineering"].employees[0].name, "Alice");
    }

    // 4. Adding a second employee to the same department appends — does not overwrite
    #[test]
    fn add_second_employee_to_same_department() {
        let mut dir = Directory::new();
        dir.add_employee_to_department("Alice", "Engineering");
        dir.add_employee_to_department("Bob", "Engineering");
        assert_eq!(dir.departments["Engineering"].employees.len(), 2);
    }

    // 5. Employees in different departments are stored separately
    #[test]
    fn add_employees_to_different_departments() {
        let mut dir = Directory::new();
        dir.add_employee_to_department("Alice", "Engineering");
        dir.add_employee_to_department("Bob", "Sales");
        assert_eq!(dir.departments.len(), 2);
        assert_eq!(dir.departments["Engineering"].employees.len(), 1);
        assert_eq!(dir.departments["Sales"].employees.len(), 1);
    }

    // 6. send_text_command correctly parses "Add X to Y" and stores the employee
    #[test]
    fn send_text_command_adds_employee() {
        let mut dir = Directory::new();
        dir.send_text_command("Add Alice to Engineering");
        assert_eq!(dir.departments["Engineering"].employees[0].name, "Alice");
    }

    // 7. A second send_text_command to the same department appends, not overwrites
    #[test]
    fn send_text_command_appends_to_existing_department() {
        let mut dir = Directory::new();
        dir.send_text_command("Add Alice to Engineering");
        dir.send_text_command("Add Bob to Engineering");
        assert_eq!(dir.departments["Engineering"].employees.len(), 2);
    }

    // 8. An empty command does not panic
    #[test]
    fn send_text_command_empty_string_does_not_panic() {
        let mut dir = Directory::new();
        dir.send_text_command("");
    }

    // 9. A partial command missing the department does not panic
    #[test]
    fn send_text_command_missing_department_does_not_panic() {
        let mut dir = Directory::new();
        dir.send_text_command("Add Alice");
    }

    // 10. Employees in a department are stored in alphabetical order
    // NOTE: this test will FAIL until list_employees_in_department sorts employees
    #[test]
    fn employees_in_department_are_sorted_alphabetically() {
        let mut dir = Directory::new();
        dir.add_employee_to_department("Zara", "Sales");
        dir.add_employee_to_department("Alice", "Sales");
        dir.add_employee_to_department("Mike", "Sales");

        let names: Vec<&str> = dir.departments["Sales"]
            .employees
            .iter()
            .map(|e| e.name.as_str())
            .collect();

        assert_eq!(names, vec!["Alice", "Mike", "Zara"]);
    }
}
