const MEM_SIZE: usize = 1024 * 64;

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
}

fn reset(cpu: &mut Cpu, mem: &mut Mem) {
    cpu.pc = 0xfffc;
    cpu.sp = 0x0100;
    cpu.a = 0;
    cpu.x = 0;
    cpu.y = 0;
    cpu.status = 0;
    mem.data = [0; MEM_SIZE];
}

fn main() {
    let mut cpu = Cpu{pc:0, sp:0, a:0, x:0, y:0, status:0};
    let mut mem = Mem{data:[0; MEM_SIZE]};

    println!("Initializing");
    reset(&mut cpu, &mut mem);

    println!("Done");
}
