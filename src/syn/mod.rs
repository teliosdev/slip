use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use std::sync::Arc;

mod enum_;
pub mod function;
pub mod item;
mod kind;
mod module;
mod roll;
mod root;
mod struct_;
mod unit;
mod use_;

pub use self::enum_::{Enum, EnumVariant};
pub use self::function::Function;
pub use self::item::Item;
pub use self::kind::Type;
pub use self::module::Module;
pub use self::roll::Roll;
pub use self::root::Root;
pub use self::struct_::{Struct, StructElement};
pub use self::unit::Unit;
pub use self::use_::{Use, UseTrail};

pub trait BasicNode: Sized {
    fn span(&self) -> Span;
}

pub trait Node: BasicNode {
    fn parse(stream: &mut TokenStream) -> Result<Self, Error>;
}

pub fn of(source: String) -> Result<Root, Error> {
    let mut set = crate::diag::DiagnosticSet::new();
    let file = set.create("(implicit)".to_owned(), source);
    let mut token_stream = crate::stream::TokenStream::new(file, &mut set);
    Root::parse(&mut token_stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::test::Bencher;

    const BASIC_SOURCE: &str = r#"
use Slip::List;

module Some::Program {
    fn some_func(): Slip::Int {
        return 42;
    }
}
    "#;

    #[bench]
    fn bench_basic_parse(b: &mut Bencher) {
        b.iter(|| of(BASIC_SOURCE.to_owned()).unwrap())
    }
}
