use crate::ast::import_decl::Import;
use crate::ast::operator::Operator;

/// A type to denote infix declarations of the form:
/// infix <int> <symbol>.
#[derive(Clone, Debug, PartialEq)]
pub struct Infix {pub op : Operator, pub fix: i32, pub prec: i32}

#[macro_export]
macro_rules! infix {
    ($op:expr, $fix:expr, $prec:expr) => {
        TDeclaration::InfixDecl(Infix { op: $op, fix: $fix, prec: $prec })
    };
}

impl Infix {
    pub fn agdaify(&self) -> String {
        let mut code = String::new();

        let op = match self.op {
            Operator::Necessity => "□",
            Operator::Possibility => "◇",
            Operator::PropEq => "≡",
            Operator::Product => "×"
        };

        // Start the record declaration with its name and type.
        if self.fix == 1 {
            code.push_str(&format!("infix {} _{}", self.prec, op));
        }
        else if self.fix == 2 {
            code.push_str(&format!("infix {} _{}_", self.prec, op));
        }
        else {panic!("Invalid fixity.")}

        code
    }
}

