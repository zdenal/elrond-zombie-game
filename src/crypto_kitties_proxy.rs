multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Kitty<M: ManagedTypeApi> {
    pub is_gestating: bool,
    pub is_ready: bool,
    pub cooldown_index: u64,
    pub next_action_at: u64,
    pub siring_with_id: u64,
    pub birth_time: u64,
    pub matron_id: u64,
    pub sire_id: u64,
    pub generation: u64,
    pub genes: u64,
}

#[multiversx_sc::proxy]
pub trait CryptoKittiesProxy {
    #[endpoint]
    fn get_kitty(&self, id: usize) -> Kitty;
}
