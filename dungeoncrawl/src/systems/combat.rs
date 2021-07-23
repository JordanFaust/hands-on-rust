use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get the intended victims of an attack.
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            log(format!("Health before attack: {}", health.current));
            health.current -= 1;
            // If the victims health will reach zero remove it from the system
            if health.current < 1 {
                commands.remove(*victim);
            }
            log(format!("Health after attack: {}", health.current));
        }
        // Remove the WantToAttack message
        commands.remove(*message);
    });
}
