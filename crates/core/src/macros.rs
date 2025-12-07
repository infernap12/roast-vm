use crate::class_file::constant_pool::ConstantPoolError;
use crate::value::Value;

#[macro_export]
macro_rules! store {
	($self:expr, l, $index:expr) => {{
		{
			let index: usize = $index;
			let value = $self.pop()?;
			trace!("\tStoring: {value} into local[{index}]");
			$self.vars.set(index , value);

			Ok(ExecutionResult::Continue)
		}
	}};
	($self:expr, d, $index:expr) => {
		store!($self, l, $index)
	};
	($self:expr, i, $index:expr) => {{
		{
			let index: usize = $index;
			let value = $self.pop()?;
			trace!("\tStoring: {value} into local[{index}]");
			$self.vars.set(index , value);
			Ok(ExecutionResult::Continue)
		}
	}};
	($self:expr, f, $index:expr) => {
		store!($self, i, $index)
	};
	($self:expr, a, $index:expr) => {
		store!($self, i, $index)
	};
}

#[macro_export]
macro_rules! load {
	($self:expr, i, $index:expr) => {{
		{
			let index: usize = $index;
			let value = $self.vars.get(index);
			trace!("\tLoading: local[{index}] - {value} onto stack");
			$self.stack.push(value.clone());
			Ok(ExecutionResult::Continue)
		}
	}};
	($self:expr, d, $index:expr) => {
		load!($self, l, $index)
	};
	($self:expr, l, $index:expr) => {
		load!($self, i, $index)
	};
	($self:expr, f, $index:expr) => {
		load!($self, i, $index)
	};
	($self:expr, a, $index:expr) => {
		load!($self, i, $index)
	};
}

#[macro_export]
macro_rules! pool_get_impl {
	($fn_name:ident => $result:ty, $variant:ident) => {
		fn $fn_name(&self, index: u16) -> Result<&$result, ConstantPoolError> {
			let cp_entry = self.get_constant(index)?;
			match cp_entry {
				ConstantPoolEntry::$variant(value) => Ok(value),
				_ => Err(ConstantPoolError(format!(
					"Expected {} constant at index {}",
					stringify!($variant),
					index
				))),
			}
		}
	};
}

