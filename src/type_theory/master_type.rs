use crate::type_theory::function_type::FunctionType;
use crate::type_theory::simple_type::SimpleType;
use crate::type_theory::utility::Agdaify;

pub enum Type {
    Simple(SimpleType),
    Func(FunctionType)
}

impl Agdaify for Type {
    fn to_agda(&self) -> String {
        match self {
            Type::Simple(smp) => {smp.to_agda()}
            Type::Func(fun) => {fun.to_agda()}
        }
    }
}