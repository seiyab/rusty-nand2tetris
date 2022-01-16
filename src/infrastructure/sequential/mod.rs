mod sequential_circuit;
pub use self::sequential_circuit::SequentialCircuit;

mod func_sc;
pub use self::func_sc::*;

mod array_sc;
pub use self::array_sc::*;

pub mod primitive;

mod feedback_sc;
pub use self::feedback_sc::*;

mod feedforward_sc;
pub use self::feedforward_sc::*;

mod mut_sc;
pub use self::mut_sc::*;

pub mod testing {
    pub use super::array_sc::testing::*;
    pub use super::feedforward_sc::testing::*;
}
