struct Cpu {
    pc: u16, // program counter
    sp: u16, // stack pointer
    a: u8, // a reg
    x: u8, // x reg
    y: u8, // y reg
    status: u8, // status flags
}

fn reset_cpu(cpu: &mut Cpu) {
    cpu.pc = 0xfffc;
    cpu.sp = 0x0100;
    cpu.a = 0;
    cpu.x = 0;
    cpu.y = 0;
    cpu.status = 0;
}

fn main() {
    let mut cpu = Cpu{pc:0, sp:0, a:0, x:0, y:0, status:0};

    println!("Initializing");
    reset_cpu(&mut cpu);

    println!("Done");
}
