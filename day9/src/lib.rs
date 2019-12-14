// Link to puzzle: https://adventofcode.com/2019/day/5
//
// Summary:
//
// OPCODES:
//  1:  Add instruction, followed by two parameters and lastly the address to
//      where the results should be stored.
//  2:  Multiply instruction, followed by two parameters and lastly the
//      address where the results should be stored.
//  3:  Takes a single integer as input and saves it to the address pointed
//      to by its only parameter.
//  4:  Outputs the value of its only parameter, for example instruction
//      '4, 5' would output the value at adress 50.
//
// PARAMETER MODES:
//  0 -> POSITION_MODE: parameter will be interpreted as a position, for
//     example:
//         parameter 50, the value is stored at adress 50 in memory.
//  1 -> IMMEDIATE_MODE: The parameter value is simply the value that will
//         be interpreted.
//
// INSTSTRUCTIONS FORMAT:
//    Parameter mode is stored in the same value as the instruction's
//    opcode.
//
//    OPCODE: The opcode is a two-digit number based only on the ones and
//    tens digits of the value.
//
// PARAMETERS:
//     Parameter modes are single digits, one per parameter, read right-
//     to-left from the opcode: the first parameter's mode is in the hundreds
//     digit, the second parameter's mode is in the thousands digit, the third
//     parameter's mode is in the ten-thousands digit, and so on.
//     Missing modes are 0.
//
// EXAMPLE PROGRAM: '1002,4,3,4,33'
//     1002 gives the following instructions:
//         02 -> opcode 2 which is multiply instruction
//         0  -> position mode for parameter 1
//         1  -> immediate mode for parameter 2
//
//     This will result in position mode for first parameter, 4. Which points
//     to the value 33. The second parameter is in immediate mode which
//     gives the second parameter a value of 3.
//
//     The last value 4 is where we store the result (33*3=99), which is in
//     position 4. Executing the program will result as following:
//
//         '1002, 4, 3, 4, 99'
//
// FINALLY:
//     - Should take an input parameter
//     - Length of the instructions varies,depending on the instruction
//       format.

#[derive(Clone, Copy, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    pub fn from_num(mode: usize) -> Self {
        match mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unexpected parameter mode: {}", mode),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
    ChangeRelative,
    Exit,
}

impl OpCode {
    pub fn from_num(op_code: usize) -> Self {
        match op_code {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equal,
            9 => OpCode::ChangeRelative,
            99 => OpCode::Exit,
            _ => panic!("Unexpected op code: {}", op_code),
        }
    }
}

pub struct MemHandler {
    raw: Vec<i64>,
    op: usize,
    relative_base: i64,
    result: Vec<i64>,
    input: Vec<i64>,
    finished: bool,
}

impl MemHandler {
    pub fn new(raw: Vec<i64>) -> Self {
        return MemHandler {
            raw,
            op: 0,
            relative_base: 0,
            result: vec![],
            input: vec![],
            finished: false,
        };
    }

    fn next(&mut self) -> (OpCode, [ParamMode; 3]) {
        let next = (self.get(self.op) % 100) as usize;
        let op_code = OpCode::from_num(next);

        let mut next_modes = (self.get(self.op) / 100) as usize;
        let mut modes = [ParamMode::Position; 3];

        for mode in modes.iter_mut() {
            *mode = ParamMode::from_num(next_modes % 10);
            next_modes /= 10;
        }

        return (op_code, modes);
    }

    fn get(&mut self, address: usize) -> i64 {
        if address >= self.raw.len() {
            self.allocate(address);
        }

        self.raw[address]
    }

    pub fn set(&mut self, address: usize, val: i64) {
        if address >= self.raw.len() {
            self.allocate(address);
        }

        self.raw[address] = val;
    }

    pub fn move_op(&mut self, to: usize) {
        self.op = to;
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.insert(0, input);
    }

    pub fn get_result(&self) -> Option<Vec<i64>> {
        if self.result.len() == 0 {
            None
        } else {
            Some(self.result.clone())
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
    pub fn move_op_by(&mut self, steps: usize) {
        self.op += steps;
    }

    // Dynamic allocate
    fn allocate(&mut self, max_size: usize) {
        while self.raw.len() - 1 < max_size {
            self.raw.push(0);
        }
    }

    fn get_next_position(&mut self, offset: usize, mode: ParamMode) -> usize {
        let address = self.get(self.op + offset);

        let position = match mode {
            ParamMode::Relative => address + self.relative_base,
            _ => address,
        };

        if position < 0 {
            panic!("Position less than 0!");
        }

        return position as usize;
    }

    //
    //  POSITION_MODE:  0
    //  IMMEDIATE_MODE: 1
    //  RELATIVE_MODE:  2
    //
    fn get_next_parameter(&mut self, offset: usize, mode: ParamMode) -> i64 {
        let address = self.get(self.op + offset);

        match mode {
            ParamMode::Position => self.get(address as usize),
            ParamMode::Relative => self.get((address + self.relative_base) as usize),
            ParamMode::Immediate => address,
        }
    }

    pub fn run(&mut self) -> Option<i64> {
        let (op_code, modes) = self.next();
        match op_code {
            OpCode::Add => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                let arg1 = self.get_next_parameter(2, modes[1]);
                let arg2 = self.get_next_position(3, modes[2]);
                self.set(arg2, arg0 + arg1);
                self.move_op_by(4);
            }
            OpCode::Multiply => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                let arg1 = self.get_next_parameter(2, modes[1]);
                let arg2 = self.get_next_position(3, modes[2]);
                self.set(arg2, arg0 * arg1);
                self.move_op_by(4);
            }
            OpCode::Input => {
                let arg0 = self.get_next_position(1, modes[0]);
                if let Some(val) = self.input.pop() {
                    self.set(arg0, val);
                    self.move_op_by(2);
                } else {
                    eprintln!("Missing input");
                }
            }
            OpCode::Output => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                self.result.push(arg0);
                self.move_op_by(2);
                return Some(arg0);
            }
            OpCode::JumpIfTrue => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                if arg0 != 0 {
                    let arg1 = self.get_next_parameter(2, modes[1]) as usize;
                    self.move_op(arg1);
                } else {
                    self.move_op_by(3);
                }
            }
            OpCode::JumpIfFalse => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                if arg0 == 0 {
                    let arg1 = self.get_next_parameter(2, modes[1]) as usize;
                    self.move_op(arg1);
                } else {
                    self.move_op_by(3);
                }
            }
            OpCode::LessThan => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                let arg1 = self.get_next_parameter(2, modes[1]);
                let arg3 = self.get_next_position(3, modes[2]);
                let val = if arg0 < arg1 { 1 } else { 0 };
                self.set(arg3, val);
                self.move_op_by(4);
            }
            OpCode::Equal => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                let arg1 = self.get_next_parameter(2, modes[1]);
                let arg2 = self.get_next_position(3, modes[2]);
                let val = if arg0 == arg1 { 1 } else { 0 };
                self.set(arg2, val);
                self.move_op_by(4);
            }
            OpCode::ChangeRelative => {
                let arg0 = self.get_next_parameter(1, modes[0]);
                self.relative_base += arg0;
                self.move_op_by(2);
            }
            OpCode::Exit => self.finished = true,
        }

        return None;
    }
}
