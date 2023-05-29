#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod crypto_kitties_proxy;
mod helper;
mod storage;
mod zombie;
mod zombie_factory;
mod zombie_feeding;

#[multiversx_sc::contract]
pub trait ZombiesContract:
    zombie_factory::ZombieFactory + zombie_feeding::ZombieFeeding + storage::Storage
{
    #[init]
    fn init(&self) {
        self.dna_digits().set(16u8);
        self.zombies_count().set(1usize);
        self.level_up_fee().set(1000000000000000u64);
        self.attack_victory_probability().set(70);
    }

    #[only_owner]
    #[endpoint]
    fn set_kitties_sc_address(&self, address: ManagedAddress) {
        self.crypto_kitties_sc_address().set(address);
        self.cooldown_time().set(86400u64);
    }

    #[only_owner]
    #[endpoint]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let collected_fees = self.collected_fees().get();
        self.send().direct_egld(&caller, &collected_fees);
        self.collected_fees().clear();
    }
}
