/// Program Counter (program_counter) :
/// holds the address for the next machine language instruction to be executed.
///
/// Stack Pointer (stack_ptr):
/// Memory space [0x0100 .. 0x1FF] is used for stack. The stack pointer holds the address of the top of that space. NES Stack (as all stacks) grows from top to bottom: when a byte gets pushed to the stack, SP register decrements. When a byte is retrieved from the stack, SP register increments.
///
/// Accumulator (accumulator):
/// stores the results of arithmetic, logic, and memory access operations. It used as an input parameter for some operations.
///
/// Index Register X (register_x):
/// used as an offset in specific memory addressing modes (more on this later). Can be used for auxiliary storage needs (holding temp values, being used as a counter, etc.)
///
/// Index Register Y (register_y):
/// similar use cases as register X.
///
/// Processor status (status):
/// 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)
///
/// 6502 cpu instructions book: http://49.212.183.201/6502/6502_report.htm

use std::borrow::Borrow;
use std::num::FpCategory::Zero;
use std::ops::Add;
use crate::adressing_modes::AddrMode;
use crate::ops_codes::*;

pub struct CPU {
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_ptr: u8,
    pub status: u8,
    pub program_counter: u16,
    pub memory: [u8; 0xFFFF],
}

pub static NEGATIVE: u8 = 0b1000_0000;
pub static OVERFLOW: u8 = 0b0100_0000;
pub static BREAK: u8 = 0b0001_0000;
pub static DECIMAL: u8 = 0b0000_1000;
pub static INTERRUPT: u8 = 0b0000_0100;
pub static ZERO: u8 = 0b0000_0010;
pub static CARRY: u8 = 0b0000_0001;

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            stack_ptr: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }
    fn set_status_carry(&mut self) {
        self.status |= CARRY;
    }
    fn clear_status_carry(&mut self) {
        self.status &= !CARRY;
    }
    fn set_status_zero(&mut self) {
        self.status |= ZERO;
    }
    fn clear_status_zero(&mut self) {
        self.status &= !ZERO;
    }
    fn set_status_interrupt(&mut self) {
        self.status |= INTERRUPT;
    }
    fn clear_status_interrupt(&mut self) {
        self.status &= !INTERRUPT;
    }
    fn set_status_deciaml(&mut self) {
        self.status |= DECIMAL;
    }
    fn clear_status_decimal(&mut self) {
        self.status &= !DECIMAL;
    }
    fn set_status_break(&mut self) {
        self.status |= BREAK;
    }
    fn clear_status_break(&mut self) {
        self.status &= !BREAK;
    }
    fn set_status_overflow(&mut self) {
        self.status |= OVERFLOW;
    }
    fn clear_status_overflow(&mut self) {
        self.status &= !OVERFLOW;
    }
    fn set_status_negative(&mut self) {
        self.status |= NEGATIVE;
    }
    fn clear_status_negative(&mut self) {
        self.status &= !NEGATIVE;
    }
    fn memory_read(&self, pos: u16) -> u8 {
        self.memory[pos as usize]
    }
    fn memory_read_u16(&self, pos: u16) -> u16 {
        u16::from_ne_bytes([self.memory[pos as usize], self.memory[(pos+1) as usize]])
    }
    fn memory_write(&mut self,pos: u16,data: u8) {
        self.memory[pos as usize] = data;
    }
    fn memory_write_u16(&mut self,pos: u16,data: u16) {
        let low = (data & 0xff) as u8;
        let high = (data >> 8) as u8;
        self.memory_write(pos,low);
        self.memory_write(pos+1,high);
    }
    fn load_program(&mut self,program: Vec<u8>) {
        self.memory[0x8000..(0x8000+program.len())].copy_from_slice(&program);
        self.memory_write_u16(0xFFFC,0x8000);
    }
    fn reset(&mut self) {
        self.status = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.accumulator = 0;
        self.stack_ptr = 0;

        self.program_counter = self.memory_read_u16(0xFFFC);
    }
    fn calc_token(&mut self,result: u8) {
        // Token:Z -> This bit is set when the 7th binary bit of ops_code is 0. Otherwise it will be cleared.
        if result == 0 {
            self.set_status_zero();
        }else {
            self.clear_status_zero();
        }
        // Token:N -> When the high bit of the ops_code is set (negative), this bit is set, otherwise it is cleared.
        if result & 0b1000_0000 != 0 {
            self.set_status_negative();
        }else {
            self.clear_status_negative();
        }
    }
    fn get_operand_addr(&mut self,mode: &AddrMode) -> u16 {
        match mode {
            AddrMode::Immediate => self.program_counter,
            AddrMode::Absolute => self.memory_read_u16(self.program_counter),
            AddrMode::AbsoluteX => {
                let base = self.memory_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            },
            AddrMode::AbsoluteY => {
                let base = self.memory_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            },
            AddrMode::Indirect => {
                let base = self.memory_read(self.program_counter);
                let ptr = base.wrapping_add(self.register_x) as u16;
                self.memory_read_u16(ptr)
            },
            AddrMode::IndirectX => {
                let base = self.memory_read(self.program_counter);
                let ptr = base.wrapping_add(self.register_x) as u16;
                self.memory_read_u16(ptr)
            },
            AddrMode::IndirectY => {
                let base = self.memory_read(self.program_counter);
                let addr = base.wrapping_add(base) as u16;
                addr.wrapping_add(self.register_y as u16)
            },
            AddrMode::ZeroPage => self.memory_read(self.program_counter) as u16,
            AddrMode::ZeroPageX => self.memory_read(self.program_counter).wrapping_add(self.register_x) as u16,
            AddrMode::ZeroPageY => self.memory_read(self.program_counter).wrapping_add(self.register_y) as u16,
            AddrMode::Accumulator => {
                panic!("mode accumulator is not supported");
            },
            AddrMode::Implied => {
                panic!("mode implied is not supported");
            },
            AddrMode::Relative => {
                panic!("mode relative is not supported");
            },
        }
    }
    fn set_accumulator(&mut self,data: u8) {
        self.accumulator = data;
        self.calc_token(self.accumulator);
    }
    fn add_to_accumulator(&mut self,data: u8) {
        let carry_out = if self.status & CARRY != 0 {
            1
        }else {
            0
        };
        let sum = self.accumulator as u16 + data as u16 + carry_out;
        if sum > 255 {
            self.set_status_carry();
        }else {
            self.clear_status_carry();
        }

        let sum = sum as u8;
        if (sum ^ data) & (sum ^ self.accumulator) & 0b1000_0000 != 0 {
            self.set_status_overflow();
        }else {
            self.clear_status_overflow();
        }
        self.set_accumulator(data);
    }
    // branch specifies the target of conditional transfer.
    // The second byte of the instruction becomes an operand
    // and is added as an offset to the instruction pointer to the next instruction.
    fn branch(&mut self) {
        let offset = self.memory_read(self.program_counter) as i8;
        let next = self.program_counter.wrapping_add(1).wrapping_add(offset as u16);
        self.program_counter = next;
    }
    fn compare(&mut self,register: u8,mode: &AddrMode) {
        let addr = self.get_operand_addr(mode);
        let data = self.memory_read(addr);
        if data <= register {
            self.set_status_carry();
        }else {
            self.clear_status_carry();
        }
        self.calc_token(register.wrapping_sub(data));
    }
    fn adc(&mut self,mode: &AddrMode) {
        let addr = self.get_operand_addr(mode);
        let data = self.memory_read(addr);
        self.add_to_accumulator(data);
    }
    fn and(&mut self,mode: &AddrMode) {
        let addr = self.get_operand_addr(mode);
        let data = self.memory_read(addr);
        self.set_accumulator(self.accumulator & data);
    }
    fn asl(&mut self,mode: &AddrMode) {
        match mode {
            AddrMode::Accumulator => {
                if self.accumulator >> 7 & 1 != 0 {
                    self.set_status_carry();
                }else {
                    self.clear_status_carry();
                }
                self.set_accumulator(self.accumulator << 1)
            },
            _ => {
                let addr = self.get_operand_addr(mode);
                let data = self.memory_read(addr);
                if data >> 7 & 1 != 0 {
                    self.set_status_carry();
                }else {
                    self.clear_status_carry();
                }
                self.memory_write(addr,data);
                self.calc_token(data);
            }
        }
    }
    fn bcc(&mut self) {
        if self.status & 1 == 0 {
            self.branch();
        }
    }
    fn bcs(&mut self) {
        if self.status & 1 != 0 {
            self.branch();
        }
    }
    fn beq(&mut self) {
        if self.status >> 1 & 1 != 0 {
            self.branch()
        }
    }
    fn bne(&mut self) {
        if self.status >> 1 & 1 == 0 {
            self.branch();
        }
    }
    fn bit(&mut self,mode: &AddrMode) {
        let addr = self.get_operand_addr(mode);
        let data = self.memory_read(addr);
        if data >> 7 & 1 != 0 {
            self.set_status_negative();
        }else {
            self.clear_status_negative();
        }
        if data >> 6 & 1 != 0 {
            self.set_status_overflow();
        }else {
            self.clear_status_overflow();
        }
        if !(self.accumulator & data) == 0 {
            self.clear_status_zero();
        }else {
            self.set_status_zero();
        }
    }
    fn bmi(&mut self) {
        if self.status >> 7 & 1 != 0 {
            self.branch();
        }
    }
    fn bpl(&mut self) {
        if self.status >> 7 & 1 == 0 {
            self.branch();
        }
    }
    fn bvc(&mut self) {
        if self.status >> 6 & 1 == 0 {
            self.branch();
        }
    }
    fn bvs(&mut self) {
        if self.status >> 6 & 1 != 0 {
            self.branch();
        }
    }
    fn clc(&mut self) {
        self.clear_status_carry();
    }
    fn cld(&mut self) {
        self.clear_status_decimal();
    }
    fn cli(&mut self) {
        self.clear_status_interrupt();
    }
    fn clv(&mut self) {
        self.clear_status_overflow();
    }
    fn lda(&mut self,mode: &AddrMode) {
        let addr = self.get_operand_addr(mode);
        let data = self.memory_read(addr);
        self.set_accumulator(data);
    }
    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.calc_token(self.register_x);
    }
    fn tay(&mut self) {
        self.register_y = self.accumulator;
        self.calc_token(self.register_y);
    }
    fn inx(&mut self) {
        if self.register_x >= 255 {
            self.register_x = 0
        }else {
            self.register_x += 1
        }
        self.calc_token(self.register_x);
    }
    fn iny(&mut self) {
        if self.register_y >= 255 {
            self.register_y = 0
        }else {
            self.register_y += 1
        }
        self.calc_token(self.register_y);
    }
    fn sta(&mut self,mode: &AddrMode) {
        let addr = self.get_operand_addr(mode);
        self.memory_write(addr,self.accumulator);
    }
    pub fn run(&mut self) {
        loop {
            let ops_addr = self.memory_read(self.program_counter);
            self.program_counter += 1;
            let ops_code = OpCodesMap.get(&ops_addr).unwrap();
            let mode = ops_code.addressing_mode.borrow();
            match ops_code.opc {
                // LDA: load data into accumulator
                0xA9 | 0xA5 |0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1  => {
                    self.lda(mode);
                },
                // STA: Store Accumulator in Memory
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(mode);
                }
                // BRK: force break
                0x00 => return,
                // transfer accumulator to register_x index
                0xAA => self.tax(),
                // increment register_x index
                0xE8 => self.inx(),
                _ => todo!()
            }

            self.program_counter += (ops_code.bytes-1) as u16;
        }
    }
    fn load_and_run(&mut self,program: Vec<u8>) {
        self.load_program(program);
        self.reset();
        self.run();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 0xC1)
    }
}