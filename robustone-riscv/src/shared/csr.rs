//! RISC-V CSR (Control and Status Register) lookup tables.
//!
//! Generated from Capstone's RISCVGenSystemOperands.inc.
//! Do not edit manually.

/// CSR descriptor: symbolic name and whether the CSR is only valid for RV32.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsrInfo {
    pub name: &'static str,
    pub rv32_only: bool,
}

/// Look up a CSR by its 12-bit address.
///
/// Returns None for unknown CSR addresses.
pub fn lookup_csr(addr: u16) -> Option<CsrInfo> {
    match addr {
        0x001 => Some(CsrInfo {
            name: "fflags",
            rv32_only: false,
        }),
        0x002 => Some(CsrInfo {
            name: "frm",
            rv32_only: false,
        }),
        0x003 => Some(CsrInfo {
            name: "fcsr",
            rv32_only: false,
        }),
        0x008 => Some(CsrInfo {
            name: "vstart",
            rv32_only: false,
        }),
        0x009 => Some(CsrInfo {
            name: "vxsat",
            rv32_only: false,
        }),
        0x00A => Some(CsrInfo {
            name: "vxrm",
            rv32_only: false,
        }),
        0x00F => Some(CsrInfo {
            name: "vcsr",
            rv32_only: false,
        }),
        0x015 => Some(CsrInfo {
            name: "seed",
            rv32_only: false,
        }),
        0x017 => Some(CsrInfo {
            name: "jvt",
            rv32_only: false,
        }),
        0x100 => Some(CsrInfo {
            name: "sstatus",
            rv32_only: false,
        }),
        0x104 => Some(CsrInfo {
            name: "sie",
            rv32_only: false,
        }),
        0x105 => Some(CsrInfo {
            name: "stvec",
            rv32_only: false,
        }),
        0x106 => Some(CsrInfo {
            name: "scounteren",
            rv32_only: false,
        }),
        0x10A => Some(CsrInfo {
            name: "senvcfg",
            rv32_only: false,
        }),
        0x10C => Some(CsrInfo {
            name: "sstateen0",
            rv32_only: false,
        }),
        0x10D => Some(CsrInfo {
            name: "sstateen1",
            rv32_only: false,
        }),
        0x10E => Some(CsrInfo {
            name: "sstateen2",
            rv32_only: false,
        }),
        0x10F => Some(CsrInfo {
            name: "sstateen3",
            rv32_only: false,
        }),
        0x114 => Some(CsrInfo {
            name: "sieh",
            rv32_only: true,
        }),
        0x140 => Some(CsrInfo {
            name: "sscratch",
            rv32_only: false,
        }),
        0x141 => Some(CsrInfo {
            name: "sepc",
            rv32_only: false,
        }),
        0x142 => Some(CsrInfo {
            name: "scause",
            rv32_only: false,
        }),
        0x143 => Some(CsrInfo {
            name: "stval",
            rv32_only: false,
        }),
        0x144 => Some(CsrInfo {
            name: "sip",
            rv32_only: false,
        }),
        0x14D => Some(CsrInfo {
            name: "stimecmp",
            rv32_only: false,
        }),
        0x150 => Some(CsrInfo {
            name: "siselect",
            rv32_only: false,
        }),
        0x151 => Some(CsrInfo {
            name: "sireg",
            rv32_only: false,
        }),
        0x154 => Some(CsrInfo {
            name: "siph",
            rv32_only: true,
        }),
        0x15C => Some(CsrInfo {
            name: "stopei",
            rv32_only: false,
        }),
        0x15D => Some(CsrInfo {
            name: "stimecmph",
            rv32_only: true,
        }),
        0x180 => Some(CsrInfo {
            name: "satp",
            rv32_only: false,
        }),
        0x200 => Some(CsrInfo {
            name: "vsstatus",
            rv32_only: false,
        }),
        0x204 => Some(CsrInfo {
            name: "vsie",
            rv32_only: false,
        }),
        0x205 => Some(CsrInfo {
            name: "vstvec",
            rv32_only: false,
        }),
        0x214 => Some(CsrInfo {
            name: "vsieh",
            rv32_only: true,
        }),
        0x240 => Some(CsrInfo {
            name: "vsscratch",
            rv32_only: false,
        }),
        0x241 => Some(CsrInfo {
            name: "vsepc",
            rv32_only: false,
        }),
        0x242 => Some(CsrInfo {
            name: "vscause",
            rv32_only: false,
        }),
        0x243 => Some(CsrInfo {
            name: "vstval",
            rv32_only: false,
        }),
        0x244 => Some(CsrInfo {
            name: "vsip",
            rv32_only: false,
        }),
        0x24D => Some(CsrInfo {
            name: "vstimecmp",
            rv32_only: false,
        }),
        0x250 => Some(CsrInfo {
            name: "vsiselect",
            rv32_only: false,
        }),
        0x251 => Some(CsrInfo {
            name: "vsireg",
            rv32_only: false,
        }),
        0x254 => Some(CsrInfo {
            name: "vsiph",
            rv32_only: true,
        }),
        0x25C => Some(CsrInfo {
            name: "vstopei",
            rv32_only: false,
        }),
        0x25D => Some(CsrInfo {
            name: "vstimecmph",
            rv32_only: true,
        }),
        0x280 => Some(CsrInfo {
            name: "vsatp",
            rv32_only: false,
        }),
        0x300 => Some(CsrInfo {
            name: "mstatus",
            rv32_only: false,
        }),
        0x301 => Some(CsrInfo {
            name: "misa",
            rv32_only: false,
        }),
        0x302 => Some(CsrInfo {
            name: "medeleg",
            rv32_only: false,
        }),
        0x303 => Some(CsrInfo {
            name: "mideleg",
            rv32_only: false,
        }),
        0x304 => Some(CsrInfo {
            name: "mie",
            rv32_only: false,
        }),
        0x305 => Some(CsrInfo {
            name: "mtvec",
            rv32_only: false,
        }),
        0x306 => Some(CsrInfo {
            name: "mcounteren",
            rv32_only: false,
        }),
        0x308 => Some(CsrInfo {
            name: "mvien",
            rv32_only: false,
        }),
        0x309 => Some(CsrInfo {
            name: "mvip",
            rv32_only: false,
        }),
        0x30A => Some(CsrInfo {
            name: "menvcfg",
            rv32_only: false,
        }),
        0x30C => Some(CsrInfo {
            name: "mstateen0",
            rv32_only: false,
        }),
        0x30D => Some(CsrInfo {
            name: "mstateen1",
            rv32_only: false,
        }),
        0x30E => Some(CsrInfo {
            name: "mstateen2",
            rv32_only: false,
        }),
        0x30F => Some(CsrInfo {
            name: "mstateen3",
            rv32_only: false,
        }),
        0x310 => Some(CsrInfo {
            name: "mstatush",
            rv32_only: true,
        }),
        0x313 => Some(CsrInfo {
            name: "midelegh",
            rv32_only: true,
        }),
        0x314 => Some(CsrInfo {
            name: "mieh",
            rv32_only: true,
        }),
        0x318 => Some(CsrInfo {
            name: "mvienh",
            rv32_only: true,
        }),
        0x319 => Some(CsrInfo {
            name: "mviph",
            rv32_only: true,
        }),
        0x31A => Some(CsrInfo {
            name: "menvcfgh",
            rv32_only: true,
        }),
        0x31C => Some(CsrInfo {
            name: "mstateen0h",
            rv32_only: true,
        }),
        0x31D => Some(CsrInfo {
            name: "mstateen1h",
            rv32_only: true,
        }),
        0x31E => Some(CsrInfo {
            name: "mstateen2h",
            rv32_only: true,
        }),
        0x31F => Some(CsrInfo {
            name: "mstateen3h",
            rv32_only: true,
        }),
        0x320 => Some(CsrInfo {
            name: "mcountinhibit",
            rv32_only: false,
        }),
        0x323 => Some(CsrInfo {
            name: "mhpmevent3",
            rv32_only: false,
        }),
        0x324 => Some(CsrInfo {
            name: "mhpmevent4",
            rv32_only: false,
        }),
        0x325 => Some(CsrInfo {
            name: "mhpmevent5",
            rv32_only: false,
        }),
        0x326 => Some(CsrInfo {
            name: "mhpmevent6",
            rv32_only: false,
        }),
        0x327 => Some(CsrInfo {
            name: "mhpmevent7",
            rv32_only: false,
        }),
        0x328 => Some(CsrInfo {
            name: "mhpmevent8",
            rv32_only: false,
        }),
        0x329 => Some(CsrInfo {
            name: "mhpmevent9",
            rv32_only: false,
        }),
        0x32A => Some(CsrInfo {
            name: "mhpmevent10",
            rv32_only: false,
        }),
        0x32B => Some(CsrInfo {
            name: "mhpmevent11",
            rv32_only: false,
        }),
        0x32C => Some(CsrInfo {
            name: "mhpmevent12",
            rv32_only: false,
        }),
        0x32D => Some(CsrInfo {
            name: "mhpmevent13",
            rv32_only: false,
        }),
        0x32E => Some(CsrInfo {
            name: "mhpmevent14",
            rv32_only: false,
        }),
        0x32F => Some(CsrInfo {
            name: "mhpmevent15",
            rv32_only: false,
        }),
        0x330 => Some(CsrInfo {
            name: "mhpmevent16",
            rv32_only: false,
        }),
        0x331 => Some(CsrInfo {
            name: "mhpmevent17",
            rv32_only: false,
        }),
        0x332 => Some(CsrInfo {
            name: "mhpmevent18",
            rv32_only: false,
        }),
        0x333 => Some(CsrInfo {
            name: "mhpmevent19",
            rv32_only: false,
        }),
        0x334 => Some(CsrInfo {
            name: "mhpmevent20",
            rv32_only: false,
        }),
        0x335 => Some(CsrInfo {
            name: "mhpmevent21",
            rv32_only: false,
        }),
        0x336 => Some(CsrInfo {
            name: "mhpmevent22",
            rv32_only: false,
        }),
        0x337 => Some(CsrInfo {
            name: "mhpmevent23",
            rv32_only: false,
        }),
        0x338 => Some(CsrInfo {
            name: "mhpmevent24",
            rv32_only: false,
        }),
        0x339 => Some(CsrInfo {
            name: "mhpmevent25",
            rv32_only: false,
        }),
        0x33A => Some(CsrInfo {
            name: "mhpmevent26",
            rv32_only: false,
        }),
        0x33B => Some(CsrInfo {
            name: "mhpmevent27",
            rv32_only: false,
        }),
        0x33C => Some(CsrInfo {
            name: "mhpmevent28",
            rv32_only: false,
        }),
        0x33D => Some(CsrInfo {
            name: "mhpmevent29",
            rv32_only: false,
        }),
        0x33E => Some(CsrInfo {
            name: "mhpmevent30",
            rv32_only: false,
        }),
        0x33F => Some(CsrInfo {
            name: "mhpmevent31",
            rv32_only: false,
        }),
        0x340 => Some(CsrInfo {
            name: "mscratch",
            rv32_only: false,
        }),
        0x341 => Some(CsrInfo {
            name: "mepc",
            rv32_only: false,
        }),
        0x342 => Some(CsrInfo {
            name: "mcause",
            rv32_only: false,
        }),
        0x343 => Some(CsrInfo {
            name: "mtval",
            rv32_only: false,
        }),
        0x344 => Some(CsrInfo {
            name: "mip",
            rv32_only: false,
        }),
        0x34A => Some(CsrInfo {
            name: "mtinst",
            rv32_only: false,
        }),
        0x34B => Some(CsrInfo {
            name: "mtval2",
            rv32_only: false,
        }),
        0x350 => Some(CsrInfo {
            name: "miselect",
            rv32_only: false,
        }),
        0x351 => Some(CsrInfo {
            name: "mireg",
            rv32_only: false,
        }),
        0x354 => Some(CsrInfo {
            name: "miph",
            rv32_only: true,
        }),
        0x35C => Some(CsrInfo {
            name: "mtopei",
            rv32_only: false,
        }),
        0x3A0 => Some(CsrInfo {
            name: "pmpcfg0",
            rv32_only: false,
        }),
        0x3A1 => Some(CsrInfo {
            name: "pmpcfg1",
            rv32_only: true,
        }),
        0x3A2 => Some(CsrInfo {
            name: "pmpcfg2",
            rv32_only: false,
        }),
        0x3A3 => Some(CsrInfo {
            name: "pmpcfg3",
            rv32_only: true,
        }),
        0x3A4 => Some(CsrInfo {
            name: "pmpcfg4",
            rv32_only: false,
        }),
        0x3A5 => Some(CsrInfo {
            name: "pmpcfg5",
            rv32_only: true,
        }),
        0x3A6 => Some(CsrInfo {
            name: "pmpcfg6",
            rv32_only: false,
        }),
        0x3A7 => Some(CsrInfo {
            name: "pmpcfg7",
            rv32_only: true,
        }),
        0x3A8 => Some(CsrInfo {
            name: "pmpcfg8",
            rv32_only: false,
        }),
        0x3A9 => Some(CsrInfo {
            name: "pmpcfg9",
            rv32_only: true,
        }),
        0x3AA => Some(CsrInfo {
            name: "pmpcfg10",
            rv32_only: false,
        }),
        0x3AB => Some(CsrInfo {
            name: "pmpcfg11",
            rv32_only: true,
        }),
        0x3AC => Some(CsrInfo {
            name: "pmpcfg12",
            rv32_only: false,
        }),
        0x3AD => Some(CsrInfo {
            name: "pmpcfg13",
            rv32_only: true,
        }),
        0x3AE => Some(CsrInfo {
            name: "pmpcfg14",
            rv32_only: false,
        }),
        0x3AF => Some(CsrInfo {
            name: "pmpcfg15",
            rv32_only: true,
        }),
        0x3B0 => Some(CsrInfo {
            name: "pmpaddr0",
            rv32_only: false,
        }),
        0x3B1 => Some(CsrInfo {
            name: "pmpaddr1",
            rv32_only: false,
        }),
        0x3B2 => Some(CsrInfo {
            name: "pmpaddr2",
            rv32_only: false,
        }),
        0x3B3 => Some(CsrInfo {
            name: "pmpaddr3",
            rv32_only: false,
        }),
        0x3B4 => Some(CsrInfo {
            name: "pmpaddr4",
            rv32_only: false,
        }),
        0x3B5 => Some(CsrInfo {
            name: "pmpaddr5",
            rv32_only: false,
        }),
        0x3B6 => Some(CsrInfo {
            name: "pmpaddr6",
            rv32_only: false,
        }),
        0x3B7 => Some(CsrInfo {
            name: "pmpaddr7",
            rv32_only: false,
        }),
        0x3B8 => Some(CsrInfo {
            name: "pmpaddr8",
            rv32_only: false,
        }),
        0x3B9 => Some(CsrInfo {
            name: "pmpaddr9",
            rv32_only: false,
        }),
        0x3BA => Some(CsrInfo {
            name: "pmpaddr10",
            rv32_only: false,
        }),
        0x3BB => Some(CsrInfo {
            name: "pmpaddr11",
            rv32_only: false,
        }),
        0x3BC => Some(CsrInfo {
            name: "pmpaddr12",
            rv32_only: false,
        }),
        0x3BD => Some(CsrInfo {
            name: "pmpaddr13",
            rv32_only: false,
        }),
        0x3BE => Some(CsrInfo {
            name: "pmpaddr14",
            rv32_only: false,
        }),
        0x3BF => Some(CsrInfo {
            name: "pmpaddr15",
            rv32_only: false,
        }),
        0x3C0 => Some(CsrInfo {
            name: "pmpaddr16",
            rv32_only: false,
        }),
        0x3C1 => Some(CsrInfo {
            name: "pmpaddr17",
            rv32_only: false,
        }),
        0x3C2 => Some(CsrInfo {
            name: "pmpaddr18",
            rv32_only: false,
        }),
        0x3C3 => Some(CsrInfo {
            name: "pmpaddr19",
            rv32_only: false,
        }),
        0x3C4 => Some(CsrInfo {
            name: "pmpaddr20",
            rv32_only: false,
        }),
        0x3C5 => Some(CsrInfo {
            name: "pmpaddr21",
            rv32_only: false,
        }),
        0x3C6 => Some(CsrInfo {
            name: "pmpaddr22",
            rv32_only: false,
        }),
        0x3C7 => Some(CsrInfo {
            name: "pmpaddr23",
            rv32_only: false,
        }),
        0x3C8 => Some(CsrInfo {
            name: "pmpaddr24",
            rv32_only: false,
        }),
        0x3C9 => Some(CsrInfo {
            name: "pmpaddr25",
            rv32_only: false,
        }),
        0x3CA => Some(CsrInfo {
            name: "pmpaddr26",
            rv32_only: false,
        }),
        0x3CB => Some(CsrInfo {
            name: "pmpaddr27",
            rv32_only: false,
        }),
        0x3CC => Some(CsrInfo {
            name: "pmpaddr28",
            rv32_only: false,
        }),
        0x3CD => Some(CsrInfo {
            name: "pmpaddr29",
            rv32_only: false,
        }),
        0x3CE => Some(CsrInfo {
            name: "pmpaddr30",
            rv32_only: false,
        }),
        0x3CF => Some(CsrInfo {
            name: "pmpaddr31",
            rv32_only: false,
        }),
        0x3D0 => Some(CsrInfo {
            name: "pmpaddr32",
            rv32_only: false,
        }),
        0x3D1 => Some(CsrInfo {
            name: "pmpaddr33",
            rv32_only: false,
        }),
        0x3D2 => Some(CsrInfo {
            name: "pmpaddr34",
            rv32_only: false,
        }),
        0x3D3 => Some(CsrInfo {
            name: "pmpaddr35",
            rv32_only: false,
        }),
        0x3D4 => Some(CsrInfo {
            name: "pmpaddr36",
            rv32_only: false,
        }),
        0x3D5 => Some(CsrInfo {
            name: "pmpaddr37",
            rv32_only: false,
        }),
        0x3D6 => Some(CsrInfo {
            name: "pmpaddr38",
            rv32_only: false,
        }),
        0x3D7 => Some(CsrInfo {
            name: "pmpaddr39",
            rv32_only: false,
        }),
        0x3D8 => Some(CsrInfo {
            name: "pmpaddr40",
            rv32_only: false,
        }),
        0x3D9 => Some(CsrInfo {
            name: "pmpaddr41",
            rv32_only: false,
        }),
        0x3DA => Some(CsrInfo {
            name: "pmpaddr42",
            rv32_only: false,
        }),
        0x3DB => Some(CsrInfo {
            name: "pmpaddr43",
            rv32_only: false,
        }),
        0x3DC => Some(CsrInfo {
            name: "pmpaddr44",
            rv32_only: false,
        }),
        0x3DD => Some(CsrInfo {
            name: "pmpaddr45",
            rv32_only: false,
        }),
        0x3DE => Some(CsrInfo {
            name: "pmpaddr46",
            rv32_only: false,
        }),
        0x3DF => Some(CsrInfo {
            name: "pmpaddr47",
            rv32_only: false,
        }),
        0x3E0 => Some(CsrInfo {
            name: "pmpaddr48",
            rv32_only: false,
        }),
        0x3E1 => Some(CsrInfo {
            name: "pmpaddr49",
            rv32_only: false,
        }),
        0x3E2 => Some(CsrInfo {
            name: "pmpaddr50",
            rv32_only: false,
        }),
        0x3E3 => Some(CsrInfo {
            name: "pmpaddr51",
            rv32_only: false,
        }),
        0x3E4 => Some(CsrInfo {
            name: "pmpaddr52",
            rv32_only: false,
        }),
        0x3E5 => Some(CsrInfo {
            name: "pmpaddr53",
            rv32_only: false,
        }),
        0x3E6 => Some(CsrInfo {
            name: "pmpaddr54",
            rv32_only: false,
        }),
        0x3E7 => Some(CsrInfo {
            name: "pmpaddr55",
            rv32_only: false,
        }),
        0x3E8 => Some(CsrInfo {
            name: "pmpaddr56",
            rv32_only: false,
        }),
        0x3E9 => Some(CsrInfo {
            name: "pmpaddr57",
            rv32_only: false,
        }),
        0x3EA => Some(CsrInfo {
            name: "pmpaddr58",
            rv32_only: false,
        }),
        0x3EB => Some(CsrInfo {
            name: "pmpaddr59",
            rv32_only: false,
        }),
        0x3EC => Some(CsrInfo {
            name: "pmpaddr60",
            rv32_only: false,
        }),
        0x3ED => Some(CsrInfo {
            name: "pmpaddr61",
            rv32_only: false,
        }),
        0x3EE => Some(CsrInfo {
            name: "pmpaddr62",
            rv32_only: false,
        }),
        0x3EF => Some(CsrInfo {
            name: "pmpaddr63",
            rv32_only: false,
        }),
        0x5A8 => Some(CsrInfo {
            name: "scontext",
            rv32_only: false,
        }),
        0x600 => Some(CsrInfo {
            name: "hstatus",
            rv32_only: false,
        }),
        0x602 => Some(CsrInfo {
            name: "hedeleg",
            rv32_only: false,
        }),
        0x603 => Some(CsrInfo {
            name: "hideleg",
            rv32_only: false,
        }),
        0x604 => Some(CsrInfo {
            name: "hie",
            rv32_only: false,
        }),
        0x605 => Some(CsrInfo {
            name: "htimedelta",
            rv32_only: false,
        }),
        0x606 => Some(CsrInfo {
            name: "hcounteren",
            rv32_only: false,
        }),
        0x607 => Some(CsrInfo {
            name: "hgeie",
            rv32_only: false,
        }),
        0x608 => Some(CsrInfo {
            name: "hvien",
            rv32_only: false,
        }),
        0x609 => Some(CsrInfo {
            name: "hvictl",
            rv32_only: false,
        }),
        0x60A => Some(CsrInfo {
            name: "henvcfg",
            rv32_only: false,
        }),
        0x60C => Some(CsrInfo {
            name: "hstateen0",
            rv32_only: false,
        }),
        0x60D => Some(CsrInfo {
            name: "hstateen1",
            rv32_only: false,
        }),
        0x60E => Some(CsrInfo {
            name: "hstateen2",
            rv32_only: false,
        }),
        0x60F => Some(CsrInfo {
            name: "hstateen3",
            rv32_only: false,
        }),
        0x613 => Some(CsrInfo {
            name: "hidelegh",
            rv32_only: true,
        }),
        0x615 => Some(CsrInfo {
            name: "htimedeltah",
            rv32_only: true,
        }),
        0x618 => Some(CsrInfo {
            name: "hvienh",
            rv32_only: true,
        }),
        0x61A => Some(CsrInfo {
            name: "henvcfgh",
            rv32_only: true,
        }),
        0x61C => Some(CsrInfo {
            name: "hstateen0h",
            rv32_only: true,
        }),
        0x61D => Some(CsrInfo {
            name: "hstateen1h",
            rv32_only: true,
        }),
        0x61E => Some(CsrInfo {
            name: "hstateen2h",
            rv32_only: true,
        }),
        0x61F => Some(CsrInfo {
            name: "hstateen3h",
            rv32_only: true,
        }),
        0x643 => Some(CsrInfo {
            name: "htval",
            rv32_only: false,
        }),
        0x644 => Some(CsrInfo {
            name: "hip",
            rv32_only: false,
        }),
        0x645 => Some(CsrInfo {
            name: "hvip",
            rv32_only: false,
        }),
        0x646 => Some(CsrInfo {
            name: "hviprio1",
            rv32_only: false,
        }),
        0x647 => Some(CsrInfo {
            name: "hviprio2",
            rv32_only: false,
        }),
        0x64A => Some(CsrInfo {
            name: "htinst",
            rv32_only: false,
        }),
        0x655 => Some(CsrInfo {
            name: "hviph",
            rv32_only: true,
        }),
        0x656 => Some(CsrInfo {
            name: "hviprio1h",
            rv32_only: true,
        }),
        0x657 => Some(CsrInfo {
            name: "hviprio2h",
            rv32_only: true,
        }),
        0x680 => Some(CsrInfo {
            name: "hgatp",
            rv32_only: false,
        }),
        0x6A8 => Some(CsrInfo {
            name: "hcontext",
            rv32_only: false,
        }),
        0x723 => Some(CsrInfo {
            name: "mhpmevent3h",
            rv32_only: true,
        }),
        0x724 => Some(CsrInfo {
            name: "mhpmevent4h",
            rv32_only: true,
        }),
        0x725 => Some(CsrInfo {
            name: "mhpmevent5h",
            rv32_only: true,
        }),
        0x726 => Some(CsrInfo {
            name: "mhpmevent6h",
            rv32_only: true,
        }),
        0x727 => Some(CsrInfo {
            name: "mhpmevent7h",
            rv32_only: true,
        }),
        0x728 => Some(CsrInfo {
            name: "mhpmevent8h",
            rv32_only: true,
        }),
        0x729 => Some(CsrInfo {
            name: "mhpmevent9h",
            rv32_only: true,
        }),
        0x72A => Some(CsrInfo {
            name: "mhpmevent10h",
            rv32_only: true,
        }),
        0x72B => Some(CsrInfo {
            name: "mhpmevent11h",
            rv32_only: true,
        }),
        0x72C => Some(CsrInfo {
            name: "mhpmevent12h",
            rv32_only: true,
        }),
        0x72D => Some(CsrInfo {
            name: "mhpmevent13h",
            rv32_only: true,
        }),
        0x72E => Some(CsrInfo {
            name: "mhpmevent14h",
            rv32_only: true,
        }),
        0x72F => Some(CsrInfo {
            name: "mhpmevent15h",
            rv32_only: true,
        }),
        0x730 => Some(CsrInfo {
            name: "mhpmevent16h",
            rv32_only: true,
        }),
        0x731 => Some(CsrInfo {
            name: "mhpmevent17h",
            rv32_only: true,
        }),
        0x732 => Some(CsrInfo {
            name: "mhpmevent18h",
            rv32_only: true,
        }),
        0x733 => Some(CsrInfo {
            name: "mhpmevent19h",
            rv32_only: true,
        }),
        0x734 => Some(CsrInfo {
            name: "mhpmevent20h",
            rv32_only: true,
        }),
        0x735 => Some(CsrInfo {
            name: "mhpmevent21h",
            rv32_only: true,
        }),
        0x736 => Some(CsrInfo {
            name: "mhpmevent22h",
            rv32_only: true,
        }),
        0x737 => Some(CsrInfo {
            name: "mhpmevent23h",
            rv32_only: true,
        }),
        0x738 => Some(CsrInfo {
            name: "mhpmevent24h",
            rv32_only: true,
        }),
        0x739 => Some(CsrInfo {
            name: "mhpmevent25h",
            rv32_only: true,
        }),
        0x73A => Some(CsrInfo {
            name: "mhpmevent26h",
            rv32_only: true,
        }),
        0x73B => Some(CsrInfo {
            name: "mhpmevent27h",
            rv32_only: true,
        }),
        0x73C => Some(CsrInfo {
            name: "mhpmevent28h",
            rv32_only: true,
        }),
        0x73D => Some(CsrInfo {
            name: "mhpmevent29h",
            rv32_only: true,
        }),
        0x73E => Some(CsrInfo {
            name: "mhpmevent30h",
            rv32_only: true,
        }),
        0x73F => Some(CsrInfo {
            name: "mhpmevent31h",
            rv32_only: true,
        }),
        0x747 => Some(CsrInfo {
            name: "mseccfg",
            rv32_only: false,
        }),
        0x757 => Some(CsrInfo {
            name: "mseccfgh",
            rv32_only: true,
        }),
        0x7A0 => Some(CsrInfo {
            name: "tselect",
            rv32_only: false,
        }),
        0x7A1 => Some(CsrInfo {
            name: "tdata1",
            rv32_only: false,
        }),
        0x7A2 => Some(CsrInfo {
            name: "tdata2",
            rv32_only: false,
        }),
        0x7A3 => Some(CsrInfo {
            name: "tdata3",
            rv32_only: false,
        }),
        0x7A8 => Some(CsrInfo {
            name: "mcontext",
            rv32_only: false,
        }),
        0x7B0 => Some(CsrInfo {
            name: "dcsr",
            rv32_only: false,
        }),
        0x7B1 => Some(CsrInfo {
            name: "dpc",
            rv32_only: false,
        }),
        0x7B2 => Some(CsrInfo {
            name: "dscratch0",
            rv32_only: false,
        }),
        0x7B3 => Some(CsrInfo {
            name: "dscratch1",
            rv32_only: false,
        }),
        0xB00 => Some(CsrInfo {
            name: "mcycle",
            rv32_only: false,
        }),
        0xB02 => Some(CsrInfo {
            name: "minstret",
            rv32_only: false,
        }),
        0xB03 => Some(CsrInfo {
            name: "mhpmcounter3",
            rv32_only: false,
        }),
        0xB04 => Some(CsrInfo {
            name: "mhpmcounter4",
            rv32_only: false,
        }),
        0xB05 => Some(CsrInfo {
            name: "mhpmcounter5",
            rv32_only: false,
        }),
        0xB06 => Some(CsrInfo {
            name: "mhpmcounter6",
            rv32_only: false,
        }),
        0xB07 => Some(CsrInfo {
            name: "mhpmcounter7",
            rv32_only: false,
        }),
        0xB08 => Some(CsrInfo {
            name: "mhpmcounter8",
            rv32_only: false,
        }),
        0xB09 => Some(CsrInfo {
            name: "mhpmcounter9",
            rv32_only: false,
        }),
        0xB0A => Some(CsrInfo {
            name: "mhpmcounter10",
            rv32_only: false,
        }),
        0xB0B => Some(CsrInfo {
            name: "mhpmcounter11",
            rv32_only: false,
        }),
        0xB0C => Some(CsrInfo {
            name: "mhpmcounter12",
            rv32_only: false,
        }),
        0xB0D => Some(CsrInfo {
            name: "mhpmcounter13",
            rv32_only: false,
        }),
        0xB0E => Some(CsrInfo {
            name: "mhpmcounter14",
            rv32_only: false,
        }),
        0xB0F => Some(CsrInfo {
            name: "mhpmcounter15",
            rv32_only: false,
        }),
        0xB10 => Some(CsrInfo {
            name: "mhpmcounter16",
            rv32_only: false,
        }),
        0xB11 => Some(CsrInfo {
            name: "mhpmcounter17",
            rv32_only: false,
        }),
        0xB12 => Some(CsrInfo {
            name: "mhpmcounter18",
            rv32_only: false,
        }),
        0xB13 => Some(CsrInfo {
            name: "mhpmcounter19",
            rv32_only: false,
        }),
        0xB14 => Some(CsrInfo {
            name: "mhpmcounter20",
            rv32_only: false,
        }),
        0xB15 => Some(CsrInfo {
            name: "mhpmcounter21",
            rv32_only: false,
        }),
        0xB16 => Some(CsrInfo {
            name: "mhpmcounter22",
            rv32_only: false,
        }),
        0xB17 => Some(CsrInfo {
            name: "mhpmcounter23",
            rv32_only: false,
        }),
        0xB18 => Some(CsrInfo {
            name: "mhpmcounter24",
            rv32_only: false,
        }),
        0xB19 => Some(CsrInfo {
            name: "mhpmcounter25",
            rv32_only: false,
        }),
        0xB1A => Some(CsrInfo {
            name: "mhpmcounter26",
            rv32_only: false,
        }),
        0xB1B => Some(CsrInfo {
            name: "mhpmcounter27",
            rv32_only: false,
        }),
        0xB1C => Some(CsrInfo {
            name: "mhpmcounter28",
            rv32_only: false,
        }),
        0xB1D => Some(CsrInfo {
            name: "mhpmcounter29",
            rv32_only: false,
        }),
        0xB1E => Some(CsrInfo {
            name: "mhpmcounter30",
            rv32_only: false,
        }),
        0xB1F => Some(CsrInfo {
            name: "mhpmcounter31",
            rv32_only: false,
        }),
        0xB80 => Some(CsrInfo {
            name: "mcycleh",
            rv32_only: true,
        }),
        0xB82 => Some(CsrInfo {
            name: "minstreth",
            rv32_only: true,
        }),
        0xB83 => Some(CsrInfo {
            name: "mhpmcounter3h",
            rv32_only: true,
        }),
        0xB84 => Some(CsrInfo {
            name: "mhpmcounter4h",
            rv32_only: true,
        }),
        0xB85 => Some(CsrInfo {
            name: "mhpmcounter5h",
            rv32_only: true,
        }),
        0xB86 => Some(CsrInfo {
            name: "mhpmcounter6h",
            rv32_only: true,
        }),
        0xB87 => Some(CsrInfo {
            name: "mhpmcounter7h",
            rv32_only: true,
        }),
        0xB88 => Some(CsrInfo {
            name: "mhpmcounter8h",
            rv32_only: true,
        }),
        0xB89 => Some(CsrInfo {
            name: "mhpmcounter9h",
            rv32_only: true,
        }),
        0xB8A => Some(CsrInfo {
            name: "mhpmcounter10h",
            rv32_only: true,
        }),
        0xB8B => Some(CsrInfo {
            name: "mhpmcounter11h",
            rv32_only: true,
        }),
        0xB8C => Some(CsrInfo {
            name: "mhpmcounter12h",
            rv32_only: true,
        }),
        0xB8D => Some(CsrInfo {
            name: "mhpmcounter13h",
            rv32_only: true,
        }),
        0xB8E => Some(CsrInfo {
            name: "mhpmcounter14h",
            rv32_only: true,
        }),
        0xB8F => Some(CsrInfo {
            name: "mhpmcounter15h",
            rv32_only: true,
        }),
        0xB90 => Some(CsrInfo {
            name: "mhpmcounter16h",
            rv32_only: true,
        }),
        0xB91 => Some(CsrInfo {
            name: "mhpmcounter17h",
            rv32_only: true,
        }),
        0xB92 => Some(CsrInfo {
            name: "mhpmcounter18h",
            rv32_only: true,
        }),
        0xB93 => Some(CsrInfo {
            name: "mhpmcounter19h",
            rv32_only: true,
        }),
        0xB94 => Some(CsrInfo {
            name: "mhpmcounter20h",
            rv32_only: true,
        }),
        0xB95 => Some(CsrInfo {
            name: "mhpmcounter21h",
            rv32_only: true,
        }),
        0xB96 => Some(CsrInfo {
            name: "mhpmcounter22h",
            rv32_only: true,
        }),
        0xB97 => Some(CsrInfo {
            name: "mhpmcounter23h",
            rv32_only: true,
        }),
        0xB98 => Some(CsrInfo {
            name: "mhpmcounter24h",
            rv32_only: true,
        }),
        0xB99 => Some(CsrInfo {
            name: "mhpmcounter25h",
            rv32_only: true,
        }),
        0xB9A => Some(CsrInfo {
            name: "mhpmcounter26h",
            rv32_only: true,
        }),
        0xB9B => Some(CsrInfo {
            name: "mhpmcounter27h",
            rv32_only: true,
        }),
        0xB9C => Some(CsrInfo {
            name: "mhpmcounter28h",
            rv32_only: true,
        }),
        0xB9D => Some(CsrInfo {
            name: "mhpmcounter29h",
            rv32_only: true,
        }),
        0xB9E => Some(CsrInfo {
            name: "mhpmcounter30h",
            rv32_only: true,
        }),
        0xB9F => Some(CsrInfo {
            name: "mhpmcounter31h",
            rv32_only: true,
        }),
        0xC00 => Some(CsrInfo {
            name: "cycle",
            rv32_only: false,
        }),
        0xC01 => Some(CsrInfo {
            name: "time",
            rv32_only: false,
        }),
        0xC02 => Some(CsrInfo {
            name: "instret",
            rv32_only: false,
        }),
        0xC03 => Some(CsrInfo {
            name: "hpmcounter3",
            rv32_only: false,
        }),
        0xC04 => Some(CsrInfo {
            name: "hpmcounter4",
            rv32_only: false,
        }),
        0xC05 => Some(CsrInfo {
            name: "hpmcounter5",
            rv32_only: false,
        }),
        0xC06 => Some(CsrInfo {
            name: "hpmcounter6",
            rv32_only: false,
        }),
        0xC07 => Some(CsrInfo {
            name: "hpmcounter7",
            rv32_only: false,
        }),
        0xC08 => Some(CsrInfo {
            name: "hpmcounter8",
            rv32_only: false,
        }),
        0xC09 => Some(CsrInfo {
            name: "hpmcounter9",
            rv32_only: false,
        }),
        0xC0A => Some(CsrInfo {
            name: "hpmcounter10",
            rv32_only: false,
        }),
        0xC0B => Some(CsrInfo {
            name: "hpmcounter11",
            rv32_only: false,
        }),
        0xC0C => Some(CsrInfo {
            name: "hpmcounter12",
            rv32_only: false,
        }),
        0xC0D => Some(CsrInfo {
            name: "hpmcounter13",
            rv32_only: false,
        }),
        0xC0E => Some(CsrInfo {
            name: "hpmcounter14",
            rv32_only: false,
        }),
        0xC0F => Some(CsrInfo {
            name: "hpmcounter15",
            rv32_only: false,
        }),
        0xC10 => Some(CsrInfo {
            name: "hpmcounter16",
            rv32_only: false,
        }),
        0xC11 => Some(CsrInfo {
            name: "hpmcounter17",
            rv32_only: false,
        }),
        0xC12 => Some(CsrInfo {
            name: "hpmcounter18",
            rv32_only: false,
        }),
        0xC13 => Some(CsrInfo {
            name: "hpmcounter19",
            rv32_only: false,
        }),
        0xC14 => Some(CsrInfo {
            name: "hpmcounter20",
            rv32_only: false,
        }),
        0xC15 => Some(CsrInfo {
            name: "hpmcounter21",
            rv32_only: false,
        }),
        0xC16 => Some(CsrInfo {
            name: "hpmcounter22",
            rv32_only: false,
        }),
        0xC17 => Some(CsrInfo {
            name: "hpmcounter23",
            rv32_only: false,
        }),
        0xC18 => Some(CsrInfo {
            name: "hpmcounter24",
            rv32_only: false,
        }),
        0xC19 => Some(CsrInfo {
            name: "hpmcounter25",
            rv32_only: false,
        }),
        0xC1A => Some(CsrInfo {
            name: "hpmcounter26",
            rv32_only: false,
        }),
        0xC1B => Some(CsrInfo {
            name: "hpmcounter27",
            rv32_only: false,
        }),
        0xC1C => Some(CsrInfo {
            name: "hpmcounter28",
            rv32_only: false,
        }),
        0xC1D => Some(CsrInfo {
            name: "hpmcounter29",
            rv32_only: false,
        }),
        0xC1E => Some(CsrInfo {
            name: "hpmcounter30",
            rv32_only: false,
        }),
        0xC1F => Some(CsrInfo {
            name: "hpmcounter31",
            rv32_only: false,
        }),
        0xC20 => Some(CsrInfo {
            name: "vl",
            rv32_only: false,
        }),
        0xC21 => Some(CsrInfo {
            name: "vtype",
            rv32_only: false,
        }),
        0xC22 => Some(CsrInfo {
            name: "vlenb",
            rv32_only: false,
        }),
        0xC80 => Some(CsrInfo {
            name: "cycleh",
            rv32_only: true,
        }),
        0xC81 => Some(CsrInfo {
            name: "timeh",
            rv32_only: true,
        }),
        0xC82 => Some(CsrInfo {
            name: "instreth",
            rv32_only: true,
        }),
        0xC83 => Some(CsrInfo {
            name: "hpmcounter3h",
            rv32_only: true,
        }),
        0xC84 => Some(CsrInfo {
            name: "hpmcounter4h",
            rv32_only: true,
        }),
        0xC85 => Some(CsrInfo {
            name: "hpmcounter5h",
            rv32_only: true,
        }),
        0xC86 => Some(CsrInfo {
            name: "hpmcounter6h",
            rv32_only: true,
        }),
        0xC87 => Some(CsrInfo {
            name: "hpmcounter7h",
            rv32_only: true,
        }),
        0xC88 => Some(CsrInfo {
            name: "hpmcounter8h",
            rv32_only: true,
        }),
        0xC89 => Some(CsrInfo {
            name: "hpmcounter9h",
            rv32_only: true,
        }),
        0xC8A => Some(CsrInfo {
            name: "hpmcounter10h",
            rv32_only: true,
        }),
        0xC8B => Some(CsrInfo {
            name: "hpmcounter11h",
            rv32_only: true,
        }),
        0xC8C => Some(CsrInfo {
            name: "hpmcounter12h",
            rv32_only: true,
        }),
        0xC8D => Some(CsrInfo {
            name: "hpmcounter13h",
            rv32_only: true,
        }),
        0xC8E => Some(CsrInfo {
            name: "hpmcounter14h",
            rv32_only: true,
        }),
        0xC8F => Some(CsrInfo {
            name: "hpmcounter15h",
            rv32_only: true,
        }),
        0xC90 => Some(CsrInfo {
            name: "hpmcounter16h",
            rv32_only: true,
        }),
        0xC91 => Some(CsrInfo {
            name: "hpmcounter17h",
            rv32_only: true,
        }),
        0xC92 => Some(CsrInfo {
            name: "hpmcounter18h",
            rv32_only: true,
        }),
        0xC93 => Some(CsrInfo {
            name: "hpmcounter19h",
            rv32_only: true,
        }),
        0xC94 => Some(CsrInfo {
            name: "hpmcounter20h",
            rv32_only: true,
        }),
        0xC95 => Some(CsrInfo {
            name: "hpmcounter21h",
            rv32_only: true,
        }),
        0xC96 => Some(CsrInfo {
            name: "hpmcounter22h",
            rv32_only: true,
        }),
        0xC97 => Some(CsrInfo {
            name: "hpmcounter23h",
            rv32_only: true,
        }),
        0xC98 => Some(CsrInfo {
            name: "hpmcounter24h",
            rv32_only: true,
        }),
        0xC99 => Some(CsrInfo {
            name: "hpmcounter25h",
            rv32_only: true,
        }),
        0xC9A => Some(CsrInfo {
            name: "hpmcounter26h",
            rv32_only: true,
        }),
        0xC9B => Some(CsrInfo {
            name: "hpmcounter27h",
            rv32_only: true,
        }),
        0xC9C => Some(CsrInfo {
            name: "hpmcounter28h",
            rv32_only: true,
        }),
        0xC9D => Some(CsrInfo {
            name: "hpmcounter29h",
            rv32_only: true,
        }),
        0xC9E => Some(CsrInfo {
            name: "hpmcounter30h",
            rv32_only: true,
        }),
        0xC9F => Some(CsrInfo {
            name: "hpmcounter31h",
            rv32_only: true,
        }),
        0xDA0 => Some(CsrInfo {
            name: "scountovf",
            rv32_only: false,
        }),
        0xDB0 => Some(CsrInfo {
            name: "stopi",
            rv32_only: false,
        }),
        0xE12 => Some(CsrInfo {
            name: "hgeip",
            rv32_only: false,
        }),
        0xEB0 => Some(CsrInfo {
            name: "vstopi",
            rv32_only: false,
        }),
        0xF11 => Some(CsrInfo {
            name: "mvendorid",
            rv32_only: false,
        }),
        0xF12 => Some(CsrInfo {
            name: "marchid",
            rv32_only: false,
        }),
        0xF13 => Some(CsrInfo {
            name: "mimpid",
            rv32_only: false,
        }),
        0xF14 => Some(CsrInfo {
            name: "mhartid",
            rv32_only: false,
        }),
        0xF15 => Some(CsrInfo {
            name: "mconfigptr",
            rv32_only: false,
        }),
        0xFB0 => Some(CsrInfo {
            name: "mtopi",
            rv32_only: false,
        }),
        _ => None,
    }
}

/// Look up a CSR by its 12-bit address, considering the active XLEN.
///
/// Returns the CSR's symbolic name only when the CSR is defined for the
/// active XLEN. RV32-only CSRs (e.g. pmpcfg1, mstatush) resolve only on RV32.
pub fn lookup_csr_for_xlen(addr: u16, xlen64: bool) -> Option<&'static str> {
    let info = lookup_csr(addr)?;
    if xlen64 && info.rv32_only {
        None
    } else {
        Some(info.name)
    }
}
