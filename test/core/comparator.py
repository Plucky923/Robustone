"""
Output comparison functionality for the test framework.
"""

from dataclasses import dataclass, field
from enum import Enum
import json
import re
from typing import Any, Dict, List, Optional

try:
    from .utils import normalize_output
except ImportError:  # pragma: no cover - script-mode fallback
    from utils import normalize_output


def _extract_asm_text(tool_output: str) -> str:
    """
    Strip address/byte prefix from cstool/robustone output.

    Handles formats like:
    - '0  fd 2f  jal\t0x7fe' → 'jal 0x7fe'
    - ' 0  fd 2f        jal\t0x7fe' → 'jal 0x7fe'
    - '.byte\t0xff' → '.byte 0xff'
    """
    stripped = tool_output.strip()
    if not stripped:
        return ""
    parts = stripped.split()

    i = 0
    if parts and parts[0].isdigit():
        i = 1  # Skip address

    # Skip hex bytes (2-character hex strings)
    while (
        i < len(parts)
        and len(parts[i]) == 2
        and all(c in "0123456789abcdefABCDEF" for c in parts[i])
    ):
        i += 1

    asm_parts = parts[i:]
    return " ".join(asm_parts)


class ComparisonResult(Enum):
    """Result of output comparison."""

    MATCH = "match"
    MISMATCH = "mismatch"
    COMMAND_FAILURE = "command_failure"
    DOCUMENTATION_DRIFT = "documentation_drift"


class ComparisonSurface(Enum):
    """Independent comparison surfaces within the differential harness."""

    TEXT = "text"
    SEMANTIC_DETAIL = "semantic_detail"


@dataclass
class SurfaceComparison:
    """Comparison result for one explicit surface."""

    surface: ComparisonSurface
    matched: bool
    robustone_value: str
    cstool_value: str


@dataclass
class TestCaseResult:
    """Result of a single test case."""

    hex_input: str
    result: ComparisonResult
    expected_output: str
    robustone_output: str
    cstool_output: str
    note: str
    robustone_exit_code: int = 0
    cstool_exit_code: int = 0
    robustone_stderr: str = ""
    cstool_stderr: str = ""
    execution_time_ms: int = 0
    surface_results: List[SurfaceComparison] = field(default_factory=list)


@dataclass
class ArchTestSummary:
    """Summary of test results for an architecture."""

    arch_name: str
    total_cases: int
    matches: int
    mismatches: int
    command_failures: int
    documentation_drifts: int
    execution_time_ms: int
    results: List[TestCaseResult]


_RISCV_GPR_NAMES = [
    "zero",
    "ra",
    "sp",
    "gp",
    "tp",
    "t0",
    "t1",
    "t2",
    "s0",
    "s1",
    "a0",
    "a1",
    "a2",
    "a3",
    "a4",
    "a5",
    "a6",
    "a7",
    "s2",
    "s3",
    "s4",
    "s5",
    "s6",
    "s7",
    "s8",
    "s9",
    "s10",
    "s11",
    "t3",
    "t4",
    "t5",
    "t6",
]

_RISCV_FPR_NAMES = [
    "ft0",
    "ft1",
    "ft2",
    "ft3",
    "ft4",
    "ft5",
    "ft6",
    "ft7",
    "fs0",
    "fs1",
    "fa0",
    "fa1",
    "fa2",
    "fa3",
    "fa4",
    "fa5",
    "fa6",
    "fa7",
    "fs2",
    "fs3",
    "fs4",
    "fs5",
    "fs6",
    "fs7",
    "fs8",
    "fs9",
    "fs10",
    "fs11",
    "ft8",
    "ft9",
    "ft10",
    "ft11",
]

_RISCV_REGISTER_IDS: Dict[str, int] = {"fp": 8}
_RISCV_REGISTER_IDS.update({f"x{idx}": idx for idx in range(32)})
_RISCV_REGISTER_IDS.update({f"f{idx}": 32 + idx for idx in range(32)})
_RISCV_REGISTER_IDS.update({name: idx for idx, name in enumerate(_RISCV_GPR_NAMES)})
_RISCV_REGISTER_IDS.update(
    {name: 32 + idx for idx, name in enumerate(_RISCV_FPR_NAMES)}
)

