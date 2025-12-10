use std::fmt::{Display, Formatter};
use deku::DekuError;
use crate::class_file::constant_pool::ConstantPoolError;

#[derive(Debug)]
pub enum VmError {
	ConstantPoolError(String),
	StackError(String),
	InvariantError(String),
	DekuError(DekuError),
	LoaderError(String),
	ExecutionError,
	NativeError(String),
	Exception {
		message: String,
		stack_trace: Vec<StackTraceElement>,
	},
}

impl VmError {
	pub fn stack_not_int() -> Self {
		Self::InvariantError("Value on stack was not an int".to_string())
	}
}

impl Display for VmError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			VmError::ConstantPoolError(msg) => write!(f, "Constant pool error: {}", msg),
			VmError::StackError(msg) => write!(f, "Stack error: {}", msg),
			VmError::InvariantError(msg) => write!(f, "Invariant error: {}", msg),
			VmError::DekuError(err) => write!(f, "Deku error: {}", err),
			VmError::LoaderError(msg) => write!(f, "Loader error: {}", msg),
			VmError::ExecutionError => write!(f, "Execution error"),
			VmError::NativeError(msg) => write!(f, "Native error {msg}"),
			_ => { write!(f, "idk lol") }
		}
	}
}

impl From<ConstantPoolError> for VmError {
	fn from(value: ConstantPoolError) -> Self {
		Self::ConstantPoolError(value.to_string())
	}
}

impl From<DekuError> for VmError {
	fn from(value: DekuError) -> Self {
		Self::DekuError(value)
	}
}
#[derive(Debug)]
pub struct StackTraceElement {
	pub class: String,
	pub method: String,
	pub file: Option<String>,
	pub line: Option<u16>,
}

impl VmError {
	pub(crate) fn with_frame(self, elem: StackTraceElement) -> Self {
		match self {
			VmError::Exception { message, mut stack_trace } => {
				stack_trace.push(elem);
				VmError::Exception { message, stack_trace }
			}
			other => VmError::Exception {
				message: format!("{}", other),
				stack_trace: vec![elem],
			},
		}
	}
}
