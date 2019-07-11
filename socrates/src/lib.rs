extern crate socrates_core;

pub use socrates_core::*;
pub mod common {
    pub use socrates_core::common::*;
}

pub mod module {
    pub use socrates_core::module::*;
}
pub mod service {
    pub use socrates_core::service::*;
}

pub mod component {
    pub use socrates_core::component::*;
}

extern crate socrates_macro;
pub use socrates_macro::*;
