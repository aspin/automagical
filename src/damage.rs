use bevy::prelude::*;
use bevy_rapier3d::physics::EventQueue;
use bevy_rapier3d::rapier::geometry::{ColliderSet, ContactEvent};
use crate::enemy::Enemy;
use crate::projectile::Projectile;

pub fn examine_collisions(
    mut commands: Commands,
    events: ResMut<EventQueue>,
    collider_set: Res<ColliderSet>,
    mut enemy_query: Query<(&mut Enemy)>,
    projectile_query: Query<(&Projectile)>,
) {
    while let Ok(contact_event) = events.contact_events.pop() {

        if let ContactEvent::Started(handle_1, handle_2) = contact_event {
            let entity_1 = Entity::from_bits(collider_set.get(handle_1).unwrap().user_data as u64);
            let entity_2 = Entity::from_bits(collider_set.get(handle_2).unwrap().user_data as u64);

            let mut maybe_enemy_entity: Option<Entity> = Option::None;
            if let Ok(_) = enemy_query.get_component::<Enemy>(entity_1) {
                maybe_enemy_entity.replace(entity_1);
            } else if let Ok(_) = enemy_query.get_component::<Enemy>(entity_2) {
                maybe_enemy_entity.replace(entity_2);
            }

            let mut maybe_projectile_entity: Option<Entity> = Option::None;
            if let Ok(_) = projectile_query.get_component::<Projectile>(entity_1) {
                maybe_projectile_entity.replace(entity_1);
            } else if let Ok(_) = projectile_query.get_component::<Projectile>(entity_2){
                maybe_projectile_entity.replace(entity_2);
            }

            if let Some(enemy_entity) = maybe_enemy_entity {
                if let Some(projectile) = maybe_projectile_entity {
                    let mut enemy = enemy_query.get_component_mut::<Enemy>(enemy_entity).unwrap();
                    let projectile = projectile_query.get_component::<Projectile>(projectile).unwrap();
                    enemy.take_damage(projectile.damage);
                    if enemy.hp <= 0 {
                        commands.despawn(enemy_entity);
                    } else {
                        println!("enemy hp {}", enemy.hp);
                    }
                }
            }
        }
    }
}