pub use iso3166;

pub mod client;
pub mod common;
pub mod information;
pub mod internet_acquiring;
pub mod p2p_credit;
pub mod p2p_debit;
pub mod partner;
pub mod tokens;
pub mod verification;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
