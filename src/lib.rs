#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(extended_key_value_attributes)]
#![allow(non_snake_case)]
#![allow(unused_macros)]

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
    ( $fname: ident ) => {
        #[doc=concat!("Call ", stringify!($fname))]
        pub fn $fname() {
            unsafe {
                asm!(stringify!($fname));
            }
        }
    };
}

/// Define function for system instruction with type specifier
macro_rules! sysop_type_func {
    ( $fname: ident, $op: tt, $op_type: tt, $desc: tt) => {
        #[doc = concat!("Call ", $op, " with type ", $op_type, "

        ", $desc)]
        pub fn $fname() {
            unsafe {
                asm!(concat!($op, " ", $op_type));
            }
        }
    }
}

/// Define function for system instruction with register parameter
macro_rules! sysop_type_param_func {
    ( $fname: ident, $op: tt, $op_type: tt) => {
        #[doc=concat!("Call ", $op, " with type ", $op_type)]
        pub fn $fname(v: u64) {
            unsafe {
                llvm_asm!(concat!($op, " ", $op_type, ", $0")
                    :
                    : "r"(v));
            }
        }
    };
}

/// Write constant in range of 0..16 to system register
macro_rules! sysreg_write_const {
    ( $name: ident, $reg_name: tt) => {
        macro_rules! $name {
            ($val: expr) => {
                unsafe {
                    llvm_asm!(concat!("msr ", $reg_name, ", $0")
                        :
                        : "r"($val));
                }
            }
        }
    }
}

sysop_type_func!(tlbi_alle1, "tlbi", "alle1", "Invalidate all stage 1 translations used at EL1");
sysop_type_func!(tlbi_alle1is, "tlbi", "alle1is", "Invalidate all stage 1 translations used at EL1, Inner Shareable");
sysop_type_func!(tlbi_alle2, "tlbi", "alle2", "Invalidate all stage 1 translations used at EL2");
sysop_type_func!(tlbi_alle2is, "tlbi", "alle2is", "Invalidate all stage 1 translations used at EL2, Inner Shareable");
#[cfg(not(feature = "errata_a57_813419"))]
sysop_type_func!(tlbi_alle3, "tlbi", "alle3", "Invalidate all stage 1 translations used at EL3");
#[cfg(not(feature = "errata_a57_813419"))]
sysop_type_func!(tlbi_alle3is, "tlbi", "alle3is", "Invalidate all stage 1 translations used at EL3, Inner Shareable");
sysop_type_func!(tlbi_vmalle1, "tlbi", "vmalle1", "Invalidate all stage 1 translations used at EL1 with the current VMID");
sysop_type_param_func!(tlbi_vaae1is, "tlbi", "vaae1is");
sysop_type_param_func!(tlbi_vaale1is, "tlbi", "vaale1is");
sysop_type_param_func!(tlbi_vae2is, "tlbi", "vae2is");
sysop_type_param_func!(tlbi_vale2is, "tlbi", "vale2is");
#[cfg(not(feature = "errata_a57_813419"))]
sysop_type_param_func!(tlbi_vae3is, "tlbi", "vae3is");
#[cfg(not(feature = "errata_a57_813419"))]
sysop_type_param_func!(tlbi_vale3is, "tlbi", "vale3is");

// Cache maintenance accessor prototypes
sysop_type_param_func!(dc_isw, "dc", "isw");
sysop_type_param_func!(dc_cisw, "dc", "cisw");
sysop_type_param_func!(dc_csw, "dc", "csw");
sysop_type_param_func!(dc_cvac, "dc", "cvac");
sysop_type_param_func!(dc_ivac, "dc", "ivac");
sysop_type_param_func!(dc_civac, "dc", "civac");
sysop_type_param_func!(dc_cvau, "dc", "cvau");
sysop_type_param_func!(dc_zva, "dc", "zva");

// Address translation accessor prototypes
sysop_type_param_func!(at_s12e1r, "at", "s12e1r");
sysop_type_param_func!(at_s12e1w, "at", "s12e1w");
sysop_type_param_func!(at_s12e0r, "at", "s12e0r");
sysop_type_param_func!(at_s12e0w, "at", "s12e0w");
sysop_type_param_func!(at_s1e2r, "at", "s1e2r");

