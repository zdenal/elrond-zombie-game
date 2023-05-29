multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::crypto_kitties_proxy::{CryptoKittiesProxy, Kitty};
use crate::{storage::Storage, zombie_factory::ZombieFactory};

#[multiversx_sc::module]
pub trait ZombieFeeding: Storage + ZombieFactory + CryptoKittiesProxy + Helper {
    #[endpoint]
    fn feed_on_kitty(&self, zombie_id: usize, kitty_id: usize) {
        let kitties_sc_address = self.crypto_kitties_sc_address().get();

        self.kitty_proxy(kitties_sc_address)
            .get_kitty(kitty_id)
            .async_call()
            .with_callback(self.callbacks().get_kitty_callback(zombie_id))
            .call_and_exit();
    }

    #[view]
    fn is_ready(zombie_id: usize) -> bool {
        self.zombies(zombie_id).get() <= self.blockchain().get_block_timestamp()
    }

    #[callback]
    fn get_kitty_callback(
        &self,
        zombie_id: usize,
        #[call_result] result: ManagedAsyncCallResult<Kitty>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(Kitty { genes, .. }) => {
                self.feed_and_multiply(zombie_id, genes)
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }

    #[proxy]
    fn kitty_proxy(&self, address: ManagedAddress) -> crypto_kitties_proxy::Proxy<Self::Api>;

    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64) {
        self.check_ownership(zombie_id);

        require!(self.is_ready(), "Zombie is not ready");

        let zombie = self.zombies(zombie_id).get();

        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);

        let verified_target_dna = target_dna % max_dna_value;
        let new_dna = (zombie.dna + verified_target_dna) / 2;

        self.create_zombie(self.blockchain().get_caller(), "new_zombie", new_dna);
        self.trigger_cooldown(zombie_id);
    }

    fn trigger_cooldown(&self, zombie_id: usize) {
        let zombie = self.zombies(zombie_id).get();
        let ready_time = self.blockchain().get_block_timestamp() + self.cooldown_time().get();
        self.zombies(zombie_id)
            .update(|zombie| zombie.cooldown_time = ready_time)
    }
}
