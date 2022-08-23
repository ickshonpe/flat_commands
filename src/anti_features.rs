use crate::*;
use bevy::prelude::*;

pub trait WithId<'w, 's, 'a>: ParentCommands<'w, 's, 'a> {
    /// Allows access to the current entity id in a closure.
    fn with_id(&mut self, mut f: impl FnMut(Entity)) -> &mut Self {
        f(self.id());
        self
    }
}

impl<'w, 's, 'a> WithId<'w, 's, 'a> for RootCommands<'w, 's, 'a> {}
impl<'w, 's, 'a> WithId<'w, 's, 'a> for ChildCommands<'w, 's, 'a> {}
impl<'w, 's, 'a> WithId<'w, 's, 'a> for EntityCommands<'w, 's, 'a> {}

#[cfg(test)]
mod tests {
    use crate::anti_features::*;

    #[derive(Default, Component)]
    struct X;

    #[derive(Default, Bundle)]
    struct B {
        x: X,
    }

    struct Ids(Vec<Entity>);

    fn spawn_hierachy_1(mut commands: Commands) {
        let mut ids = vec![];
        commands
            .spawn_root(B::default())
            .with_id(|id| ids.push(id))
            .with_child(B::default())
            .with_id(|id| ids.push(id))
            .with_child(B::default())
            .with_id(|id| ids.push(id))
            .with_descendants(|sub_root| {
                sub_root
                    .with_child(B::default())
                    .with_id(|id| ids.push(id))
                    .with_child(B::default())
                    .with_id(|id| ids.push(id));
            });

        commands.insert_resource(Ids(ids));
    }

    #[test]
    fn test_hierachy_1() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_1)
            .run(&mut world);
        assert_eq!(world.entities().len(), 5);
        let ids = &world.get_resource::<Ids>().unwrap().0;
        assert_eq!(ids.len(), 5);
        for (&parent, &child) in ids[..4].iter().zip(&ids[1..]) {
            println!("{:?}, {:?}", parent, child);
            assert_eq!(world.entity(parent).get::<Children>().unwrap()[0], child);
        }
        for (&child, &parent) in ids[1..].iter().zip(&ids[..4]) {
            assert_eq!(world.entity(child).get::<Parent>().unwrap().get(), parent);
        }
    }
}