// Misc accessor prototype
#[macro_export]
sysreg_write_const!(write_daifclr, "daifclr");
#[macro_export]
sysreg_write_const!(write_daifset, "daifset");
sysreg_read_func!(read_par_el1, "par_el1");
sysreg_read_func!(read_id_pfr1_el1, "id_pfr1_el1");
sysreg_read_func!(read_id_aa64pfr0_el1, "id_aa64pfr0_el1");
sysreg_read_func!(read_id_aa64dfr0_el1, "id_aa64dfr0_el1");
sysreg_read_func!(read_CurrentEl, "CurrentEl");
sysreg_rw_func!(read_daif, write_daif, "daif");
sysreg_rw_func!(read_spsr_el1, write_spsr_el1, "spsr_el1");
sysreg_rw_func!(read_spsr_el2, write_spsr_el2, "spsr_el2");
sysreg_rw_func!(read_spsr_el3, write_spsr_el3, "spsr_el3");
sysreg_rw_func!(read_elr_el1, write_elr_el1, "elr_el1");
sysreg_rw_func!(read_elr_el2, write_elr_el2, "elr_el2");
sysreg_rw_func!(read_elr_el3, write_elr_el3, "elr_el3");

sysop_func!(wfi);
sysop_func!(wfe);
sysop_func!(sev);

sysop_type_func!(dsb_sy, "dsb", "sy", "Full system DSB operation");
sysop_type_func!(dmb_sy, "dmb", "sy", "Full system DMB operation");
sysop_type_func!(dmb_st, "dmb", "st", "DMB operation that waits only for stores to complete");
sysop_type_func!(dmb_ld, "dmb", "ld", "DMB operation that waits only for loads to complete");
sysop_type_func!(dsb_ish, "dsb", "ish", "DSB operation only to the inner shareable domain");
sysop_type_func!(dsb_nsh, "dsb", "nsh", "DSB operation only out to the point of unification");
sysop_type_func!(dsb_ishst, "dsb", "ishst", "DSB operation that waits only for stores to complete, and only to the inner shareable domain");
sysop_type_func!(dmb_ish, "dmb", "ish", "DMB operation only to the inner shareable domain");
sysop_type_func!(dmb_ishst, "dmb", "ishst", "DMB operation that waits only for stores to complete, and only to the inner shareable domain");

sysop_func!(isb);

// System register accessor prototypes
sysreg_read_func!(read_midr_el1, "midr_el1");
sysreg_read_func!(read_mpidr_el1, "mpidr_el1");
sysreg_read_func!(read_id_aa64mmfr0_el1, "id_aa64mmfr0_el1");

sysreg_rw_func!(read_scr_el3, write_scr_el3, "scr_el3");
sysreg_rw_func!(read_hcr_el2, write_hcr_el2, "hcr_el2");

sysreg_rw_func!(read_vbar_el1, write_vbar_el1, "vbar_el1");
sysreg_rw_func!(read_vbar_el2, write_vbar_el2, "vbar_el2");
sysreg_rw_func!(read_vbar_el3, write_vbar_el3, "vbar_el3");

sysreg_rw_func!(read_sctlr_el1, write_sctlr_el1, "sctlr_el1");
sysreg_rw_func!(read_sctlr_el2, write_sctlr_el2, "sctlr_el2");
sysreg_rw_func!(read_sctlr_el3, write_sctlr_el3, "sctlr_el3");

sysreg_rw_func!(read_actlr_el1, write_actlr_el1, "actlr_el1");
sysreg_rw_func!(read_actlr_el2, write_actlr_el2, "actlr_el2");
sysreg_rw_func!(read_actlr_el3, write_actlr_el3, "actlr_el3");

sysreg_rw_func!(read_esr_el1, write_esr_el1, "esr_el1");
sysreg_rw_func!(read_esr_el2, write_esr_el2, "esr_el2");
sysreg_rw_func!(read_esr_el3, write_esr_el3, "esr_el3");

sysreg_rw_func!(read_afsr0_el1, write_afsr0_el1, "afsr0_el1");
sysreg_rw_func!(read_afsr0_el2, write_afsr0_el2, "afsr0_el2");
sysreg_rw_func!(read_afsr0_el3, write_afsr0_el3, "afsr0_el3");

sysreg_rw_func!(read_afsr1_el1, write_afsr1_el1, "afsr1_el1");
sysreg_rw_func!(read_afsr1_el2, write_afsr1_el2, "afsr1_el2");
sysreg_rw_func!(read_afsr1_el3, write_afsr1_el3, "afsr1_el3");

