use std::fmt::{Display, Formatter};
use crate::attributes::ArrayType;
use deku_derive::DekuRead;

//noinspection SpellCheckingInspection
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(id_type = "u8", ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub enum Ops {
	// Constants
	#[deku(id = 0x00)]
	nop,
	#[deku(id = 0x01)]
	aconst_null,
	#[deku(id = 0x02)]
	iconst_m1,
	#[deku(id = 0x03)]
	iconst_0,
	#[deku(id = 0x04)]
	iconst_1,
	#[deku(id = 0x05)]
	iconst_2,
	#[deku(id = 0x06)]
	iconst_3,
	#[deku(id = 0x07)]
	iconst_4,
	#[deku(id = 0x08)]
	iconst_5,
	#[deku(id = 0x09)]
	lconst_0,
	#[deku(id = 0x0a)]
	lconst_1,
	#[deku(id = 0x0b)]
	fconst_0,
	#[deku(id = 0x0c)]
	fconst_1,
	#[deku(id = 0x0d)]
	fconst_2,
	#[deku(id = 0x0e)]
	dconst_0,
	#[deku(id = 0x0f)]
	dconst_1,
	#[deku(id = 0x10)]
	bipush(u8),
	#[deku(id = 0x11)]
	sipush(u16),
	#[deku(id = 0x12)]
	ldc(u8),
	#[deku(id = 0x13)]
	ldc_w(u16),
	#[deku(id = 0x14)]
	ldc2_w(u16),

	// Loads
	#[deku(id = 0x15)]
	iload(u8),
	#[deku(id = 0x16)]
	lload(u8),
	#[deku(id = 0x17)]
	fload(u8),
	#[deku(id = 0x18)]
	dload(u8),
	#[deku(id = 0x19)]
	aload(u8),
	#[deku(id = 0x1A)]
	iload_0,
	#[deku(id = 0x1B)]
	iload_1,
	#[deku(id = 0x1C)]
	iload_2,
	#[deku(id = 0x1D)]
	iload_3,
	#[deku(id = 0x1E)]
	lload_0,
	#[deku(id = 0x1F)]
	lload_1,
	#[deku(id = 0x20)]
	lload_2,
	#[deku(id = 0x21)]
	lload_3,
	#[deku(id = 0x22)]
	fload_0,
	#[deku(id = 0x23)]
	fload_1,
	#[deku(id = 0x24)]
	fload_2,
	#[deku(id = 0x25)]
	fload_3,
	#[deku(id = 0x26)]
	dload_0,
	#[deku(id = 0x27)]
	dload_1,
	#[deku(id = 0x28)]
	dload_2,
	#[deku(id = 0x29)]
	dload_3,
	#[deku(id = 0x2A)]
	aload_0,
	#[deku(id = 0x2B)]
	aload_1,
	#[deku(id = 0x2C)]
	aload_2,
	#[deku(id = 0x2D)]
	aload_3,
	#[deku(id = 0x2E)]
	iaload,
	#[deku(id = 0x2F)]
	laload,
	#[deku(id = 0x30)]
	faload,
	#[deku(id = 0x31)]
	daload,
	#[deku(id = 0x32)]
	aaload,
	#[deku(id = 0x33)]
	baload,
	#[deku(id = 0x34)]
	caload,
	#[deku(id = 0x35)]
	saload,

	// Stores
	#[deku(id = 0x36)]
	istore(u8),
	#[deku(id = 0x37)]
	lstore(u8),
	#[deku(id = 0x38)]
	fstore(u8),
	#[deku(id = 0x39)]
	dstore(u8),
	#[deku(id = 0x3A)]
	astore(u8),
	#[deku(id = 0x3B)]
	istore_0,
	#[deku(id = 0x3C)]
	istore_1,
	#[deku(id = 0x3D)]
	istore_2,
	#[deku(id = 0x3E)]
	istore_3,
	#[deku(id = 0x3F)]
	lstore_0,
	#[deku(id = 0x40)]
	lstore_1,
	#[deku(id = 0x41)]
	lstore_2,
	#[deku(id = 0x42)]
	lstore_3,
	#[deku(id = 0x43)]
	fstore_0,
	#[deku(id = 0x44)]
	fstore_1,
	#[deku(id = 0x45)]
	fstore_2,
	#[deku(id = 0x46)]
	fstore_3,
	#[deku(id = 0x47)]
	dstore_0,
	#[deku(id = 0x48)]
	dstore_1,
	#[deku(id = 0x49)]
	dstore_2,
	#[deku(id = 0x4A)]
	dstore_3,
	#[deku(id = 0x4B)]
	astore_0,
	#[deku(id = 0x4C)]
	astore_1,
	#[deku(id = 0x4D)]
	astore_2,
	#[deku(id = 0x4E)]
	astore_3,
	#[deku(id = 0x4F)]
	iastore,
	#[deku(id = 0x50)]
	lastore,
	#[deku(id = 0x51)]
	fastore,
	#[deku(id = 0x52)]
	dastore,
	#[deku(id = 0x53)]
	aastore,
	#[deku(id = 0x54)]
	bastore,
	#[deku(id = 0x55)]
	castore,
	#[deku(id = 0x56)]
	sastore,

	//stack
	#[deku(id = 0x57)]
	pop,
	#[deku(id = 0x58)]
	pop2,
	#[deku(id = 0x59)]
	dup,
	#[deku(id = 0x5a)]
	dup_x1,
	#[deku(id = 0x5b)]
	dup_x2,
	#[deku(id = 0x5c)]
	dup2,
	#[deku(id = 0x5d)]
	dup2_x1,
	#[deku(id = 0x5e)]
	dup2_x2,
	#[deku(id = 0x5f)]
	swap,

	//math
	#[deku(id = 0x60)]
	iadd,
	#[deku(id = 0x61)]
	ladd,
	#[deku(id = 0x62)]
	fadd,
	#[deku(id = 0x63)]
	dadd,
	#[deku(id = 0x64)]
	isub,
	#[deku(id = 0x65)]
	lsub,
	#[deku(id = 0x66)]
	fsub,
	#[deku(id = 0x67)]
	dsub,
	#[deku(id = 0x68)]
	imul,
	#[deku(id = 0x69)]
	lmul,
	#[deku(id = 0x6a)]
	fmul,
	#[deku(id = 0x6b)]
	dmul,
	#[deku(id = 0x6c)]
	idiv,
	#[deku(id = 0x6d)]
	ldiv,
	#[deku(id = 0x6e)]
	fdiv,
	#[deku(id = 0x6f)]
	ddiv,
	#[deku(id = 0x70)]
	irem,
	#[deku(id = 0x71)]
	lrem,
	#[deku(id = 0x72)]
	frem,
	#[deku(id = 0x73)]
	drem,
	#[deku(id = 0x74)]
	ineg,
	#[deku(id = 0x75)]
	lneg,
	#[deku(id = 0x76)]
	fneg,
	#[deku(id = 0x77)]
	dneg,
	#[deku(id = 0x78)]
	ishl,
	#[deku(id = 0x79)]
	lshl,
	#[deku(id = 0x7a)]
	ishr,
	#[deku(id = 0x7b)]
	lshr,
	#[deku(id = 0x7c)]
	iushr,
	#[deku(id = 0x7d)]
	lushr,
	#[deku(id = 0x7e)]
	iand,
	#[deku(id = 0x7f)]
	land,
	#[deku(id = 0x80)]
	ior,
	#[deku(id = 0x81)]
	lor,
	#[deku(id = 0x82)]
	ixor,
	#[deku(id = 0x83)]
	lxor,
	#[deku(id = 0x84)]
	iinc(u8, i8),

	//conversions
	#[deku(id = 0x85)]
	i2l,
	#[deku(id = 0x86)]
	i2f,
	#[deku(id = 0x87)]
	i2d,
	#[deku(id = 0x88)]
	l2i,
	#[deku(id = 0x89)]
	l2f,
	#[deku(id = 0x8a)]
	l2d,
	#[deku(id = 0x8b)]
	f2i,
	#[deku(id = 0x8c)]
	f2l,
	#[deku(id = 0x8d)]
	f2d,
	#[deku(id = 0x8e)]
	d2i,
	#[deku(id = 0x8f)]
	d2l,
	#[deku(id = 0x90)]
	d2f,
	#[deku(id = 0x91)]
	i2b,
	#[deku(id = 0x92)]
	i2c,
	#[deku(id = 0x93)]
	i2s,

	// comparisons
	#[deku(id = 0x94)]
	lcmp,
	#[deku(id = 0x95)]
	fcmpl,
	#[deku(id = 0x96)]
	fcmpg,
	#[deku(id = 0x97)]
	dcmpl,
	#[deku(id = 0x98)]
	dcmpg,
	#[deku(id = 0x99)]
	ifeq(i16),
	#[deku(id = 0x9a)]
	ifne(i16),
	#[deku(id = 0x9b)]
	iflt(i16),
	#[deku(id = 0x9c)]
	ifge(i16),
	#[deku(id = 0x9d)]
	ifgt(i16),
	#[deku(id = 0x9e)]
	ifle(i16),
	#[deku(id = 0x9f)]
	if_icmpeq(i16),
	#[deku(id = 0xa0)]
	if_icmpne(i16),
	#[deku(id = 0xa1)]
	if_icmplt(i16),
	#[deku(id = 0xa2)]
	if_icmpge(i16),
	#[deku(id = 0xa3)]
	if_icmpgt(i16),
	#[deku(id = 0xa4)]
	if_icmple(i16),
	#[deku(id = 0xa5)]
	if_acmpeq(i16),
	#[deku(id = 0xa6)]
	if_acmpne(i16),

	// control
	#[deku(id = 0xa7)]
	goto(i16),

	// discontinued
	#[deku(id = 0xa8)]
	jsr(u16),
	#[deku(id = 0xa9)]
	ret(u8),
	//
	#[deku(id = 0xaa)]
	tableswitch,
	#[deku(id = 0xab)]
	lookupswitch,
	#[deku(id = 0xac)]
	ireturn,
	#[deku(id = 0xad)]
	lreturn,
	#[deku(id = 0xae)]
	freturn,
	#[deku(id = 0xaf)]
	dreturn,
	#[deku(id = 0xb0)]
	areturn,
	// return
	#[deku(id = 0xb1)]
	return_void,

	// references
	#[deku(id = 0xB2)]
	getstatic(u16),
	#[deku(id = 0xB3)]
	putstatic(u16),
	#[deku(id = 0xB4)]
	getfield(u16),
	#[deku(id = 0xB5)]
	putfield(u16),
	#[deku(id = 0xB6)]
	invokevirtual(u16),
	#[deku(id = 0xB7)]
	invokespecial(u16),
	#[deku(id = 0xB8)]
	invokestatic(u16),
	// 4th byte always zero/0
	#[deku(id = 0xB9)]
	invokeinterface(u16, u8, u8),
	// 3rd, 4th must be zero/0
	#[deku(id = 0xBA)]
	invokedynamic(u16, u16),
	#[deku(id = 0xBB)]
	new(u16),
	#[deku(id = 0xBC)]
	newarray(ArrayType),
	#[deku(id = 0xBD)]
	anewarray(u16),
	#[deku(id = 0xBE)]
	arraylength,
	#[deku(id = 0xBF)]
	athrow,
	#[deku(id = 0xC0)]
	checkcast(u16),
	#[deku(id = 0xC1)]
	instanceof(u16),
	#[deku(id = 0xC2)]
	monitorenter,
	#[deku(id = 0xC3)]
	monitorexit,
	//extended
	#[deku(id = 0xC4)]
	wide,
	#[deku(id = 0xC5)]
	multianewarray(u16, u8),
	#[deku(id = 0xC6)]
	ifnull(i16),
	#[deku(id = 0xC7)]
	ifnonnull(i16),
	#[deku(id = 0xC8)]
	goto_w(i32),
	#[deku(id = 0xC9)]
	jsr_w(i32),
	// Reserved
	#[deku(id = 0xCA)]
	breakpoint,
	#[deku(id = 0xFE)]
	impdep1,
	#[deku(id = 0xFF)]
	impdep2,
}

impl Display for Ops {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			// Constants
			Ops::nop => write!(f, "nop"),
			Ops::aconst_null => write!(f, "aconst_null"),
			Ops::iconst_m1 => write!(f, "iconst_m1"),
			Ops::iconst_0 => write!(f, "iconst_0"),
			Ops::iconst_1 => write!(f, "iconst_1"),
			Ops::iconst_2 => write!(f, "iconst_2"),
			Ops::iconst_3 => write!(f, "iconst_3"),
			Ops::iconst_4 => write!(f, "iconst_4"),
			Ops::iconst_5 => write!(f, "iconst_5"),
			Ops::lconst_0 => write!(f, "lconst_0"),
			Ops::lconst_1 => write!(f, "lconst_1"),
			Ops::fconst_0 => write!(f, "fconst_0"),
			Ops::fconst_1 => write!(f, "fconst_1"),
			Ops::fconst_2 => write!(f, "fconst_2"),
			Ops::dconst_0 => write!(f, "dconst_0"),
			Ops::dconst_1 => write!(f, "dconst_1"),
			Ops::bipush(v) => write!(f, "bipush {}", v),
			Ops::sipush(v) => write!(f, "sipush {}", v),
			Ops::ldc(idx) => write!(f, "ldc #{}", idx),
			Ops::ldc_w(idx) => write!(f, "ldc_w #{}", idx),
			Ops::ldc2_w(idx) => write!(f, "ldc2_w #{}", idx),

			// Loads
			Ops::iload(idx) => write!(f, "iload {}", idx),
			Ops::lload(idx) => write!(f, "lload {}", idx),
			Ops::fload(idx) => write!(f, "fload {}", idx),
			Ops::dload(idx) => write!(f, "dload {}", idx),
			Ops::aload(idx) => write!(f, "aload {}", idx),
			Ops::iload_0 => write!(f, "iload_0"),
			Ops::iload_1 => write!(f, "iload_1"),
			Ops::iload_2 => write!(f, "iload_2"),
			Ops::iload_3 => write!(f, "iload_3"),
			Ops::lload_0 => write!(f, "lload_0"),
			Ops::lload_1 => write!(f, "lload_1"),
			Ops::lload_2 => write!(f, "lload_2"),
			Ops::lload_3 => write!(f, "lload_3"),
			Ops::fload_0 => write!(f, "fload_0"),
			Ops::fload_1 => write!(f, "fload_1"),
			Ops::fload_2 => write!(f, "fload_2"),
			Ops::fload_3 => write!(f, "fload_3"),
			Ops::dload_0 => write!(f, "dload_0"),
			Ops::dload_1 => write!(f, "dload_1"),
			Ops::dload_2 => write!(f, "dload_2"),
			Ops::dload_3 => write!(f, "dload_3"),
			Ops::aload_0 => write!(f, "aload_0"),
			Ops::aload_1 => write!(f, "aload_1"),
			Ops::aload_2 => write!(f, "aload_2"),
			Ops::aload_3 => write!(f, "aload_3"),
			Ops::iaload => write!(f, "iaload"),
			Ops::laload => write!(f, "laload"),
			Ops::faload => write!(f, "faload"),
			Ops::daload => write!(f, "daload"),
			Ops::aaload => write!(f, "aaload"),
			Ops::baload => write!(f, "baload"),
			Ops::caload => write!(f, "caload"),
			Ops::saload => write!(f, "saload"),

			// Stores
			Ops::istore(idx) => write!(f, "istore {}", idx),
			Ops::lstore(idx) => write!(f, "lstore {}", idx),
			Ops::fstore(idx) => write!(f, "fstore {}", idx),
			Ops::dstore(idx) => write!(f, "dstore {}", idx),
			Ops::astore(idx) => write!(f, "astore {}", idx),
			Ops::istore_0 => write!(f, "istore_0"),
			Ops::istore_1 => write!(f, "istore_1"),
			Ops::istore_2 => write!(f, "istore_2"),
			Ops::istore_3 => write!(f, "istore_3"),
			Ops::lstore_0 => write!(f, "lstore_0"),
			Ops::lstore_1 => write!(f, "lstore_1"),
			Ops::lstore_2 => write!(f, "lstore_2"),
			Ops::lstore_3 => write!(f, "lstore_3"),
			Ops::fstore_0 => write!(f, "fstore_0"),
			Ops::fstore_1 => write!(f, "fstore_1"),
			Ops::fstore_2 => write!(f, "fstore_2"),
			Ops::fstore_3 => write!(f, "fstore_3"),
			Ops::dstore_0 => write!(f, "dstore_0"),
			Ops::dstore_1 => write!(f, "dstore_1"),
			Ops::dstore_2 => write!(f, "dstore_2"),
			Ops::dstore_3 => write!(f, "dstore_3"),
			Ops::astore_0 => write!(f, "astore_0"),
			Ops::astore_1 => write!(f, "astore_1"),
			Ops::astore_2 => write!(f, "astore_2"),
			Ops::astore_3 => write!(f, "astore_3"),
			Ops::iastore => write!(f, "iastore"),
			Ops::lastore => write!(f, "lastore"),
			Ops::fastore => write!(f, "fastore"),
			Ops::dastore => write!(f, "dastore"),
			Ops::aastore => write!(f, "aastore"),
			Ops::bastore => write!(f, "bastore"),
			Ops::castore => write!(f, "castore"),
			Ops::sastore => write!(f, "sastore"),

			// Stack
			Ops::pop => write!(f, "pop"),
			Ops::pop2 => write!(f, "pop2"),
			Ops::dup => write!(f, "dup"),
			Ops::dup_x1 => write!(f, "dup_x1"),
			Ops::dup_x2 => write!(f, "dup_x2"),
			Ops::dup2 => write!(f, "dup2"),
			Ops::dup2_x1 => write!(f, "dup2_x1"),
			Ops::dup2_x2 => write!(f, "dup2_x2"),
			Ops::swap => write!(f, "swap"),

			// Math
			Ops::iadd => write!(f, "iadd"),
			Ops::ladd => write!(f, "ladd"),
			Ops::fadd => write!(f, "fadd"),
			Ops::dadd => write!(f, "dadd"),
			Ops::isub => write!(f, "isub"),
			Ops::lsub => write!(f, "lsub"),
			Ops::fsub => write!(f, "fsub"),
			Ops::dsub => write!(f, "dsub"),
			Ops::imul => write!(f, "imul"),
			Ops::lmul => write!(f, "lmul"),
			Ops::fmul => write!(f, "fmul"),
			Ops::dmul => write!(f, "dmul"),
			Ops::idiv => write!(f, "idiv"),
			Ops::ldiv => write!(f, "ldiv"),
			Ops::fdiv => write!(f, "fdiv"),
			Ops::ddiv => write!(f, "ddiv"),
			Ops::irem => write!(f, "irem"),
			Ops::lrem => write!(f, "lrem"),
			Ops::frem => write!(f, "frem"),
			Ops::drem => write!(f, "drem"),
			Ops::ineg => write!(f, "ineg"),
			Ops::lneg => write!(f, "lneg"),
			Ops::fneg => write!(f, "fneg"),
			Ops::dneg => write!(f, "dneg"),
			Ops::ishl => write!(f, "ishl"),
			Ops::lshl => write!(f, "lshl"),
			Ops::ishr => write!(f, "ishr"),
			Ops::lshr => write!(f, "lshr"),
			Ops::iushr => write!(f, "iushr"),
			Ops::lushr => write!(f, "lushr"),
			Ops::iand => write!(f, "iand"),
			Ops::land => write!(f, "land"),
			Ops::ior => write!(f, "ior"),
			Ops::lor => write!(f, "lor"),
			Ops::ixor => write!(f, "ixor"),
			Ops::lxor => write!(f, "lxor"),
			Ops::iinc(idx, val) => write!(f, "iinc {}, {}", idx, val),

			// Conversions
			Ops::i2l => write!(f, "i2l"),
			Ops::i2f => write!(f, "i2f"),
			Ops::i2d => write!(f, "i2d"),
			Ops::l2i => write!(f, "l2i"),
			Ops::l2f => write!(f, "l2f"),
			Ops::l2d => write!(f, "l2d"),
			Ops::f2i => write!(f, "f2i"),
			Ops::f2l => write!(f, "f2l"),
			Ops::f2d => write!(f, "f2d"),
			Ops::d2i => write!(f, "d2i"),
			Ops::d2l => write!(f, "d2l"),
			Ops::d2f => write!(f, "d2f"),
			Ops::i2b => write!(f, "i2b"),
			Ops::i2c => write!(f, "i2c"),
			Ops::i2s => write!(f, "i2s"),

			// Comparisons
			Ops::lcmp => write!(f, "lcmp"),
			Ops::fcmpl => write!(f, "fcmpl"),
			Ops::fcmpg => write!(f, "fcmpg"),
			Ops::dcmpl => write!(f, "dcmpl"),
			Ops::dcmpg => write!(f, "dcmpg"),
			Ops::ifeq(off) => write!(f, "ifeq {}", off),
			Ops::ifne(off) => write!(f, "ifne {}", off),
			Ops::iflt(off) => write!(f, "iflt {}", off),
			Ops::ifge(off) => write!(f, "ifge {}", off),
			Ops::ifgt(off) => write!(f, "ifgt {}", off),
			Ops::ifle(off) => write!(f, "ifle {}", off),
			Ops::if_icmpeq(off) => write!(f, "if_icmpeq {}", off),
			Ops::if_icmpne(off) => write!(f, "if_icmpne {}", off),
			Ops::if_icmplt(off) => write!(f, "if_icmplt {}", off),
			Ops::if_icmpge(off) => write!(f, "if_icmpge {}", off),
			Ops::if_icmpgt(off) => write!(f, "if_icmpgt {}", off),
			Ops::if_icmple(off) => write!(f, "if_icmple {}", off),
			Ops::if_acmpeq(off) => write!(f, "if_acmpeq {}", off),
			Ops::if_acmpne(off) => write!(f, "if_acmpne {}", off),

			// Control
			Ops::goto(off) => write!(f, "goto {}", off),
			Ops::jsr(off) => write!(f, "jsr {}", off),
			Ops::ret(idx) => write!(f, "ret {}", idx),
			Ops::tableswitch => write!(f, "tableswitch"),
			Ops::lookupswitch => write!(f, "lookupswitch"),
			Ops::ireturn => write!(f, "ireturn"),
			Ops::lreturn => write!(f, "lreturn"),
			Ops::freturn => write!(f, "freturn"),
			Ops::dreturn => write!(f, "dreturn"),
			Ops::areturn => write!(f, "areturn"),
			Ops::return_void => write!(f, "return"),

			// References
			Ops::getstatic(idx) => write!(f, "getstatic #{}", idx),
			Ops::putstatic(idx) => write!(f, "putstatic #{}", idx),
			Ops::getfield(idx) => write!(f, "getfield #{}", idx),
			Ops::putfield(idx) => write!(f, "putfield #{}", idx),
			Ops::invokevirtual(idx) => write!(f, "invokevirtual #{}", idx),
			Ops::invokespecial(idx) => write!(f, "invokespecial #{}", idx),
			Ops::invokestatic(idx) => write!(f, "invokestatic #{}", idx),
			Ops::invokeinterface(idx, count, _) => write!(f, "invokeinterface #{}, {}", idx, count),
			Ops::invokedynamic(idx, _) => write!(f, "invokedynamic #{}", idx),
			Ops::new(idx) => write!(f, "new #{}", idx),
			Ops::newarray(atype) => write!(f, "newarray {:?}", atype),
			Ops::anewarray(idx) => write!(f, "anewarray #{}", idx),
			Ops::arraylength => write!(f, "arraylength"),
			Ops::athrow => write!(f, "athrow"),
			Ops::checkcast(idx) => write!(f, "checkcast #{}", idx),
			Ops::instanceof(idx) => write!(f, "instanceof #{}", idx),
			Ops::monitorenter => write!(f, "monitorenter"),
			Ops::monitorexit => write!(f, "monitorexit"),

			// Extended
			Ops::wide => write!(f, "wide"),
			Ops::multianewarray(idx, dims) => write!(f, "multianewarray #{}, {}", idx, dims),
			Ops::ifnull(off) => write!(f, "ifnull {}", off),
			Ops::ifnonnull(off) => write!(f, "ifnonnull {}", off),
			Ops::goto_w(off) => write!(f, "goto_w {}", off),
			Ops::jsr_w(off) => write!(f, "jsr_w {}", off),

			// Reserved
			Ops::breakpoint => write!(f, "breakpoint"),
			Ops::impdep1 => write!(f, "impdep1"),
			Ops::impdep2 => write!(f, "impdep2"),
		}
	}
}