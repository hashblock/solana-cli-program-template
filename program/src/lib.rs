pub mod account_state;
pub mod error;
pub mod instruction;
pub mod processor;
pub use solana_program;

#[cfg(not(feature = "no-entrypoint"))]
mod entry_point;

solana_program::declare_id!("SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv");
