const MEM_SIZE: usize = 1024 * 64;

// JSR
const INSTR_JSR: u8 = 0x20;

// LDA
const INSTR_LDA_IM: u8 = 0xa9;
const INSTR_LDA_ZERO_PAGE: u8 = 0xa5;
const INSTR_LDA_ZERO_PAGE_X: u8 = 0xb5;
const INSTR_LDA_ABSOLUTE: u8 = 0xad;
const INSTR_LDA_ABSOLUTE_X: u8 = 0xbd;
const INSTR_LDA_ABSOLUTE_Y: u8 = 0xb9;

const STATUS_FLAG_N: u8 = 0x80;
const STATUS_FLAG_V: u8 = 0x40;
const STATUS_FLAG_B: u8 = 0x10;
const STATUS_FLAG_D: u8 = 0x08;
const STATUS_FLAG_I: u8 = 0x04;
const STATUS_FLAG_Z: u8 = 0x02;
const STATUS_FLAG_C: u8 = 0x01;

struct Mem {
    data: [u8; MEM_SIZE],
}

struct Cpu {
    pc: u16, // program counter
    sp: u16, // stack pointer
    a: u8, // a reg
    x: u8, // x reg
    y: u8, // y reg
    status: u8, // status flags
    ticks: usize // number of cpu ticks
}

fn reset(cpu: &mut Cpu, mem: &mut Mem) {
    cpu.pc = 0xfffc;
    cpu.sp = 0x0100;
    cpu.a = 0;
    cpu.x = 0;
    cpu.y = 0;
    cpu.status = 0;
    cpu.ticks = 0;
    mem.data = [0; MEM_SIZE];
}

fn set_status_flag(cpu: &mut Cpu, flag: u8) {
    cpu.status = cpu.status | flag;
}

fn clear_status_flag(cpu: &mut Cpu, flag: u8) {
    cpu.status = cpu.status & !flag;
}

fn toggle_status_flag(cpu: &mut Cpu, flag: u8, condition: bool) {
    if condition {
        set_status_flag(cpu, flag);
    }
    else {
        clear_status_flag(cpu, flag);
    }
}

fn fetch_next_byte(cpu: &mut Cpu, mem: &mut Mem) -> u8 {
    let data = mem.data[cpu.pc as usize];
    cpu.pc = cpu.pc + 1;
    cpu.ticks = cpu.ticks + 1;
    data
}

fn fetch_next_word(cpu: &mut Cpu, mem: &mut Mem) -> u16 {
    let b1 = fetch_next_byte(cpu, mem);
    let b2 = fetch_next_byte(cpu, mem);
    let data = (b1 as u16) | (b2 as u16) << 8;
    data
}

fn read_byte(cpu: &mut Cpu, mem: &mut Mem, addr: u8) -> u8 {
    let data = mem.data[addr as usize];
    cpu.ticks = cpu.ticks + 1;
    data
}

fn write_byte(cpu: &mut Cpu, mem: &mut Mem, addr: u16, data: u8) {
    mem.data[addr as usize] = data;
    cpu.ticks = cpu.ticks + 1;
}

fn write_word(cpu: &mut Cpu, mem: &mut Mem, addr: u16, data: u16) {
    mem.data[addr as usize] = (data & 0xff) as u8;
    mem.data[(addr + 1) as usize] = ((data >> 8) & 0xff) as u8;
    cpu.ticks = cpu.ticks + 2;
}

fn push_byte(cpu: &mut Cpu, mem: &mut Mem, data: u8) {
    write_byte(cpu, mem, cpu.sp, data);
    cpu.sp = cpu.sp + 1;
    cpu.ticks = cpu.ticks + 1;
}

fn push_word(cpu: &mut Cpu, mem: &mut Mem, data: u16) {
    write_word(cpu, mem, cpu.sp, data);
    cpu.sp = cpu.sp + 2;
    cpu.ticks = cpu.ticks + 1;
}

fn do_jsr(cpu: &mut Cpu, mem: &mut Mem) {
    let addr = fetch_next_word(cpu , mem);
    push_word(cpu, mem, cpu.pc - 1);
    cpu.pc = addr;
    cpu.ticks = cpu.ticks + 1;
}

fn lda_set_status(cpu: &mut Cpu) {
    toggle_status_flag(cpu, STATUS_FLAG_Z, cpu.a == 0);
    toggle_status_flag(cpu, STATUS_FLAG_N, cpu.a & 0x80 == 1);
    cpu.ticks = cpu.ticks + 1;
}

fn do_lda_im(cpu: &mut Cpu, mem: &mut Mem) {
    cpu.a = fetch_next_byte(cpu, mem);
    lda_set_status(cpu);
}

fn do_lda_zero_page(cpu: &mut Cpu, mem: &mut Mem) {
    let addr = fetch_next_byte(cpu, mem);
    cpu.a = read_byte(cpu, mem, addr);
    lda_set_status(cpu);
}

fn do_lda_zero_page_x(cpu: &mut Cpu, mem: &mut Mem) {
    let addr = fetch_next_byte(cpu, mem);
    cpu.x = cpu.x + addr;
    cpu.ticks = cpu.ticks + 1;
    cpu.a = read_byte(cpu, mem, addr);
    lda_set_status(cpu);
}

fn execute(cpu: &mut Cpu, mem: &mut Mem) {
    let mut cycle_counter = 0;

    loop {
        // Get the next instruction.
        let instr = fetch_next_byte(cpu, mem);

        // Execute the next instruction.
        match instr {

            // JSR
            INSTR_JSR => do_jsr(cpu, mem),

            // LDA
            INSTR_LDA_IM => do_lda_im(cpu, mem),
            INSTR_LDA_ZERO_PAGE => do_lda_zero_page(cpu, mem),
            INSTR_LDA_ZERO_PAGE_X => do_lda_zero_page_x(cpu, mem),
            INSTR_LDA_ABSOLUTE => {},
            INSTR_LDA_ABSOLUTE_X => {},
            INSTR_LDA_ABSOLUTE_Y => {},

            // LDX

            // LDY

            // LSR

            // Unhandled
            _ => println!("Instruction not handled."),
        }

        // Decrement the cycle counter.
        cycle_counter = cycle_counter - (1 as u32);
    }
}

fn main() {
    let mut cpu = Cpu{pc:0, sp:0, a:0, x:0, y:0, status:0, ticks:0};
    let mut mem = Mem{data:[0; MEM_SIZE]};

    println!("Initializing");
    reset(&mut cpu, &mut mem);

    println!("Executing");
    execute(&mut cpu, &mut mem);

    println!("Done");
}
