#![no_std]
#![feature(llvm_asm)]

macro_rules! sysreg_read_func {
    ( $name: ident, $t: tt ) => {
        pub fn $name() -> u64 {
            let mut v: u64;
            unsafe {
                llvm_asm!(concat!("mrs $0, ", $t)
                    : "=r"(v)
                );
            }
            v
        }
    };
}

macro_rules! sysreg_write_func {
    ( $name: ident, $t: tt) => {
        pub fn $name(v: u64) {
            unsafe {
                llvm_asm!(concat!("msr ", $t, ", $0")
                    :
                    : "r"(v));
            }
        }
    }
}

macro_rules! sysreg_rw_func {
    ( $read_name: ident, $write_name: ident, $t: tt) => {
        sysreg_read_func!($read_name, $t);
        sysreg_write_func!($write_name, $t);
    }
}

sysreg_rw_func!(read_cntfrq_el0, write_cntfrq_el0, "cntfrq_el0");
sysreg_read_func!(read_cntpct_el0,  "cntpct_el0");
