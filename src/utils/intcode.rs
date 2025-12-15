use std::ops::RangeInclusive;

// Intcode Virtual Machine
#[derive(Debug, Clone)]
pub struct Machine {
    ip: usize,        // Instruction Pointer
    cs: Vec<usize>,   // Code Sequence
    os: bool,         // Operating System running?
}

impl Machine {
    // *** All the basic shit ***
    // Create a new virtual machine
    pub fn new(prog: &Vec<usize>) -> Self {
        Self { 
            ip: 0,
            cs: prog.clone(),
            os: true,
        }
    }

    // Run the machine
    pub fn run(&mut self) {
        while self.os {
            match self.get_opcode() {
                1  => self.add(),
                2  => self.mul(),
                99 => self.hcf(),
                _  => panic!("Invalid opcode"),
            }
        }
    }

    // Get the opcode
    fn get_opcode(&self) -> usize {
        self.cs[self.ip]
    }

    // Read parameters for an operation
    fn get_params(&mut self) -> [usize; 3] {
        [self.cs[self.ip + 1], self.cs[self.ip + 2], self.cs[self.ip + 3]]
    }

    // Increment the instruction pointer
    fn inc_ptr(&mut self) {
        if self.ip < self.cs.len() - 4 {
            self.ip += 4;
        } else {
            self.hcf();
        }
    }

    // Inject a value at a given memory location
    pub fn inject(&mut self, index: usize, value: usize) {
        self.cs[index] = value;
    }

    // Read the value at a given location
    pub fn read(&self, index: usize) -> usize {
        self.cs[index]
    }

    // Resets the machine and loads a program 
    pub fn reboot(&mut self, prog: &Vec<usize>) {
        self.ip = 0;
        self.cs = prog.clone();
        self.os = true;
    }

    // *** All the opcode shit ***
    // Opcode 0 - ADD values from indices A and B, place into index C
    fn add(&mut self,) {
        let [loc1, loc2, loc3] = self.get_params();
        self.cs[loc3] = self.cs[loc1] + self.cs[loc2];
        self.inc_ptr();
    }

    // Opcode 1 - MULTIPLY values from indices A and B, place into index C
    fn mul(&mut self) {
        let [loc1, loc2, loc3] = self.get_params();
        self.cs[loc3] = self.cs[loc1] * self.cs[loc2];
        self.inc_ptr();
    }

    // Opcode 99 - Halt and Catch Fire
    fn hcf(&mut self) {
        self.os = false;
    }
}