sysreg_rw_func!(read_far_el1, write_far_el1, "far_el1");
sysreg_rw_func!(read_far_el2, write_far_el2, "far_el2");
sysreg_rw_func!(read_far_el3, write_far_el3, "far_el3");

sysreg_rw_func!(read_mair_el1, write_mair_el1, "mair_el1");
sysreg_rw_func!(read_mair_el2, write_mair_el2, "mair_el2");
sysreg_rw_func!(read_mair_el3, write_mair_el3, "mair_el3");

sysreg_rw_func!(read_amair_el1, write_amair_el1, "amair_el1");
sysreg_rw_func!(read_amair_el2, write_amair_el2, "amair_el2");
sysreg_rw_func!(read_amair_el3, write_amair_el3, "amair_el3");

sysreg_read_func!(read_rvbar_el1, "rvbar_el1");
sysreg_read_func!(read_rvbar_el2, "rvbar_el2");
sysreg_read_func!(read_rvbar_el3, "rvbar_el3");

sysreg_rw_func!(read_rmr_el1, write_rmr_el1, "rmr_el1");
sysreg_rw_func!(read_rmr_el2, write_rmr_el2, "rmr_el2");
sysreg_rw_func!(read_rmr_el3, write_rmr_el3, "rmr_el3");

sysreg_rw_func!(read_tcr_el1, write_tcr_el1, "tcr_el1");
sysreg_rw_func!(read_tcr_el2, write_tcr_el2, "tcr_el2");
sysreg_rw_func!(read_tcr_el3, write_tcr_el3, "tcr_el3");

sysreg_rw_func!(read_ttbr0_el1, write_ttbr0_el1, "ttbr0_el1");
sysreg_rw_func!(read_ttbr0_el2, write_ttbr0_el2, "ttbr0_el2");
sysreg_rw_func!(read_ttbr0_el3, write_ttbr0_el3, "ttbr0_el3");

sysreg_rw_func!(read_ttbr1_el1, write_ttbr1_el1, "ttbr1_el1");

sysreg_rw_func!(read_vttbr_el2, write_vttbr_el2, "vttbr_el2");

sysreg_rw_func!(read_cptr_el2, write_cptr_el2, "cptr_el2");
sysreg_rw_func!(read_cptr_el3, write_cptr_el3, "cptr_el3");

sysreg_rw_func!(read_cpacr_el1, write_cpacr_el1, "cpacr_el1");
sysreg_rw_func!(read_cntfrq_el0, write_cntfrq_el0, "cntfrq_el0");
sysreg_rw_func!(read_cntps_ctl_el1, write_cntps_ctl_el1, "cntps_ctl_el1");
sysreg_rw_func!(read_cntps_tval_el1, write_cntps_tval_el1, "cntps_tval_el1");
sysreg_rw_func!(read_cntps_cval_el1, write_cntps_cval_el1, "cntps_cval_el1");
sysreg_read_func!(read_cntpct_el0,  "cntpct_el0");
sysreg_rw_func!(read_cnthctl_el2, write_cnthctl_el2, "cnthctl_el2");

sysreg_rw_func!(read_tpidr_el3, write_tpidr_el3, "tpidr_el3");

sysreg_rw_func!(read_cntvoff_el2, write_cntvoff_el2, "cntvoff_el2");

sysreg_rw_func!(read_vpidr_el2, write_vpidr_el2, "vpidr_el2");
sysreg_rw_func!(read_vmpidr_el2, write_vmpidr_el2, "vmpidr_el2");
sysreg_rw_func!(read_cntp_ctl_el0, write_cntp_ctl_el0, "cntp_ctl_el0");

sysreg_read_func!(read_isr_el1,  "isr_el1");

sysreg_read_func!(read_ctr_el0,  "ctr_el0");

sysreg_rw_func!(read_mdcr_el2, write_mdcr_el2, "mdcr_el2");
sysreg_rw_func!(read_mdcr_el3, write_mdcr_el3, "mdcr_el3");
sysreg_rw_func!(read_hstr_el2, write_hstr_el2, "hstr_el2");
sysreg_rw_func!(read_cnthp_ctl_el2, write_cnthp_ctl_el2, "cnthp_ctl_el2");
sysreg_rw_func!(read_pmcr_el0, write_pmcr_el0, "pmcr_el0");

//TODO: dcache functions, dcsw functions, mmu disabling functions, get_afflvl_shift, mpidr_mask_lower_afflvls, eret, smc