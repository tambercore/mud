
/// A type to denote literate segments in an AgdaFile.
/// LaTeX segments will be parsed as Strings.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Literate {pub content : String}

#[macro_export]
macro_rules! literate {
    (($tt:tt)) => {
        Literate { content: $tt }
    };
}