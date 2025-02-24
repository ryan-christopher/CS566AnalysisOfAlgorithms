// Stack struct has two fields, elements (stored as a vector) and 
// min_index (an optional usize value)
#[derive(Default)]
pub struct Stack {
    pub elements: Vec<i32>,
    pub min_index: Option<usize>,
}

// implementations on the Stack struct
impl Stack {
    // push an element to the end of the stack
    fn push(&mut self, new_val: i32) {
        self.elements.push(new_val);
        // if the vector is empty, the first vaue will be the minimum value
        if self.elements.len() == 1 {
            self.min_index = Some(0);
        }
        // otherwise compare the value added to the current mininum
        else {
            if self.elements[self.min_index.unwrap() as usize] > new_val {
                self.min_index = Some(self.elements.len() - 1);
            }
        }
    }

    // pop the element from the end of the stack
    fn pop(&mut self) -> Option<i32> {
        let pop_element = self.elements.pop();
        // call find_min when the minimum value is popped
        if self.min_index == Some(self.elements.len()) {
            self.find_min();
        }
        pop_element
    }

    // when a new minimum value needs to be found, find_min
    // will iterate through the elements and find the index 
    // of the lowest value
    fn find_min(&mut self) {
        let mut new_min_index = 0;
        for n in 1..self.elements.len() {
            if self.elements[n] < self.elements[new_min_index] {
                new_min_index = n;
            }
        }
        self.min_index = Some(new_min_index);
    }

    // retrieve the minimum value from the stack
    fn min(&self) -> String {
        if self.elements.len() > 0 {
            self.elements[self.min_index.unwrap()].to_string()
        }
        // if the vector is empty, there is no min
        else {
            "None".to_string()
        }
    }
}

pub fn stack_tests() {
    // test case 1
    println!("=== Stack 1 ===");
    let mut stack_1: Stack = Default::default();
    println!("{:?}", stack_1.elements);
    println!("Minumum Element: {}", stack_1.min());
    stack_1.push(5);
    stack_1.push(2);
    stack_1.push(10);
    println!("{:?}", stack_1.elements);
    println!("Minumum Element: {}", stack_1.min());
    
    stack_1.push(1);
    println!("{:?}", stack_1.elements);
    println!("Minumum Element: {}", stack_1.min());
    println!("Popped val: {:?}", stack_1.pop().unwrap());
    println!("{:?}", stack_1.elements);
    println!("Minumum Element: {}", stack_1.min());
    // test case 2
    println!("=== Stack 2 ===");
    let mut stack_2: Stack = Default::default();
    println!("{:?}", stack_2.elements);
    stack_2.push(1);
    stack_2.push(7);
    stack_2.push(2);
    println!("{:?}", stack_2.elements);
    println!("Minumum Element: {}", stack_2.min());
    println!("Popped val: {:?}", stack_2.pop().unwrap());
    println!("{:?}", stack_2.elements);
}