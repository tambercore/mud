use crate::type_theory::function_type::FunctionType;
use crate::type_theory::simple_type::SimpleType;

pub enum Type {
    Simple(SimpleType),
    Func(FunctionType)
}