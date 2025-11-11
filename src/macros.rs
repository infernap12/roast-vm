use crate::Value;

#[macro_export]
macro_rules! store {
    ($self:expr, l, $index:expr) => {
        {{
                let index: usize = $index;
                let value = $self.stack.pop().expect("Must contain value on stack");
                println!("\tStoring: {value:?} into local[{index}]");
                $self.vars[index] = value;
                $self.vars[index + 1] = Value::Reference(None);
                Ok(ExecutionResult::Continue)
            }}
    };
    ($self:expr, d, $index:expr) => {store!($self, l, $index)};
    ($self:expr, i, $index:expr) => {
        {{
                let index: usize = $index;
                let value = $self.stack.pop().expect("Must contain value on stack");
                println!("\tStoring: {value:?} into local[{index}]");
                $self.vars[index] = value;
                Ok(ExecutionResult::Continue)
            }}
    };
    ($self:expr, f, $index:expr) => {store!($self, i, $index)};
}

#[macro_export]
macro_rules! load {
    ($self:expr, i, $index:expr) => {
        {{
                let index: usize = $index;
                let value = $self.vars.get(index).expect("Local var to exist");
                println!("\tLoading: local[{index}] - {value:?} onto stack");
                $self.stack.push(value.clone());
                Ok(ExecutionResult::Continue)
            }}
    };
    ($self:expr, d, $index:expr) => {load!($self, l, $index)};
    ($self:expr, l, $index:expr) => {load!($self, i, $index)};
    ($self:expr, f, $index:expr) => {load!($self, i, $index)};
}
