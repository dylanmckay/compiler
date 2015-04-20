
pub use self::ty::{Type,TypeTrait};

pub use self::array::Array;
pub use self::float::Float;
pub use self::function::Function;
pub use self::integer::Integer;
pub use self::label::Label;
pub use self::strukt::Struct;
pub use self::vector::Vector;
pub use self::void::Void;

pub mod ty;

pub mod array;
pub mod float;
pub mod function;
pub mod integer;
pub mod label;
pub mod strukt;
pub mod vector;
pub mod void;
