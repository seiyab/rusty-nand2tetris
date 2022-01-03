mod register;
pub use register::{Register16, RegisterInput};

mod ram16k;
mod ram4k;
mod ram512;
mod ram64;
mod ram8;
pub use ram16k::{MutRam16k, Ram16kInput};
