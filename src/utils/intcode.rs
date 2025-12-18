use std::{collections::VecDeque, ops::RangeInclusive};

// Intcode Virtual Machine
#[derive(Debug, Clone)]
pub struct Machine {
    ip: usize,           // Instruction Pointer
    cs: Vec<isize>,      // Code Sequence
    iq: VecDeque<isize>, // Input queue
    oq: Vec<isize>,      // Output queue
    pm: [isize; 2],      // Parameter mode
    os: bool,            // Operating System running?
}

impl Machine {
    // *** All the basic shit ***
    // Create a new virtual machine
    pub fn new(prog: &Vec<isize>) -> Self {
        Self { 
            ip: 0,
            cs: prog.clone(),
            iq: VecDeque::new(),
            oq: Vec::new(),
            pm: [0; 2],
            os: true,
        }
    }

    // Run the machine
    pub fn run(&mut self) {
        while self.os {
            let opcode = self.fetch_inst();
            match opcode {
                1  => self.add(),
                2  => self.mul(),
                3  => self.inp(),
                4  => self.out(),
                5  => self.jnz(),
                6  => self.jz(),
                7  => self.lt(),
                8  => self.eq(),
                99 => self.hcf(),
                _  => panic!("Invalid opcode"),
            }
        }
    }

    // Fetch the next instruction
    fn fetch_inst(&mut self) -> isize {
        let mut code = self.cs[self.ip];
        let opcode = code % 100;
        code /= 100;

        for idx in 0..2 {
            self.pm[idx] = code % 10;
            code /= 10;
        }

        opcode
    }

    // Read parameters for an operation
    fn get_params(&mut self, size: usize) -> Vec<usize> {
        let mut params = Vec::with_capacity(size);

        for offset in 1..=size {
            params.push(self.cs[self.ip + offset] as usize);
        }

        params
    }

    // Increment the instruction pointer
    fn inc_ptr(&mut self, offset: usize) {
        if self.ip < self.cs.len() - offset {
            self.ip += offset;
        } else {
            self.hcf();
        }
    }

    // Inject a value at a given memory location
    pub fn inject(&mut self, index: usize, value: isize) {
        self.cs[index] = value;
    }

    // Extends the input queue
    pub fn input_ext(&mut self, inputs: &[isize]) {
        self.iq.extend(inputs.iter());
    }

    // Load inputs into queue
    pub fn load(&mut self, inputs: VecDeque<isize>) {
        self.iq = inputs;
    }

    // Returns a value based on parameter mode
    pub fn param(&mut self, params: &Vec<usize>, pos: usize) -> isize {
        let param = params[pos];
        if self.pm[pos] == 1 {
            return param as isize;
        } else {
            return self.cs[param];
        }
    }

    // Read the value at a given location
    pub fn read(&self, index: usize) -> isize {
        self.cs[index]
    }

    // Outputs only the last entry of the output
    pub fn read_last(&self) -> usize {
        *self.oq.last().unwrap() as usize
    }

    // Displays the output queue
    pub fn read_out(&self) {
        println!("{:?}", self.oq);
    }

    // Resets the machine and loads a program 
    pub fn reboot(&mut self, prog: &Vec<isize>) {
        self.ip = 0;
        self.cs = prog.clone();
        self.os = true;
    }

    // SHOW content of memory location
    pub fn show(&self, pos: usize) {
        println!("{}", self.cs[pos]);
    }

    // *** All the opcode shit ***
    // Opcode 1 - ADD values from indices A and B, place into index C
    fn add(&mut self,) {
        let params = self.get_params(3);
        self.cs[params[2]] = self.param(&params, 0) + self.param(&params, 1);
        self.inc_ptr(4);
    }

    // Opcode 2 - MULTIPLY values from indices A and B, place into index C
    fn mul(&mut self) {
        let params = self.get_params(3);
        self.cs[params[2]] = self.param(&params, 0) * self.param(&params, 1);
        self.inc_ptr(4);
    }

    // Opcode 3 - Takes an INPUT value, and stores it at address X
    fn inp(&mut self) {
        let params = self.get_params(1);
        self.cs[params[0]] = self.iq.pop_front().unwrap();
        self.inc_ptr(2);
    }

    // Opcode 4 - OUTPUTS a value from address X
    fn out(&mut self) {
        let params = self.get_params(1);
        let output = self.param(&params, 0);
        self.oq.push(output);
        self.inc_ptr(2);
    }

    // Opcode 5 - JUMP-IF-TRUE, if the value in A is non-zero, sets to instruction pointer to value B
    fn jnz(&mut self) {
        let params = self.get_params(2);
        if self.param(&params, 0) != 0 {
            self.ip = self.param(&params, 1) as usize;
        } else {
            self.inc_ptr(3);
        }
    }

    // Opcode 6 - JUMP-IF-FALSE, if a value in A is zero, sets to instruction pointer to value B
    fn jz(&mut self) {
        let params = self.get_params(2);
        if self.param(&params, 0) == 0 {
            self.ip = self.param(&params, 1) as usize;
        } else {
            self.inc_ptr(3);
        }
    }

    // Opcode 7 - Tests if value A is LESS THAN value B, and puts the truth in value C
    fn lt(&mut self) {
        let params = self.get_params(3);
        self.cs[params[2]] = if self.param(&params, 0) < self.param(&params, 1) {
            1
        } else {
            0
        };
        self.inc_ptr(4);
    }

    // Opcode 8 - Tests if value A is EQUAL to value B, and puts the truth in value C
    fn eq(&mut self) {
        let params = self.get_params(3);
        self.cs[params[2]] = if self.param(&params, 0) == self.param(&params, 1) {
            1
        } else {
            0
        };
        self.inc_ptr(4);
    }

    // Opcode 99 - Halt and Catch Fire
    fn hcf(&mut self) {
        self.os = false;
    }
}