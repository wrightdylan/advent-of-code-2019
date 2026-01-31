use std::{collections::VecDeque, ops::RangeInclusive};

pub type Program = Vec<isize>;
pub type Memory = Vec<isize>;

// Intcode Virtual Machine
#[derive(Debug, Clone)]
pub struct Machine {
    ip: usize,           // Instruction Pointer
    cs: Memory,          // Code Sequence
    iq: VecDeque<isize>, // Input queue
    oq: Vec<isize>,      // Output queue
    pm: [isize; 3],      // Parameter mode
    os: bool,            // Operating System running?
    ps: bool,            // Pause operations (e.g. wait for input)
    rb: isize,           // Relative base
}

impl Machine {
    // *** All the basic shit ***
    // Create a new virtual machine
    pub fn new(prog: &Program) -> Self {
        Self { 
            ip: 0,
            cs: prog.clone(),
            iq: VecDeque::new(),
            oq: Vec::new(),
            pm: [0; 3],
            os: true,
            ps: false,
            rb: 0,
        }
    }

    // Run the machine
    pub fn run(&mut self) {
        while self.os && !self.ps {
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
                9  => self.rbx(),
                99 => self.hcf(),
                _  => panic!("Invalid opcode"),
            }
        }
    }

    // ////////////////////////////////////////////////////////////////////////
    // Dump the output queue
    pub fn dump_output(&self) -> &Vec<isize> {
        &self.oq
    }

    // Fetch the next instruction
    fn fetch_inst(&mut self) -> isize {
        let mut code = self.cs[self.ip];
        let opcode = code % 100;
        code /= 100;

        for idx in 0..3 {
            self.pm[idx] = code % 10;
            code /= 10;
        }

        opcode
    }

    // Gets the address from memory
    fn get_addr(&mut self, offset: usize) -> usize {
        let addr = match self.pm[offset - 1] {
            0 => self.cs[self.ip + offset] as usize,
            1 => self.ip + offset,
            2 => (self.rb + self.cs[self.ip + offset]) as usize,
            _ => unreachable!(),
        };

        if addr >= self.cs.len() {
            self.cs.resize(addr + 1, 0);
        };

        addr
    }
    
    // Fetches a parameter for an operation according to parameter mode
    fn get_param(&mut self, offset: usize) -> isize {
        let addr = self.get_addr(offset);
        self.cs[addr]
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

    // Drains the output queue from one machine and uses it as the input for another
    pub fn input_from(&mut self, other: &mut Machine) {
        self.iq.extend(other.oq.drain(..));
    }

    // Checks if the machine is still running
    pub fn is_running(&self) -> bool {
        self.os
    }

    // Load inputs into queue
    pub fn load(&mut self, inputs: VecDeque<isize>) {
        self.iq = inputs;
    }

    // Parses the program
    pub fn parse(input: &str) -> Program {
        input
            .split(',')
            .map(|line| line.parse().unwrap())
            .collect()
    }

    // Pauses the operation and releases the machine
    pub fn pause(&mut self) {
        self.ps = true;
    }

    // Read the value at a given location
    pub fn read(&self, index: usize) -> isize {
        self.cs[index]
    }

    // Outputs only the last entry of the output
    pub fn read_last(&self) -> isize {
        *self.oq.last().unwrap()
    }

    // Displays the output queue
    pub fn read_out(&self) {
        println!("{:?}", self.oq);
    }

    // Resets the machine and loads a program 
    pub fn reboot(&mut self, prog: &Program) {
        self.ip = 0;
        self.cs = prog.clone();
        self.iq.clear();
        self.oq.clear();
        self.pm = [0; 3];
        self.os = true;
        self.ps = false;
        self.rb = 0;
    }

    // Resumes operation
    pub fn resume(&mut self) {
        self.ps = false;
        self.run();
    }

    // SHOW content of memory location
    pub fn show(&self, pos: usize) {
        println!("{}", self.cs[pos]);
    }

    // *** All the opcode shit ***
    // Format of instruction: ABCDE
    // A - mode of 3rd parameter
    // B - mode of 2nd parameter
    // C - mode of 1st parameter
    // DE - two-digit opcode
    // Modes: 0 - position, 1 - immediate, 2 - relative
    // Parameters that an instruction writes to will never be in immediate mode.

    // Opcode 1 - ADD values from indices A and B, place into index C
    fn add(&mut self,) {
        let addr = self.get_addr(3);
        self.cs[addr] = self.get_param(1) + self.get_param(2);
        self.inc_ptr(4);
    }

    // Opcode 2 - MULTIPLY values from indices A and B, place into index C
    fn mul(&mut self) {
        let addr = self.get_addr(3);
        self.cs[addr] = self.get_param(1) * self.get_param(2);
        self.inc_ptr(4);
    }

    // Opcode 3 - Takes an INPUT value, and stores it at address X
    fn inp(&mut self) {
        if let Some(inst) = self.iq.pop_front() {
            let addr = self.get_addr(1);
            self.cs[addr] = inst;
            self.inc_ptr(2);
        } else {
            self.pause();
        }
    }

    // Opcode 4 - OUTPUTS a value from address X
    fn out(&mut self) {
        let output = self.get_param(1);
        self.oq.push(output);
        self.inc_ptr(2);
    }

    // Opcode 5 - JUMP-IF-TRUE, if the value in A is non-zero, sets the instruction pointer to value B
    fn jnz(&mut self) {
        if self.get_param(1) != 0 {
            self.ip = self.get_param(2) as usize;
        } else {
            self.inc_ptr(3);
        }
    }

    // Opcode 6 - JUMP-IF-FALSE, if a value in A is zero, sets the instruction pointer to value B
    fn jz(&mut self) {
        if self.get_param(1) == 0 {
            self.ip = self.get_param(2) as usize;
        } else {
            self.inc_ptr(3);
        }
    }

    // Opcode 7 - Tests if value A is LESS THAN value B, and puts the truth in value C
    fn lt(&mut self) {
        let addr = self.get_addr(3);
        self.cs[addr] = if self.get_param(1) < self.get_param(2) {
            1
        } else {
            0
        };
        self.inc_ptr(4);
    }

    // Opcode 8 - Tests if value A is EQUAL to value B, and puts the truth in value C
    fn eq(&mut self) {
        let addr = self.get_addr(3);
        self.cs[addr] = if self.get_param(1) == self.get_param(2) {
            1
        } else {
            0
        };
        self.inc_ptr(4);
    }

    // Opcode 9 - Adjusts the relative base by an offset
    fn rbx(&mut self) {
        self.rb += self.get_param(1);
        self.inc_ptr(2);
    }

    // Opcode 99 - Halt and Catch Fire
    fn hcf(&mut self) {
        self.os = false;
        self.ps = true;
    }
}