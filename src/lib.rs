#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(extended_key_value_attributes)]
#![allow(non_snake_case)]

macro_rules! sysreg_read_func {
    ( $name: ident, $reg_name: tt ) => {
        #[doc=concat!("Read value from register ", $reg_name)]
        pub fn $name() -> u64 {
            let mut v: u64;
            unsafe {
                llvm_asm!(concat!("mrs $0, ", $reg_name)
                    : "=r"(v)
                );
            }
            v
        }
    };
}

macro_rules! sysreg_write_func {
    ( $name: ident, $reg_name: tt) => {
        #[doc=concat!("Write value to register ", $reg_name)]
        pub fn $name(v: u64) {
            unsafe {
                llvm_asm!(concat!("msr ", $reg_name, ", $0")
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

/// Define function for simple system instruction
macro_rules! sysop_func {
    ( $fname: ident, $op: tt ) => {
        #[doc=concat!("Call ", $op)]
        pub fn $fname() {
            unsafe {
                asm!($op);
            }
        }
    };
}

/// Define function for system instruction with type specifier
macro_rules! sysop_type_func {
    ( $fname: ident, $op: tt, $op_type: tt) => {
        #[doc=concat!("Call ", $op, "with type ", $op_type)]
        pub fn $fname() {
            unsafe {
                asm!(concat!($op, " ", $op_type));
            }
        }
    };
}

/// Define function for system instruction with register parameter
macro_rules! sysop_type_param_func {
    ( $fname: ident, $op: tt, $op_type: tt) => {
        #[doc=concat!("Call ", $op, "with type ", $op_type)]
        pub fn $fname(v: u64) {
            unsafe {
                llvm_asm!(concat!($op, " ", $op_type, ", $0")
                    :
                    : "r"(v));
            }
        }
    };
}

sysreg_rw_func!(read_cntfrq_el0, write_cntfrq_el0, "cntfrq_el0");
sysreg_read_func!(read_cntpct_el0,  "cntpct_el0");