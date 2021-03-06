mod register;
pub use register::{Register16, Register16Input};

mod ram16k;
mod ram4k;
pub use ram4k::{MutRam4k, Ram4kInput};
mod ram512;
mod ram64;
mod ram8;
pub use ram16k::{MutRam16k, Ram16kInput};