_RISCV_CSR_IDS = {
    "ustatus": 0x000,
    "fflags": 0x001,
    "frm": 0x002,
    "fcsr": 0x003,
    "vstart": 0x008,
    "vxsat": 0x009,
    "vxrm": 0x00A,
    "vcsr": 0x00F,
    "seed": 0x015,
    "jvt": 0x017,
    "sstatus": 0x100,
    "sedeleg": 0x102,
    "sideleg": 0x103,
    "sie": 0x104,
    "stvec": 0x105,
    "scounteren": 0x106,
    "senvcfg": 0x10A,
    "sstateen0": 0x10C,
    "sstateen1": 0x10D,
    "sstateen2": 0x10E,
    "sstateen3": 0x10F,
    "sieh": 0x114,
    "sscratch": 0x140,
    "sepc": 0x141,
    "scause": 0x142,
    "stval": 0x143,
    "sip": 0x144,
    "stimecmp": 0x14D,
    "siselect": 0x150,
    "sireg": 0x151,
    "siph": 0x154,
    "stopei": 0x15C,
    "stimecmph": 0x15D,
    "satp": 0x180,
    "vsstatus": 0x200,
    "vsie": 0x204,
    "vstvec": 0x205,
    "vsieh": 0x214,
    "vsscratch": 0x240,
    "vsepc": 0x241,
    "vscause": 0x242,
    "vstval": 0x243,
    "vsip": 0x244,
    "vstimecmp": 0x24D,
    "vsiselect": 0x250,
    "vsireg": 0x251,
    "vsiph": 0x254,
    "vstopei": 0x25C,
    "vstimecmph": 0x25D,
    "vsatp": 0x280,
    "mstatus": 0x300,
    "misa": 0x301,
    "medeleg": 0x302,
    "mideleg": 0x303,
    "mie": 0x304,
    "mtvec": 0x305,
    "mcounteren": 0x306,
    "mvien": 0x308,
    "mvip": 0x309,
    "menvcfg": 0x30A,
    "mstateen0": 0x30C,
    "mstateen1": 0x30D,
    "mstateen2": 0x30E,
    "mstateen3": 0x30F,
    "mstatush": 0x310,
    "midelegh": 0x313,
    "mieh": 0x314,
    "mvienh": 0x318,
    "mviph": 0x319,
    "menvcfgh": 0x31A,
    "mstateen0h": 0x31C,
    "mstateen1h": 0x31D,
    "mstateen2h": 0x31E,
    "mstateen3h": 0x31F,
    "mcountinhibit": 0x320,
    "mhpmevent3": 0x323,
    "mhpmevent4": 0x324,
    "mhpmevent5": 0x325,
    "mhpmevent6": 0x326,
    "mhpmevent7": 0x327,
    "mhpmevent8": 0x328,
    "mhpmevent9": 0x329,
    "mhpmevent10": 0x32A,
    "mhpmevent11": 0x32B,
    "mhpmevent12": 0x32C,
    "mhpmevent13": 0x32D,
    "mhpmevent14": 0x32E,
    "mhpmevent15": 0x32F,
    "mhpmevent16": 0x330,
    "mhpmevent17": 0x331,
    "mhpmevent18": 0x332,
    "mhpmevent19": 0x333,
    "mhpmevent20": 0x334,
    "mhpmevent21": 0x335,
    "mhpmevent22": 0x336,
    "mhpmevent23": 0x337,
    "mhpmevent24": 0x338,
    "mhpmevent25": 0x339,
    "mhpmevent26": 0x33A,
    "mhpmevent27": 0x33B,
    "mhpmevent28": 0x33C,
    "mhpmevent29": 0x33D,
    "mhpmevent30": 0x33E,
    "mhpmevent31": 0x33F,
    "mscratch": 0x340,
    "mepc": 0x341,
    "mcause": 0x342,
    "mtval": 0x343,
    "mip": 0x344,
    "mtinst": 0x34A,
    "mtval2": 0x34B,
    "miselect": 0x350,
    "mireg": 0x351,
    "miph": 0x354,
    "mtopei": 0x35C,
    "pmpcfg0": 0x3A0,
    "pmpcfg1": 0x3A1,
    "pmpcfg2": 0x3A2,
    "pmpcfg3": 0x3A3,
    "pmpcfg4": 0x3A4,
    "pmpcfg5": 0x3A5,
    "pmpcfg6": 0x3A6,
    "pmpcfg7": 0x3A7,
    "pmpcfg8": 0x3A8,
    "pmpcfg9": 0x3A9,
    "pmpcfg10": 0x3AA,
    "pmpcfg11": 0x3AB,
    "pmpcfg12": 0x3AC,
    "pmpcfg13": 0x3AD,
    "pmpcfg14": 0x3AE,
    "pmpcfg15": 0x3AF,
    "pmpaddr0": 0x3B0,
    "pmpaddr1": 0x3B1,
    "pmpaddr2": 0x3B2,
    "pmpaddr3": 0x3B3,
    "pmpaddr4": 0x3B4,
    "pmpaddr5": 0x3B5,
    "pmpaddr6": 0x3B6,
    "pmpaddr7": 0x3B7,
    "pmpaddr8": 0x3B8,
    "pmpaddr9": 0x3B9,
    "pmpaddr10": 0x3BA,
    "pmpaddr11": 0x3BB,
    "pmpaddr12": 0x3BC,
    "pmpaddr13": 0x3BD,
    "pmpaddr14": 0x3BE,
    "pmpaddr15": 0x3BF,
    "pmpaddr16": 0x3C0,
    "pmpaddr17": 0x3C1,
    "pmpaddr18": 0x3C2,
    "pmpaddr19": 0x3C3,
    "pmpaddr20": 0x3C4,
    "pmpaddr21": 0x3C5,
    "pmpaddr22": 0x3C6,
    "pmpaddr23": 0x3C7,
    "pmpaddr24": 0x3C8,
    "pmpaddr25": 0x3C9,
    "pmpaddr26": 0x3CA,
    "pmpaddr27": 0x3CB,
    "pmpaddr28": 0x3CC,
    "pmpaddr29": 0x3CD,
    "pmpaddr30": 0x3CE,
    "pmpaddr31": 0x3CF,
    "pmpaddr32": 0x3D0,
    "pmpaddr33": 0x3D1,
    "pmpaddr34": 0x3D2,
    "pmpaddr35": 0x3D3,
    "pmpaddr36": 0x3D4,
    "pmpaddr37": 0x3D5,
    "pmpaddr38": 0x3D6,
    "pmpaddr39": 0x3D7,
    "pmpaddr40": 0x3D8,
    "pmpaddr41": 0x3D9,
    "pmpaddr42": 0x3DA,
    "pmpaddr43": 0x3DB,
    "pmpaddr44": 0x3DC,
    "pmpaddr45": 0x3DD,
    "pmpaddr46": 0x3DE,
    "pmpaddr47": 0x3DF,
    "pmpaddr48": 0x3E0,
    "pmpaddr49": 0x3E1,
    "pmpaddr50": 0x3E2,
    "pmpaddr51": 0x3E3,
    "pmpaddr52": 0x3E4,
    "pmpaddr53": 0x3E5,
    "pmpaddr54": 0x3E6,
    "pmpaddr55": 0x3E7,
    "pmpaddr56": 0x3E8,
    "pmpaddr57": 0x3E9,
    "pmpaddr58": 0x3EA,
    "pmpaddr59": 0x3EB,
    "pmpaddr60": 0x3EC,
    "pmpaddr61": 0x3ED,
    "pmpaddr62": 0x3EE,
    "pmpaddr63": 0x3EF,
    "scontext": 0x5A8,
    "hstatus": 0x600,
    "hedeleg": 0x602,
    "hideleg": 0x603,
    "hie": 0x604,
    "htimedelta": 0x605,
    "hcounteren": 0x606,
    "hgeie": 0x607,
    "hvien": 0x608,
    "hvictl": 0x609,
    "henvcfg": 0x60A,
    "hstateen0": 0x60C,
    "hstateen1": 0x60D,
    "hstateen2": 0x60E,
    "hstateen3": 0x60F,
    "hidelegh": 0x613,
    "htimedeltah": 0x615,
    "hvienh": 0x618,
    "henvcfgh": 0x61A,
    "hstateen0h": 0x61C,
    "hstateen1h": 0x61D,
    "hstateen2h": 0x61E,
    "hstateen3h": 0x61F,
    "htval": 0x643,
    "hip": 0x644,
    "hvip": 0x645,
    "hviprio1": 0x646,
    "hviprio2": 0x647,
    "htinst": 0x64A,
    "hviph": 0x655,
    "hviprio1h": 0x656,
    "hviprio2h": 0x657,
    "hgatp": 0x680,
    "hcontext": 0x6A8,
    "mhpmevent3h": 0x723,
    "mhpmevent4h": 0x724,
    "mhpmevent5h": 0x725,
    "mhpmevent6h": 0x726,
    "mhpmevent7h": 0x727,
    "mhpmevent8h": 0x728,
    "mhpmevent9h": 0x729,
    "mhpmevent10h": 0x72A,
    "mhpmevent11h": 0x72B,
    "mhpmevent12h": 0x72C,
    "mhpmevent13h": 0x72D,
    "mhpmevent14h": 0x72E,
    "mhpmevent15h": 0x72F,
    "mhpmevent16h": 0x730,
    "mhpmevent17h": 0x731,
    "mhpmevent18h": 0x732,
    "mhpmevent19h": 0x733,
    "mhpmevent20h": 0x734,
    "mhpmevent21h": 0x735,
    "mhpmevent22h": 0x736,
    "mhpmevent23h": 0x737,
    "mhpmevent24h": 0x738,
    "mhpmevent25h": 0x739,
    "mhpmevent26h": 0x73A,
    "mhpmevent27h": 0x73B,
    "mhpmevent28h": 0x73C,
    "mhpmevent29h": 0x73D,
    "mhpmevent30h": 0x73E,
    "mhpmevent31h": 0x73F,
    "mseccfg": 0x747,
    "mseccfgh": 0x757,
    "tselect": 0x7A0,
    "tdata1": 0x7A1,
    "tdata2": 0x7A2,
    "tdata3": 0x7A3,
    "mcontext": 0x7A8,
    "dcsr": 0x7B0,
    "dpc": 0x7B1,
    "dscratch0": 0x7B2,
    "dscratch1": 0x7B3,
    "mcycle": 0xB00,
    "minstret": 0xB02,
    "mhpmcounter3": 0xB03,
    "mhpmcounter4": 0xB04,
    "mhpmcounter5": 0xB05,
    "mhpmcounter6": 0xB06,
    "mhpmcounter7": 0xB07,
    "mhpmcounter8": 0xB08,
    "mhpmcounter9": 0xB09,
    "mhpmcounter10": 0xB0A,
    "mhpmcounter11": 0xB0B,
    "mhpmcounter12": 0xB0C,
    "mhpmcounter13": 0xB0D,
    "mhpmcounter14": 0xB0E,
    "mhpmcounter15": 0xB0F,
    "mhpmcounter16": 0xB10,
    "mhpmcounter17": 0xB11,
    "mhpmcounter18": 0xB12,
    "mhpmcounter19": 0xB13,
    "mhpmcounter20": 0xB14,
    "mhpmcounter21": 0xB15,
    "mhpmcounter22": 0xB16,
    "mhpmcounter23": 0xB17,
    "mhpmcounter24": 0xB18,
    "mhpmcounter25": 0xB19,
    "mhpmcounter26": 0xB1A,
    "mhpmcounter27": 0xB1B,
    "mhpmcounter28": 0xB1C,
    "mhpmcounter29": 0xB1D,
    "mhpmcounter30": 0xB1E,
    "mhpmcounter31": 0xB1F,
    "mcycleh": 0xB80,
    "minstreth": 0xB82,
    "mhpmcounter3h": 0xB83,
    "mhpmcounter4h": 0xB84,
    "mhpmcounter5h": 0xB85,
    "mhpmcounter6h": 0xB86,
    "mhpmcounter7h": 0xB87,
    "mhpmcounter8h": 0xB88,
    "mhpmcounter9h": 0xB89,
    "mhpmcounter10h": 0xB8A,
    "mhpmcounter11h": 0xB8B,
    "mhpmcounter12h": 0xB8C,
    "mhpmcounter13h": 0xB8D,
    "mhpmcounter14h": 0xB8E,
    "mhpmcounter15h": 0xB8F,
    "mhpmcounter16h": 0xB90,
    "mhpmcounter17h": 0xB91,
    "mhpmcounter18h": 0xB92,
    "mhpmcounter19h": 0xB93,
    "mhpmcounter20h": 0xB94,
    "mhpmcounter21h": 0xB95,
    "mhpmcounter22h": 0xB96,
    "mhpmcounter23h": 0xB97,
    "mhpmcounter24h": 0xB98,
    "mhpmcounter25h": 0xB99,
    "mhpmcounter26h": 0xB9A,
    "mhpmcounter27h": 0xB9B,
    "mhpmcounter28h": 0xB9C,
    "mhpmcounter29h": 0xB9D,
    "mhpmcounter30h": 0xB9E,
    "mhpmcounter31h": 0xB9F,
    "cycle": 0xC00,
    "time": 0xC01,
    "instret": 0xC02,
    "hpmcounter3": 0xC03,
    "hpmcounter4": 0xC04,
    "hpmcounter5": 0xC05,
    "hpmcounter6": 0xC06,
    "hpmcounter7": 0xC07,
    "hpmcounter8": 0xC08,
    "hpmcounter9": 0xC09,
    "hpmcounter10": 0xC0A,
    "hpmcounter11": 0xC0B,
    "hpmcounter12": 0xC0C,
    "hpmcounter13": 0xC0D,
    "hpmcounter14": 0xC0E,
    "hpmcounter15": 0xC0F,
    "hpmcounter16": 0xC10,
    "hpmcounter17": 0xC11,
    "hpmcounter18": 0xC12,
    "hpmcounter19": 0xC13,
    "hpmcounter20": 0xC14,
    "hpmcounter21": 0xC15,
    "hpmcounter22": 0xC16,
    "hpmcounter23": 0xC17,
    "hpmcounter24": 0xC18,
    "hpmcounter25": 0xC19,
    "hpmcounter26": 0xC1A,
    "hpmcounter27": 0xC1B,
    "hpmcounter28": 0xC1C,
    "hpmcounter29": 0xC1D,
    "hpmcounter30": 0xC1E,
    "hpmcounter31": 0xC1F,
    "vl": 0xC20,
    "vtype": 0xC21,
    "vlenb": 0xC22,
    "cycleh": 0xC80,
    "timeh": 0xC81,
    "instreth": 0xC82,
    "hpmcounter3h": 0xC83,
    "hpmcounter4h": 0xC84,
    "hpmcounter5h": 0xC85,
    "hpmcounter6h": 0xC86,
    "hpmcounter7h": 0xC87,
    "hpmcounter8h": 0xC88,
    "hpmcounter9h": 0xC89,
    "hpmcounter10h": 0xC8A,
    "hpmcounter11h": 0xC8B,
    "hpmcounter12h": 0xC8C,
    "hpmcounter13h": 0xC8D,
    "hpmcounter14h": 0xC8E,
    "hpmcounter15h": 0xC8F,
    "hpmcounter16h": 0xC90,
    "hpmcounter17h": 0xC91,
    "hpmcounter18h": 0xC92,
    "hpmcounter19h": 0xC93,
    "hpmcounter20h": 0xC94,
    "hpmcounter21h": 0xC95,
    "hpmcounter22h": 0xC96,
    "hpmcounter23h": 0xC97,
    "hpmcounter24h": 0xC98,
    "hpmcounter25h": 0xC99,
    "hpmcounter26h": 0xC9A,
    "hpmcounter27h": 0xC9B,
    "hpmcounter28h": 0xC9C,
    "hpmcounter29h": 0xC9D,
    "hpmcounter30h": 0xC9E,
    "hpmcounter31h": 0xC9F,
    "scountovf": 0xDA0,
    "stopi": 0xDB0,
    "hgeip": 0xE12,
    "vstopi": 0xEB0,
    "mvendorid": 0xF11,
    "marchid": 0xF12,
    "mimpid": 0xF13,
    "mhartid": 0xF14,
    "mconfigptr": 0xF15,
    "mtopi": 0xFB0,
}


