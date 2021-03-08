const MEM_SIZE: usize = 1024 * 64;

const INSTR_LDA_IM: u8 = 0xa9;

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

fn fetch_next_byte(cpu: &mut Cpu, mem: &mut Mem) -> u8 {
    let intr = mem.data[cpu.pc as usize];
    cpu.pc = cpu.pc + 1;
    intr
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

fn do_lda_im(cpu: &mut Cpu, mem: &mut Mem) {
    cpu.a = fetch_next_byte(cpu, mem);
    toggle_status_flag(cpu, STATUS_FLAG_Z, cpu.a == 0);
    toggle_status_flag(cpu, STATUS_FLAG_N, cpu.a & 0x80 == 1);
    cpu.ticks = cpu.ticks + 2;
}

fn execute(cpu: &mut Cpu, mem: &mut Mem) {
    let mut cycle_counter = 0;

    loop {
        // Get the next instruction.
        let instr = fetch_next_byte(cpu, mem);

        match instr {
            INSTR_LDA_IM => do_lda_im(cpu, mem),
            _ => {},
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
