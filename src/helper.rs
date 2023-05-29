multiversx_sc::imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait ZombieHelper: storage::Storage {
    fn check_above_level(&self, level: u16, zombie_id: usize) {
        require!(
            self.zombies(zombie_id).get().level >= level,
            "Zombie is too low level"
        );
    }

    fn check_ownership(&self, zombie_id: usize) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.zombie_owner(zombie_id).get(),
            "Only the owner of the zombie can perform this operation"
        );
    }

    fn rand_mod(modulus: u8) -> u8 {
        let mut rand_source = RandomnessSource::new();
        rand_source.next_u8() % modulus
    }
}
