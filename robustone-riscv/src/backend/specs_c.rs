robustone_isa_macros::define_instructions! {
    arch = RiscV; module = c;
    insn C_UNIMP {
        mnemonic = "c.unimp";
        opcode_id = "C_UNIMP";
        pattern = robustone_isa::mask_value!(0xFFFF, 0x0000);
        format = &CIW_TYPE;
        operands = &[];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
        priority = 2;
    }
    insn C_ADDI4SPN {
        mnemonic = "c.addi4spn";
        opcode_id = "C_ADDI4SPN";
        pattern = robustone_isa::mask_value!(0xE003, 0x0000);
        format = &CIW_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmAddi4spn, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_LW {
        mnemonic = "c.lw";
        opcode_id = "C_LW";
        pattern = robustone_isa::mask_value!(0xE003, 0x4000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_SW {
        mnemonic = "c.sw";
        opcode_id = "C_SW";
        pattern = robustone_isa::mask_value!(0xE003, 0xC000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_LD {
        mnemonic = "c.ld";
        opcode_id = "C_LD";
        pattern = robustone_isa::mask_value!(0xE003, 0x6000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_SD {
        mnemonic = "c.sd";
        opcode_id = "C_SD";
        pattern = robustone_isa::mask_value!(0xE003, 0xE000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FLD {
        mnemonic = "c.fld";
        opcode_id = "C_FLD";
        pattern = robustone_isa::mask_value!(0xE003, 0x2000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FSD {
        mnemonic = "c.fsd";
        opcode_id = "C_FSD";
        pattern = robustone_isa::mask_value!(0xE003, 0xA000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCL, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FLW {
        mnemonic = "c.flw";
        opcode_id = "C_FLW";
        pattern = robustone_isa::mask_value!(0xE003, 0x6000);
        format = &CL_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FSW {
        mnemonic = "c.fsw";
        opcode_id = "C_FSW";
        pattern = robustone_isa::mask_value!(0xE003, 0xE000);
        format = &CS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::FprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCLW, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_ADDI {
        mnemonic = "c.addi";
        opcode_id = "C_ADDI";
        pattern = robustone_isa::mask_value!(0xE003, 0x0001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_JAL {
        mnemonic = "c.jal";
        opcode_id = "C_JAL";
        pattern = robustone_isa::mask_value!(0xE003, 0x2001);
        format = &CJ_TYPE;
        operands = &[
            robustone_isa::imm!(RiscVField::ImmCJ, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Call;
    }
    insn C_ADDIW {
        mnemonic = "c.addiw";
        opcode_id = "C_ADDIW";
        pattern = robustone_isa::mask_value!(0xE003, 0x2001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rd }];
    }
    insn C_LI {
        mnemonic = "c.li";
        opcode_id = "C_LI";
        pattern = robustone_isa::mask_value!(0xE003, 0x4001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_ADDI16SP {
        mnemonic = "c.addi16sp";
        opcode_id = "C_ADDI16SP";
        pattern = robustone_isa::mask_value!(0xEF83, 0x6101);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::imm!(RiscVField::ImmAddi16sp, robustone_isa::ImmediateTransform::SignExtend { bits: 10 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
        priority = 1;
    }
    insn C_LUI {
        mnemonic = "c.lui";
        opcode_id = "C_LUI";
        pattern = robustone_isa::mask_value!(0xE003, 0x6001);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rd }];
    }
    insn C_SRLI {
        mnemonic = "c.srli";
        opcode_id = "C_SRLI";
        pattern = robustone_isa::mask_value!(0xEC03, 0x8001);
        format = &CB_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCA, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_SRAI {
        mnemonic = "c.srai";
        opcode_id = "C_SRAI";
        pattern = robustone_isa::mask_value!(0xEC03, 0x8401);
        format = &CB_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCA, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_ANDI {
        mnemonic = "c.andi";
        opcode_id = "C_ANDI";
        pattern = robustone_isa::mask_value!(0xEC03, 0x8801);
        format = &CB_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCA, robustone_isa::ImmediateTransform::SignExtend { bits: 6 }, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_SUB {
        mnemonic = "c.sub";
        opcode_id = "C_SUB";
        pattern = robustone_isa::mask_value!(0xFC63, 0x8C01);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_XOR {
        mnemonic = "c.xor";
        opcode_id = "C_XOR";
        pattern = robustone_isa::mask_value!(0xFC63, 0x8C21);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_OR {
        mnemonic = "c.or";
        opcode_id = "C_OR";
        pattern = robustone_isa::mask_value!(0xFC63, 0x8C41);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_AND {
        mnemonic = "c.and";
        opcode_id = "C_AND";
        pattern = robustone_isa::mask_value!(0xFC63, 0x8C61);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_SUBW {
        mnemonic = "c.subw";
        opcode_id = "C_SUBW";
        pattern = robustone_isa::mask_value!(0xFC63, 0x9C01);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_ADDW {
        mnemonic = "c.addw";
        opcode_id = "C_ADDW";
        pattern = robustone_isa::mask_value!(0xFC63, 0x9C21);
        format = &CA_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::RdPrime, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs2Prime, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_J {
        mnemonic = "c.j";
        opcode_id = "C_J";
        pattern = robustone_isa::mask_value!(0xE003, 0xA001);
        format = &CJ_TYPE;
        operands = &[
            robustone_isa::imm!(RiscVField::ImmCJ, robustone_isa::ImmediateTransform::SignExtend { bits: 12 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Jump];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn C_BEQZ {
        mnemonic = "c.beqz";
        opcode_id = "C_BEQZ";
        pattern = robustone_isa::mask_value!(0xE003, 0xC001);
        format = &CB2_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCB, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn C_BNEZ {
        mnemonic = "c.bnez";
        opcode_id = "C_BNEZ";
        pattern = robustone_isa::mask_value!(0xE003, 0xE001);
        format = &CB2_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::GprPrime, RiscVField::Rs1Prime, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmCB, robustone_isa::ImmediateTransform::SignExtend { bits: 9 }, robustone_isa::ImmediateKind::PcRelative),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Branch];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Branch;
    }
    insn C_SLLI {
        mnemonic = "c.slli";
        opcode_id = "C_SLLI";
        pattern = robustone_isa::mask_value!(0xE003, 0x0002);
        format = &CI_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::Imm6, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_FLDSP {
        mnemonic = "c.fldsp";
        opcode_id = "C_FLDSP";
        pattern = robustone_isa::mask_value!(0xE003, 0x2002);
        format = &CI2_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmLdsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_LWSP {
        mnemonic = "c.lwsp";
        opcode_id = "C_LWSP";
        pattern = robustone_isa::mask_value!(0xE003, 0x4002);
        format = &CI2_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmLwsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rd }];
    }
    insn C_FLWSP {
        mnemonic = "c.flwsp";
        opcode_id = "C_FLWSP";
        pattern = robustone_isa::mask_value!(0xE003, 0x6002);
        format = &CI2_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmLwsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_LDSP {
        mnemonic = "c.ldsp";
        opcode_id = "C_LDSP";
        pattern = robustone_isa::mask_value!(0xE003, 0x6002);
        format = &CI2_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rd, robustone_isa::Access::Write),
            robustone_isa::imm!(RiscVField::ImmLdsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
        constraints = &[robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rd }];
    }
    insn C_FSDSP {
        mnemonic = "c.fsdsp";
        opcode_id = "C_FSDSP";
        pattern = robustone_isa::mask_value!(0xE003, 0xA002);
        format = &CSS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2C, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmSdsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_SWSP {
        mnemonic = "c.swsp";
        opcode_id = "C_SWSP";
        pattern = robustone_isa::mask_value!(0xE003, 0xC002);
        format = &CSS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2C, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmSwsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_FSWSP {
        mnemonic = "c.fswsp";
        opcode_id = "C_FSWSP";
        pattern = robustone_isa::mask_value!(0xE003, 0xE002);
        format = &CSS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Fpr, RiscVField::Rs2C, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmSwsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::CF;
        modes = ModeSet::Only(&[RiscVMode::RV32]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory, robustone_isa::InstructionGroup::Float];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_SDSP {
        mnemonic = "c.sdsp";
        opcode_id = "C_SDSP";
        pattern = robustone_isa::mask_value!(0xE003, 0xE002);
        format = &CSS_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2C, robustone_isa::Access::Read),
            robustone_isa::imm!(RiscVField::ImmSdsp, robustone_isa::ImmediateTransform::None, robustone_isa::ImmediateKind::Absolute),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::Only(&[RiscVMode::RV64]);
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Memory];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Memory;
    }
    insn C_JR {
        mnemonic = "c.jr";
        opcode_id = "C_JR";
        pattern = robustone_isa::mask_value!(0xF07F, 0x8002);
        format = &CR_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Jump];
        constraints = &[
            robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs1 },
            robustone_isa::EncodingConstraint::RegisterEquals { field: RiscVField::Rs2C, value: 0 },
        ];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Return;
        priority = 1;
    }
    insn C_MV {
        mnemonic = "c.mv";
        opcode_id = "C_MV";
        pattern = robustone_isa::mask_value!(0xF003, 0x8002);
        format = &CR_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2C, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        constraints = &[
            robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs1 },
            robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs2C },
        ];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
    insn C_EBREAK {
        mnemonic = "c.ebreak";
        opcode_id = "C_EBREAK";
        pattern = robustone_isa::mask_value!(0xFFFF, 0x9002);
        format = &CR_TYPE;
        operands = &[];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::System];
        manual = "RISC-V Unprivileged ISA Vol. I";
        priority = 2;
    }
    insn C_JALR {
        mnemonic = "c.jalr";
        opcode_id = "C_JALR";
        pattern = robustone_isa::mask_value!(0xF07F, 0x9002);
        format = &CR_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed, robustone_isa::InstructionGroup::Jump];
        constraints = &[
            robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs1 },
            robustone_isa::EncodingConstraint::RegisterEquals { field: RiscVField::Rs2C, value: 0 },
        ];
        manual = "RISC-V Unprivileged ISA Vol. I";
        effect = Call;
        priority = 1;
    }
    insn C_ADD {
        mnemonic = "c.add";
        opcode_id = "C_ADD";
        pattern = robustone_isa::mask_value!(0xF003, 0x9002);
        format = &CR_TYPE;
        operands = &[
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Write),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs1, robustone_isa::Access::Read),
            robustone_isa::reg!(RiscVRegisterClass::Gpr, RiscVField::Rs2C, robustone_isa::Access::Read),
        ];
        features = RiscVFeature::C;
        modes = ModeSet::All;
        groups = &[robustone_isa::InstructionGroup::Compressed];
        constraints = &[
            robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs1 },
            robustone_isa::EncodingConstraint::RegisterNotZero { field: RiscVField::Rs2C },
        ];
        manual = "RISC-V Unprivileged ISA Vol. I";
    }
}
