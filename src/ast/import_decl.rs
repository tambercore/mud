use crate::ast::agda_expr::format_agda_type;
use crate::ast::theorem_decl::Theorem;

/// Type denoting Agda imports.
/// Consists of the package name (String), and the list of components to import.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Import {pub package : String, pub components : Vec<String>}

#[macro_export]
macro_rules! import {
    ($package:expr, $components:expr) => {
        TDeclaration::ImportDecl(
            Import {package: $package.to_string(), components: $components}
            )
    };
}

