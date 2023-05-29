multiversx_sc::imports!();

use crate::{helper::ZombieHelper, storage, zombie::Zombie};

#[multiversx_sc::module]
pub trait ZombieFactory: storage::Storage + ZombieHelper {
    #[event("new_zombie_event")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        #[indexed] dna: u64,
        name: &ManagedBuffer,
    );

    fn create_zombie(&self, owner: ManagedAddress, name: ManagedBuffer, dna: u64) {
        self.zombies_count().update(|id| {
            let _event = self.new_zombie_event(*id, dna, &name);
            let cooldown_time =
                self.blockchain().get_block_timestamp() + self.cooldown_time().get();

            self.zombies(id).set(Zombie {
                name,
                dna,
                cooldown_time,
                level: 1u16,
                win_count: 0usize,
                loss_count: 0usize,
            });
            self.owned_zombies(&owner).insert(*id);
            self.zombie_owner(id).set(owner);
            *id += 1;
        });
    }

    #[endpoint]
    fn change_name(&self, zombie_id: usize, name: ManagedBuffer) {
        self.check_ownership(zombie_id);
        self.check_above_level(2, zombie_id);

        self.zombies(zombie_id).update(|zombie| zombie.name = name);
    }

    #[endpoint]
    fn change_dna(&self, zombie_id: usize, dna: u64) {
        self.check_ownership(zombie_id);
        self.check_above_level(20, zombie_id);

        self.zombies(zombie_id).update(|zombie| zombie.dna = dna);
    }

    #[endpoint]
    fn change_dna(&self, zombie_id: usize, dna: u64) {
        self.check_ownership(zombie_id);
        self.check_above_level(20, zombie_id);

        self.zombies(zombie_id).update(|zombie| zombie.dna = dna);
    }

    #[endpoint]
    fn attack(&self, zombie_id: usize, target_id: usize) {
        self.check_ownership(zombie_id);

        let probability = self.rand_mod(100u8);

        if probability <= self.attack_victory_probability().get() {
            let Zombie { dna: target_dna } = self.zombies(target_id).get();

            self.zombies(zombie_id).update(|zombie| {
                zombie.win_count += 1;
                zombie.level += 1;
            });

            self.feed_and_multiply(zombie_id, target_dna);

            self.zombies(target_id).update(|zombie| {
                zombie.loss_count += 1;
            });
        } else {
            self.zombies(target_id).update(|zombie| {
                zombie.win_count += 1;
            });
            self.zombies(zombie_id).update(|zombie| {
                zombie.loss_count += 1;
            });
            self.trigger_cooldown(zombie_id);
        }
    }

    #[payable("EGLD")]
    #[endpoint]
    fn level_up(&self, zombie_id: usize) {
        let fee = self.call_value().egld_value();
        require!(
            fee == self.level_up_fee().get(),
            "Payment must be must be 0.001 EGLD."
        );
        self.zombies(zombie_id).update(|zombie| zombie.level += 1);
        // TODO: missing in code source
        self.collected_fees().update(|fee| *fee += fee);
    }

    #[view]
    fn generate_random_dna(&self) -> u64 {
        let mut rand_source = RandomnessSource::new();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);

        rand_source.next_u64_in_range(0, max_dna_value)
    }
}
