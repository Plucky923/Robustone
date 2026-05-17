robustone_isa_macros::define_instructions! {
    arch = RiscV; module = f;
    insn FLW {
        mnemonic = "flw";
        opcode_id = "FLW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2007);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn FSW {
        mnemonic = "fsw";
        opcode_id = "FSW";
        pattern = robustone_isa::mask_value!(0x0000_707F, 0x0000_2027);
        format = &S_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Write),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn FMADD_S {
        mnemonic = "fmadd.s";
        opcode_id = "FMADD_S";
        pattern = robustone_isa::mask_value!(0x0600_007F, 0x0000_0043);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FADD_S {
        mnemonic = "fadd.s";
        opcode_id = "FADD_S";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x00000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSUB_S {
        mnemonic = "fsub.s";
        opcode_id = "FSUB_S";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x08000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMUL_S {
        mnemonic = "fmul.s";
        opcode_id = "FMUL_S";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x10000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FDIV_S {
        mnemonic = "fdiv.s";
        opcode_id = "FDIV_S";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x18000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Arithmetic];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSQRT_S {
        mnemonic = "fsqrt.s";
        opcode_id = "FSQRT_S";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0x58000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMIN_S {
        mnemonic = "fmin.s";
        opcode_id = "FMIN_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x28000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FEQ_S {
        mnemonic = "feq.s";
        opcode_id = "FEQ_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA0002053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMV_W_X {
        mnemonic = "fmv.w.x";
        opcode_id = "FMV_W_X";
        pattern = robustone_isa::mask_value!(0xFFF0707F, 0xF0000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMV_X_W {
        mnemonic = "fmv.x.w";
        opcode_id = "FMV_X_W";
        pattern = robustone_isa::mask_value!(0xFFF0707F, 0xE0000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_W_S {
        mnemonic = "fcvt.w.s";
        opcode_id = "FCVT_W_S";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC0000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_S_W {
        mnemonic = "fcvt.s.w";
        opcode_id = "FCVT_S_W";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD0000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCLASS_S {
        mnemonic = "fclass.s";
        opcode_id = "FCLASS_S";
        pattern = robustone_isa::mask_value!(0xFFF0707F, 0xE0001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMAX_S {
        mnemonic = "fmax.s";
        opcode_id = "FMAX_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x28001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSGNJ_S {
        mnemonic = "fsgnj.s";
        opcode_id = "FSGNJ_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x20000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSGNJN_S {
        mnemonic = "fsgnjn.s";
        opcode_id = "FSGNJN_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x20001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSGNJX_S {
        mnemonic = "fsgnjx.s";
        opcode_id = "FSGNJX_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x20002053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FLE_S {
        mnemonic = "fle.s";
        opcode_id = "FLE_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA0000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FLT_S {
        mnemonic = "flt.s";
        opcode_id = "FLT_S";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA0001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMSUB_S {
        mnemonic = "fmsub.s";
        opcode_id = "FMSUB_S";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x00000047);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FNMSUB_S {
        mnemonic = "fnmsub.s";
        opcode_id = "FNMSUB_S";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x0000004B);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FNMADD_S {
        mnemonic = "fnmadd.s";
        opcode_id = "FNMADD_S";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x0000004F);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_WU_S {
        mnemonic = "fcvt.wu.s";
        opcode_id = "FCVT_WU_S";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC0100053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_S_WU {
        mnemonic = "fcvt.s.wu";
        opcode_id = "FCVT_S_WU";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD0100053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_L_S {
        mnemonic = "fcvt.l.s";
        opcode_id = "FCVT_L_S";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC0200053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_LU_S {
        mnemonic = "fcvt.lu.s";
        opcode_id = "FCVT_LU_S";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC0300053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_S_L {
        mnemonic = "fcvt.s.l";
        opcode_id = "FCVT_S_L";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD0200053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_S_LU {
        mnemonic = "fcvt.s.lu";
        opcode_id = "FCVT_S_LU";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD0300053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::F;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}
