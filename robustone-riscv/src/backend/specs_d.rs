robustone_isa_macros::define_instructions! {
    arch = RiscV; module = d;
    insn FMADD_D {
        mnemonic = "fmadd.d";
        opcode_id = "FMADD_D";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x02000043);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FADD_D {
        mnemonic = "fadd.d";
        opcode_id = "FADD_D";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x02000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSQRT_D {
        mnemonic = "fsqrt.d";
        opcode_id = "FSQRT_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0x5A000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMIN_D {
        mnemonic = "fmin.d";
        opcode_id = "FMIN_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x2A000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FEQ_D {
        mnemonic = "feq.d";
        opcode_id = "FEQ_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA2002053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_S_D {
        mnemonic = "fcvt.s.d";
        opcode_id = "FCVT_S_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0x40100053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FLD {
        mnemonic = "fld";
        opcode_id = "FLD";
        pattern = robustone_isa::mask_value!(0x0000707F, 0x00003007);
        format = &I_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn FSD {
        mnemonic = "fsd";
        opcode_id = "FSD";
        pattern = robustone_isa::mask_value!(0x0000707F, 0x00003027);
        format = &S_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::mem_imm!(RiscVRegisterClass::Gpr, RiscVField::Rs1, RiscVField::Imm12S, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::Access::Write),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn FSUB_D {
        mnemonic = "fsub.d";
        opcode_id = "FSUB_D";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x0A000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMUL_D {
        mnemonic = "fmul.d";
        opcode_id = "FMUL_D";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x12000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FDIV_D {
        mnemonic = "fdiv.d";
        opcode_id = "FDIV_D";
        pattern = robustone_isa::mask_value!(0xFE00007F, 0x1A000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMSUB_D {
        mnemonic = "fmsub.d";
        opcode_id = "FMSUB_D";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x02000047);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FNMSUB_D {
        mnemonic = "fnmsub.d";
        opcode_id = "FNMSUB_D";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x0200004B);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FNMADD_D {
        mnemonic = "fnmadd.d";
        opcode_id = "FNMADD_D";
        pattern = robustone_isa::mask_value!(0x0600007F, 0x0200004F);
        format = &R4_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs3, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSGNJ_D {
        mnemonic = "fsgnj.d";
        opcode_id = "FSGNJ_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x22000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSGNJN_D {
        mnemonic = "fsgnjn.d";
        opcode_id = "FSGNJN_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x22001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FSGNJX_D {
        mnemonic = "fsgnjx.d";
        opcode_id = "FSGNJX_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x22002053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMAX_D {
        mnemonic = "fmax.d";
        opcode_id = "FMAX_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0x2A001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_D_S {
        mnemonic = "fcvt.d.s";
        opcode_id = "FCVT_D_S";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0x42000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FLT_D {
        mnemonic = "flt.d";
        opcode_id = "FLT_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA2001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FLE_D {
        mnemonic = "fle.d";
        opcode_id = "FLE_D";
        pattern = robustone_isa::mask_value!(0xFE00707F, 0xA2000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCLASS_D {
        mnemonic = "fclass.d";
        opcode_id = "FCLASS_D";
        pattern = robustone_isa::mask_value!(0xFFF0707F, 0xE2001053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_W_D {
        mnemonic = "fcvt.w.d";
        opcode_id = "FCVT_W_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC2000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_WU_D {
        mnemonic = "fcvt.wu.d";
        opcode_id = "FCVT_WU_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC2100053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_D_W {
        mnemonic = "fcvt.d.w";
        opcode_id = "FCVT_D_W";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD2000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_D_WU {
        mnemonic = "fcvt.d.wu";
        opcode_id = "FCVT_D_WU";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD2100053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMV_X_D {
        mnemonic = "fmv.x.d";
        opcode_id = "FMV_X_D";
        pattern = robustone_isa::mask_value!(0xFFF0707F, 0xE2000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FMV_D_X {
        mnemonic = "fmv.d.x";
        opcode_id = "FMV_D_X";
        pattern = robustone_isa::mask_value!(0xFFF0707F, 0xF2000053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_L_D {
        mnemonic = "fcvt.l.d";
        opcode_id = "FCVT_L_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC2200053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_LU_D {
        mnemonic = "fcvt.lu.d";
        opcode_id = "FCVT_LU_D";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xC2300053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_D_L {
        mnemonic = "fcvt.d.l";
        opcode_id = "FCVT_D_L";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD2200053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn FCVT_D_LU {
        mnemonic = "fcvt.d.lu";
        opcode_id = "FCVT_D_LU";
        pattern = robustone_isa::mask_value!(0xFFF0007F, 0xD2300053);
        format = &R_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::text!(RiscVField::Funct3, robustone_isa::ImmediateTransform::None),
        ];
        features = RiscVFeature::D;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}