# Mapping of mnemonics that are aliases of each other.
# Used for loose comparison between Robustone (non-aliased) and Capstone (aliased).
_MNEMONIC_EQUIVALENTS: Dict[str, str] = {
    "c.srli": "srli",
    "c.srai": "srai",
    "c.andi": "andi",
    "c.slli64": "c.slli",
    "c.srli64": "c.srli",
    "c.srai64": "c.srai",
    "c.sub": "sub",
    "c.xor": "xor",
    "c.or": "or",
    "c.and": "and",
    "c.j": "j",
    "c.jal": "jal",
    "c.beqz": "beqz",
    "c.bnez": "bnez",
    "c.slli": "slli",
    "c.addi4spn": "addi",
    "c.addi16sp": "addi",
    "c.lwsp": "lw",
    "c.swsp": "sw",
    "c.ldsp": "ld",
    "c.sdsp": "sd",
    "c.lw": "lw",
    "c.sw": "sw",
    "c.ld": "ld",
    "c.sd": "sd",
    "c.fld": "fld",
    "c.fsd": "fsd",
    "c.flw": "flw",
    "c.fsw": "fsw",
    "c.fldsp": "fld",
    "c.fsdsp": "fsd",
    "c.flwsp": "flw",
    "c.fswsp": "fsw",
    "c.addi": "addi",
    "c.addiw": "addiw",
    "c.li": "addi",
    "c.lui": "lui",
    "c.mv": "addi",
    "c.add": "add",
    "c.nop": "addi",
    "c.jr": "jr",
    "c.jalr": "jalr",
    "c.ebreak": "ebreak",
    "ret": "jr",
    "nop": "addi",
    "li": "addi",
    "mv": "addi",
    "unimp": "c.unimp",
    "csrw": "csrrw",
    "csrr": "csrrs",
    "csrs": "csrrs",
    "csrc": "csrrc",
    "csrwi": "csrrwi",
    "csrsi": "csrrsi",
    "csrci": "csrrci",
    "rdcycle": "csrrs",
    "rdtime": "csrrs",
    "rdinstret": "csrrs",
    "rdcycleh": "csrrs",
    "rdtimeh": "csrrs",
    "rdinstreth": "csrrs",
    "c.subw": "subw",
    "c.addw": "addw",
    "fmv.w.x": "fli.s",
    "fmv.d.x": "fli.d",
}