#[macro_export]
macro_rules! if_int_zero {
    ($self:expr, $offset:expr, $op:tt) => {{
        let Value::Primitive(Primitive::Int(v)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        if v $op 0 {
            Ok(ExecutionResult::Advance($offset))
        } else {
            Ok(ExecutionResult::Continue)
        }
    }};
}

#[macro_export]
macro_rules! if_int_cmp {
    ($self:expr, $offset:expr, $op:tt) => {{
        let Value::Primitive(Primitive::Int(v2)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        let Value::Primitive(Primitive::Int(v1)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        if v1 $op v2 {
            Ok(ExecutionResult::Advance($offset))
        } else {
            Ok(ExecutionResult::Continue)
        }
    }};
}

#[macro_export]
macro_rules! float_cmp {
    ($self:expr, $prim:ident, $nan_result:expr) => {{
        let v2 = $self.pop()?;
        let v1 = $self.pop()?;

        let (Value::Primitive(Primitive::$prim(f1)), Value::Primitive(Primitive::$prim(f2))) = (&v1, &v2) else {
            return Err(VmError::StackError(format!(
                "{v1:?} or {v2:?} was not a {}",
                stringify!($prim).to_lowercase()
            )));
        };

        let result: i32 = if f1.is_nan() || f2.is_nan() {
            $nan_result
        } else {
            match f1.partial_cmp(f2) {
                Some(std::cmp::Ordering::Greater) => 1,
                Some(std::cmp::Ordering::Equal) => 0,
                _ => -1,
            }
        };

        $self.stack.push(Value::from(result));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! convert_simple {
    ($self:expr, $from:ident, $to:ty) => {{
        let Value::Primitive(Primitive::$from(v)) = $self.pop()? else {
            return Err(VmError::StackError(format!(
                "Expected {}", stringify!($from).to_lowercase()
            )));
        };
        $self.stack.push(Value::from(v as $to));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! convert_float_to_int {
    ($self:expr, $from:ident, $from_ty:ty, $to:ty) => {{
        let Value::Primitive(Primitive::$from(v)) = $self.pop()? else {
            return Err(VmError::StackError(format!(
                "Expected {}", stringify!($from).to_lowercase()
            )));
        };
        let result = if v.is_nan() {
            0
        } else if v >= <$to>::MAX as $from_ty {
            <$to>::MAX
        } else if v <= <$to>::MIN as $from_ty {
            <$to>::MIN
        } else {
            v as $to
        };
        $self.stack.push(Value::from(result));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! convert_int_narrow {
    ($self:expr, $to:ty) => {{
        let Value::Primitive(Primitive::Int(v)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        $self.stack.push(Value::from((v as $to) as i32));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! array_store {
    ($self:expr, $prim:ident, $arr:ident) => {{
        let Value::Primitive(Primitive::$prim(value)) = $self.pop()? else {
            return Err(VmError::StackError(format!(
                "Value was not {}", stringify!($prim).to_lowercase()
            )));
        };
        let Value::Primitive(Primitive::Int(index)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::$arr(arr)))) = $self.pop()? else {
            return Err(VmError::StackError(format!(
                "Expected {} array reference", stringify!($arr).to_lowercase()
            )));
        };
        arr.lock().unwrap().set(index, value);
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! array_store_cast {
    ($self:expr, $arr:ident, $cast:ty) => {{
        let Value::Primitive(Primitive::Int(value)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        let Value::Primitive(Primitive::Int(index)) = $self.pop()? else {
            return Err(VmError::stack_not_int());
        };
        let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::$arr(arr)))) = $self.pop()? else {
            return Err(VmError::StackError(format!(
                "Expected {} array reference", stringify!($arr).to_lowercase()
            )));
        };
        arr.lock().unwrap().set(index, value as $cast);
        Ok(ExecutionResult::Continue)
    }};
}

//math
#[macro_export]
macro_rules! binary_op {
    ($self:expr, $prim:ident, |$a:ident, $b:ident| $expr:expr) => {{
        let Value::Primitive(Primitive::$prim($b)) = $self.pop()? else {
            return Err(VmError::StackError(format!("Expected {}", stringify!($prim).to_lowercase())));
        };
        let Value::Primitive(Primitive::$prim($a)) = $self.pop()? else {
            return Err(VmError::StackError(format!("Expected {}", stringify!($prim).to_lowercase())));
        };
        $self.stack.push(Value::from($expr));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! unary_op {
    ($self:expr, $prim:ident, |$v:ident| $expr:expr) => {{
        let Value::Primitive(Primitive::$prim($v)) = $self.pop()? else {
            return Err(VmError::StackError(format!("Expected {}", stringify!($prim).to_lowercase())));
        };
        $self.stack.push(Value::from($expr));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! int_div_rem {
    ($self:expr, $prim:ident, $ty:ty, $op:tt) => {{
        let Value::Primitive(Primitive::$prim(v2)) = $self.pop()? else {
            return Err(VmError::StackError(format!("Expected {}", stringify!($prim).to_lowercase())));
        };
        let Value::Primitive(Primitive::$prim(v1)) = $self.pop()? else {
            return Err(VmError::StackError(format!("Expected {}", stringify!($prim).to_lowercase())));
        };
        if v2 == 0 {
            return Err(VmError::InvariantError("/ by zero".into()));
        }
        let result = if v1 == <$ty>::MIN && v2 == -1 {
            <$ty>::MIN $op 1  // MIN / -1 overflows, MIN % -1 = 0
        } else {
            v1 $op v2
        };
        $self.stack.push(Value::from(result));
        Ok(ExecutionResult::Continue)
    }};
}
#[macro_export]
macro_rules! shift_op {
    ($self:expr, $prim:ident, $mask:expr, |$val:ident, $shift:ident| $expr:expr) => {{
        let Value::Primitive(Primitive::Int(s)) = $self.pop()? else {
            return Err(VmError::StackError("Expected int for shift amount".into()));
        };
        let Value::Primitive(Primitive::$prim($val)) = $self.pop()? else {
            return Err(VmError::StackError(format!("Expected {}", stringify!($prim).to_lowercase())));
        };
        let $shift = (s & $mask) as u32;
        $self.stack.push(Value::from($expr));
        Ok(ExecutionResult::Continue)
    }};
}