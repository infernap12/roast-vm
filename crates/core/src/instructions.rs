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

	// control
	#[deku(id = 0xa7)]
	goto(u16),

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
}