class OutputComparator:
    """Handles comparison of robustone and cstool outputs."""

    def __init__(self, strict_match: bool = True, ignore_whitespace: bool = True):
        """
        Initialize the comparator.

        Args:
            strict_match: If True, require exact string match
            ignore_whitespace: If True, normalize whitespace before comparison
        """
        self.strict_match = strict_match
        self.ignore_whitespace = ignore_whitespace

    def _compare_mnemonic(self, a: str, b: str) -> bool:
        """Compare two mnemonics, accounting for known alias pairs."""
        a_norm = _MNEMONIC_EQUIVALENTS.get(a, a)
        b_norm = _MNEMONIC_EQUIVALENTS.get(b, b)
        return a_norm == b_norm

    def compare_outputs(self, robustone_out: str, cstool_out: str) -> bool:
        """
        Compare two output strings.

        Args:
            robustone_out: Output from robustone
            cstool_out: Output from cstool

        Returns:
            True if outputs match according to comparison rules
        """
        if self.strict_match:
            if self.ignore_whitespace:
                return normalize_output(robustone_out) == normalize_output(cstool_out)

            return robustone_out == cstool_out

        # Normal comparison
        if normalize_output(robustone_out) == normalize_output(cstool_out):
            return True

        # Loose alias-aware comparison: extract asm text and compare mnemonics
        rob_asm = _extract_asm_text(robustone_out)
        cs_asm = _extract_asm_text(cstool_out)
        rob_parts = rob_asm.split()
        cs_parts = cs_asm.split()
        if rob_parts and cs_parts:
            return self._compare_mnemonic(rob_parts[0], cs_parts[0])

        return False

    def compare_text_surface(
        self, robustone_out: str, cstool_out: str
    ) -> SurfaceComparison:
        """Compare the human-readable text surface."""
        return SurfaceComparison(
            surface=ComparisonSurface.TEXT,
            matched=self.compare_outputs(robustone_out, cstool_out),
            robustone_value=robustone_out,
            cstool_value=cstool_out,
        )

    def compare_semantic_detail_surface(
        self, robustone_json: str, cstool_detail: str
    ) -> SurfaceComparison:
        """Compare the detail/semantic surface."""
        robustone_record = self._normalize_robustone_semantic_output(robustone_json)
        cstool_record = self._normalize_cstool_semantic_output(cstool_detail)

        return SurfaceComparison(
            surface=ComparisonSurface.SEMANTIC_DETAIL,
            matched=robustone_record == cstool_record,
            robustone_value=json.dumps(robustone_record, sort_keys=True),
            cstool_value=json.dumps(cstool_record, sort_keys=True),
        )

    def check_documentation_drift(self, expected: str, actual: str) -> bool:
        """
        Check if documented expected output differs from actual cstool output.

        Args:
            expected: Expected output from documentation
            actual: Actual output from cstool

        Returns:
            True if there is documentation drift
        """
        if not expected:
            return False
        expected_norm = normalize_output(expected)
        actual_norm = normalize_output(_extract_asm_text(actual))
        return expected_norm != actual_norm

    def classify_result(  # pylint: disable=too-many-return-statements
        self,
        expected: str,
        cstool_out: str,
        surface_results: List[SurfaceComparison],
        command_failed: bool,
    ) -> ComparisonResult:
        """
        Classify the result of a test case.

        Args:
            expected: Expected output from documentation
            cstool_out: Text output from cstool
            surface_results: Per-surface comparison results
            command_failed: Whether any surface command failed

        Returns:
            ComparisonResult classification
        """
        text_matched = any(
            s.matched for s in surface_results if s.surface == ComparisonSurface.TEXT
        )

        # In strict mode, require all surfaces to match.
        # In loose mode, text match is sufficient (semantic_detail may diverge
        # due to alias expansion differences, e.g. c.sub vs sub).
        if self.strict_match:
            if not text_matched:
                if command_failed:
                    return ComparisonResult.COMMAND_FAILURE
                if expected and self.check_documentation_drift(expected, cstool_out):
                    return ComparisonResult.DOCUMENTATION_DRIFT
                return ComparisonResult.MISMATCH

            if all(surface.matched for surface in surface_results):
                return ComparisonResult.MATCH

            if command_failed:
                return ComparisonResult.COMMAND_FAILURE
            return ComparisonResult.MISMATCH

        # Loose mode: text surface is authoritative
        if text_matched:
            return ComparisonResult.MATCH

        if command_failed:
            return ComparisonResult.COMMAND_FAILURE

        if expected and self.check_documentation_drift(expected, cstool_out):
            return ComparisonResult.DOCUMENTATION_DRIFT

        return ComparisonResult.MISMATCH

    def create_test_result(
        self,
        hex_input: str,
        expected: str,
        robustone_out: str,
        cstool_out: str,
        note: str,
        robustone_exit_code: int = 0,
        cstool_exit_code: int = 0,
        robustone_stderr: str = "",
        cstool_stderr: str = "",
        execution_time_ms: int = 0,
        robustone_semantic_out: str = "",
        cstool_semantic_out: str = "",
        robustone_semantic_exit_code: int = 0,
        cstool_semantic_exit_code: int = 0,
        robustone_semantic_stderr: str = "",
        cstool_semantic_stderr: str = "",
    ) -> TestCaseResult:
        """
        Create a TestCaseResult from test execution data.
        """
        surface_results = [self.compare_text_surface(robustone_out, cstool_out)]
        command_failed = any(
            code != 0
            for code in (
                robustone_exit_code,
                cstool_exit_code,
                robustone_semantic_exit_code,
                cstool_semantic_exit_code,
            )
        )

        semantic_parse_error = ""
        if not command_failed:
            try:
                surface_results.append(
                    self.compare_semantic_detail_surface(
                        robustone_semantic_out, cstool_semantic_out
                    )
                )
            except ValueError as exc:
                semantic_parse_error = f"semantic_detail parse failed: {exc}"
                command_failed = True

        combined_robustone_stderr = self._combine_streams(
            robustone_stderr, robustone_semantic_stderr, "semantic_detail"
        )
        combined_cstool_stderr = self._combine_streams(
            cstool_stderr, cstool_semantic_stderr, "semantic_detail"
        )
        if semantic_parse_error:
            combined_robustone_stderr = self._append_message(
                combined_robustone_stderr, semantic_parse_error
            )

        result = self.classify_result(
            expected=expected,
            cstool_out=cstool_out,
            surface_results=surface_results,
            command_failed=command_failed,
        )

        return TestCaseResult(
            hex_input=hex_input,
            result=result,
            expected_output=expected,
            robustone_output=robustone_out,
            cstool_output=cstool_out,
            note=note,
            robustone_exit_code=robustone_exit_code or robustone_semantic_exit_code,
            cstool_exit_code=cstool_exit_code or cstool_semantic_exit_code,
            robustone_stderr=combined_robustone_stderr,
            cstool_stderr=combined_cstool_stderr,
            execution_time_ms=execution_time_ms,
            surface_results=surface_results,
        )

    def generate_summary(
        self, arch_name: str, results: List[TestCaseResult], total_time_ms: int = 0
    ) -> ArchTestSummary:
        """
        Generate a summary of test results.
        """
        total_cases = len(results)
        matches = sum(1 for r in results if r.result == ComparisonResult.MATCH)
        mismatches = sum(1 for r in results if r.result == ComparisonResult.MISMATCH)
        command_failures = sum(
            1 for r in results if r.result == ComparisonResult.COMMAND_FAILURE
        )
        documentation_drifts = sum(
            1 for r in results if r.result == ComparisonResult.DOCUMENTATION_DRIFT
        )

        return ArchTestSummary(
            arch_name=arch_name,
            total_cases=total_cases,
            matches=matches,
            mismatches=mismatches,
            command_failures=command_failures,
            documentation_drifts=documentation_drifts,
            execution_time_ms=total_time_ms,
            results=results,
        )

    def get_failed_results(
        self, results: List[TestCaseResult], include_drift: bool = True
    ) -> List[TestCaseResult]:
        """
        Get list of failed test results.
        """
        failed = [
            r
            for r in results
            if r.result in [ComparisonResult.MISMATCH, ComparisonResult.COMMAND_FAILURE]
        ]
        if include_drift:
            failed.extend(
                [r for r in results if r.result == ComparisonResult.DOCUMENTATION_DRIFT]
            )
        return failed

    def format_result_detailed(self, result: TestCaseResult) -> str:
        """
        Format a test result for detailed display.
        """
        lines = [f"Instruction: {result.hex_input}"]

        if result.expected_output:
            lines.append(f"  Expected:   {result.expected_output}")

        lines.append(f"  Robustone:  {result.robustone_output}")
        lines.append(f"  Cstool:     {result.cstool_output}")

        for surface in result.surface_results:
            if surface.matched:
                continue
            lines.append(f"  Surface {surface.surface.value}: mismatch")
            lines.append(f"    Robustone: {surface.robustone_value}")
            lines.append(f"    Cstool:    {surface.cstool_value}")

        if result.note:
            lines.append(f"  Note:       {result.note}")

        if result.robustone_stderr:
            lines.append(f"  Robustone stderr: {result.robustone_stderr}")

        if result.cstool_stderr:
            lines.append(f"  Cstool stderr:    {result.cstool_stderr}")

        if result.execution_time_ms > 0:
            lines.append(f"  Execution time: {result.execution_time_ms}ms")

        return "\n".join(lines)

    def _normalize_robustone_semantic_output(self, output: str) -> List[Dict[str, Any]]:
        try:
            payload = json.loads(output)
        except json.JSONDecodeError as exc:
            raise ValueError(f"invalid robustone JSON: {exc}") from exc

        instructions = payload.get("instructions")
        if not isinstance(instructions, list) or not instructions:
            raise ValueError(
                "robustone semantic surface must contain at least one instruction"
            )

        return [
            self._normalize_robustone_semantic_instruction(instruction)
            for instruction in instructions
        ]

    def _normalize_robustone_semantic_instruction(
        self, instruction: Dict[str, Any]
    ) -> Dict[str, Any]:
        decoded = instruction.get("decoded")
        if not isinstance(decoded, dict):
            raise ValueError("robustone semantic surface is missing decoded IR")

        operands = []
        for operand in decoded.get("operands", []):
            normalized = self._normalize_robustone_operand(operand)
            if normalized is not None:
                operands.append(normalized)

        return {
            "opcode_id": self._normalize_opcode_name(
                decoded.get("opcode_id")
                or decoded.get("mnemonic")
                or instruction.get("mnemonic")
            ),
            "operands": operands,
            "registers_read": [
                self._normalize_robustone_register(register)
                for register in decoded.get("registers_read", [])
            ],
            "registers_written": [
                self._normalize_robustone_register(register)
                for register in decoded.get("registers_written", [])
            ],
        }

    def _normalize_cstool_semantic_output(  # pylint: disable=too-many-branches
        self, output: str
    ) -> List[Dict[str, Any]]:
        blocks = []
        current_block: List[str] = []
        for line in output.splitlines():
            if re.match(r"^\s*[0-9a-fA-F]+\s+[0-9a-fA-F]{2}\s", line):
                if current_block:
                    blocks.append("\n".join(current_block))
                current_block = [line]
                continue
            if current_block:
                current_block.append(line)

        if current_block:
            blocks.append("\n".join(current_block))

        if not blocks:
            raise ValueError(
                "cstool semantic surface must contain at least one instruction"
            )

        return [self._normalize_cstool_semantic_instruction(block) for block in blocks]

    def _normalize_cstool_semantic_instruction(  # pylint: disable=too-many-branches
        self, output: str
    ) -> Dict[str, Any]:
        lines = [line.strip() for line in output.splitlines() if line.strip()]
        id_line = next((line for line in lines if line.startswith("ID:")), "")
        id_match = re.search(r"\(([^)]+)\)", id_line)
        if not id_match:
            raise ValueError("cstool semantic surface is missing opcode ID")

        operands: Dict[int, Dict[str, Any]] = {}
        for line in lines:
            operand_match = re.match(
                r"operands\[(\d+)\]\.type: ([A-Z]+)(?: = (.+))?$", line
            )
            if operand_match:
                index = int(operand_match.group(1))
                operand_type = operand_match.group(2)
                operand_value = operand_match.group(3)
                if operand_type == "REG":
                    operands[index] = {
                        "kind": "register",
                        "register_id": self._riscv_register_id(operand_value or ""),
                        "access": "",
                    }
                elif operand_type == "IMM":
                    operands[index] = {
                        "kind": "immediate",
                        "value": self._parse_numeric_value(operand_value or "0"),
                        "access": "",
                    }
                elif operand_type == "MEM":
                    operands[index] = {
                        "kind": "memory",
                        "base": None,
                        "displacement": 0,
                        "access": "",
                    }
                elif operand_type == "CSR":
                    operands[index] = {
                        "kind": "immediate",
                        "value": self._riscv_csr_id(operand_value or ""),
                        "access": "",
                    }
                else:
                    raise ValueError(
                        f"unsupported cstool operand type `{operand_type}`"
                    )
                continue

            operand_match = re.match(r"operands\[(\d+)\]\.mem\.base: REG = (.+)$", line)
            if operand_match:
                index = int(operand_match.group(1))
                operands.setdefault(
                    index,
                    {"kind": "memory", "base": None, "displacement": 0, "access": ""},
                )["base"] = self._riscv_register_id(operand_match.group(2))
                continue

            operand_match = re.match(r"operands\[(\d+)\]\.mem\.disp: (.+)$", line)
            if operand_match:
                index = int(operand_match.group(1))
                operands.setdefault(
                    index,
                    {"kind": "memory", "base": None, "displacement": 0, "access": ""},
                )["displacement"] = self._parse_numeric_value(operand_match.group(2))
                continue

            operand_match = re.match(r"operands\[(\d+)\]\.access: ([A-Z| ]+)$", line)
            if operand_match:
                index = int(operand_match.group(1))
                operands.setdefault(index, {"kind": "unknown", "access": ""})[
                    "access"
                ] = operand_match.group(2).replace(" ", "")

        ordered_operands = []
        registers_read: List[int] = []
        registers_written: List[int] = []

        for index in sorted(operands):
            operand = operands[index]
            normalized_operand = self._normalize_cstool_operand(operand)
            ordered_operands.append(normalized_operand)

            access = operand.get("access", "")
            if normalized_operand["kind"] == "register":
                register_id = normalized_operand["register_id"]
                if "READ" in access:
                    registers_read.append(register_id)
                if "WRITE" in access:
                    registers_written.append(register_id)
            elif normalized_operand["kind"] == "memory":
                base = normalized_operand["base"]
                if base is not None:
                    registers_read.append(base)

        ordered_operands.extend(
            self._normalize_cstool_text_operands(lines[0], len(ordered_operands))
        )

        return {
            "opcode_id": self._normalize_opcode_name(id_match.group(1)),
            "operands": ordered_operands,
            "registers_read": registers_read,
            "registers_written": registers_written,
        }

    def _normalize_robustone_register(self, register: Dict[str, Any]) -> int:
        if register.get("architecture") != "riscv":
            raise ValueError(f"unexpected register architecture `{register}`")
        return int(register["id"])

    def _normalize_robustone_operand(
        self, operand: Dict[str, Any]
    ) -> Optional[Dict[str, Any]]:
        kind = operand.get("kind")
        if kind == "register":
            return {
                "kind": "register",
                "register_id": self._normalize_robustone_register(operand["register"]),
            }
        if kind == "immediate":
            return {"kind": "immediate", "value": int(operand["value"])}
        if kind == "memory":
            base = operand.get("base")
            return {
                "kind": "memory",
                "base": (
                    None if base is None else self._normalize_robustone_register(base)
                ),
                "displacement": int(operand["displacement"]),
            }
        if kind == "text":
            return {"kind": "text", "value": str(operand["value"]).strip().lower()}

        raise ValueError(f"unsupported robustone operand kind `{kind}`")

    def _normalize_cstool_operand(self, operand: Dict[str, Any]) -> Dict[str, Any]:
        kind = operand["kind"]
        if kind == "register":
            return {"kind": "register", "register_id": operand["register_id"]}
        if kind == "immediate":
            return {"kind": "immediate", "value": operand["value"]}
        if kind == "memory":
            return {
                "kind": "memory",
                "base": operand["base"],
                "displacement": operand["displacement"],
            }
        if kind == "text":
            return {"kind": "text", "value": operand["value"]}

        raise ValueError(f"unsupported normalized cstool operand `{kind}`")

    def _normalize_cstool_text_operands(
        self, instruction_line: str, decoded_operand_count: int
    ) -> List[Dict[str, Any]]:
        match = re.match(
            r"^[0-9a-f]+\s+(?:[0-9a-f]{2}\s+)+\S+(?:\s+(?P<operands>.+))?$",
            instruction_line,
        )
        if not match:
            return []

        operands = match.group("operands") or ""
        operand_tokens = [
            token.strip() for token in operands.split(",") if token.strip()
        ]
        if len(operand_tokens) <= decoded_operand_count:
            return []

        return [
            {"kind": "text", "value": token.lower()}
            for token in operand_tokens[decoded_operand_count:]
        ]

    def _normalize_opcode_name(self, opcode_id: Any) -> str:
        if opcode_id is None:
            raise ValueError("missing opcode identifier")
        return str(opcode_id).strip().lower().replace("_", ".")

    def _riscv_register_id(self, register_name: str) -> int:
        name = register_name.strip().lower()
        if name not in _RISCV_REGISTER_IDS:
            raise ValueError(f"unknown RISC-V register `{register_name}`")
        return _RISCV_REGISTER_IDS[name]

    def _riscv_csr_id(self, csr_name: str) -> int:
        name = csr_name.strip().lower()
        if name.startswith("0x"):
            return int(name, 16)
        if match := re.fullmatch(r"mhpmevent([3-9]|[12][0-9]|3[01])", name):
            return 0x320 + int(match.group(1))
        if match := re.fullmatch(r"mhpmcounter([3-9]|[12][0-9]|3[01])", name):
            return 0xB00 + int(match.group(1))
        if match := re.fullmatch(r"mhpmcounter([3-9]|[12][0-9]|3[01])h", name):
            return 0xB80 + int(match.group(1))
        if name not in _RISCV_CSR_IDS:
            raise ValueError(f"unknown RISC-V CSR `{csr_name}`")
        return _RISCV_CSR_IDS[name]

    def _parse_numeric_value(self, value: str) -> int:
        text = value.strip().lower()
        if not text:
            raise ValueError("empty numeric value")

        negative = text.startswith("-")
        if negative:
            text = text[1:]

        parsed = int(text, 16 if text.startswith("0x") else 10)
        if text.startswith("0x") and not negative and parsed >= (1 << 63):
            parsed -= 1 << 64

        return -parsed if negative else parsed

    def _combine_streams(self, primary: str, secondary: str, label: str) -> str:
        if not secondary:
            return primary
        secondary_block = f"{label}: {secondary}"
        return (
            primary
            if secondary_block in primary
            else self._append_message(primary, secondary_block)
        )

    def _append_message(self, current: str, message: str) -> str:
        if not current:
            return message
        if message in current:
            return current
        return f"{current} | {message}"
