multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Zombie<M: ManagedTypeApi> {
    pub dna: u64,
    pub name: ManagedBuffer<M>,
    pub level: u16,
    pub ready_time: u64,
    pub win_count: usize,
    pub loss_count: usize,
}
