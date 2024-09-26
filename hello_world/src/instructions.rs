pub mod perpetuals;
pub mod processor;
pub mod vaults;

// pub use {perpetuals::*, vaults::*};
pub use perpetuals::instruction::PerpetualsInstruction;
pub use vaults::instruction::VaultInstruction;
