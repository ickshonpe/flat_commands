pub mod anti_features;

use bevy::ecs::system::*;
use bevy::prelude::*;

pub trait FlatCommands<'w, 's> {
    /// Set an existing entity as the root entity of the hierarchy
    fn root<'a>(&'a mut self, entity: Entity) -> RootCommands<'w, 's, 'a>;

    /// Create a new entity with components from bundle and
    /// make it the root of the hierarchy
    fn spawn_root<B>(&mut self, bundle: B) -> RootCommands<'w, 's, '_>
    where
        B: Bundle;

    /// Spawn an empty entity make it the root of the hierarchy
    fn spawn_empty_root(&mut self) -> RootCommands<'w, 's, '_>;
}

impl<'w, 's> FlatCommands<'w, 's> for Commands<'w, 's> {
    fn root<'a>(&'a mut self, entity: Entity) -> RootCommands<'w, 's, 'a> {
        RootCommands {
            entity,
            commands: self,
        }
    }

    fn spawn_root<'a, B>(&'a mut self, b: B) -> RootCommands<'w, 's, 'a>
    where
        B: Bundle,
    {
        let entity = self.spawn_bundle(b).id();
        RootCommands {
            entity,
            commands: self,
        }
    }

    fn spawn_empty_root<'a>(&'a mut self) -> RootCommands<'w, 's, 'a> {
        let entity = self.spawn().id();
        RootCommands {
            entity,
            commands: self,
        }
    }
}

pub struct RootCommands<'w, 's, 'a> {
    entity: Entity,
    commands: &'a mut Commands<'w, 's>,
}

pub struct ChildCommands<'w, 's, 'a> {
    root: Entity,
    parent: Entity,
    entity: Entity,
    commands: &'a mut Commands<'w, 's>,
}

pub trait ParentCommands<'w, 's, 'a> {
    /// returns the id of the root entity of the current hierarchy
    fn root_id(&self) -> Entity;

    /// returns the id of the current entity
    fn id(&self) -> Entity;

    /// returns Commands
    fn commands(&mut self) -> &mut Commands<'w, 's>;

    /// add a component to the current entity
    fn insert(&mut self, component: impl Component) -> &mut Self {
        let entity = self.id();
        self.commands().add(Insert { entity, component });
        self
    }

    /// add a bundle of components to the current entity
    fn insert_bundle(&mut self, bundle: impl Bundle) -> &mut Self {
        let entity = self.id();
        self.commands().add(InsertBundle { entity, bundle });
        self
    }

    /// Create a child entity with components from bundle
    /// with the current entity as its parent.
    /// Then make it the current entity
    fn with_child<T>(&mut self, bundle: T) -> ChildCommands<'w, 's, '_>
    where
        T: Bundle,
    {
        let child = self.commands().spawn_bundle(bundle).id();
        self.add_child(child);
        ChildCommands {
            root: self.root_id(),
            parent: self.id(),
            entity: child,
            commands: self.commands(),
        }
    }

    /// create a child entity and make it the current entity
    fn with_empty_child<T>(&mut self) -> ChildCommands<'w, 's, '_> {
        let child = self.commands().spawn().id();
        self.add_child(child);
        ChildCommands {
            root: self.root_id(),
            parent: self.id(),
            entity: child,
            commands: self.commands(),
        }
    }

    /// makes the current entity the root of a new local hierachy
    fn with_descendants(&mut self, local_root: impl FnOnce(&mut RootCommands)) -> &mut Self {
        local_root(&mut RootCommands {
            entity: self.id(),
            commands: self.commands(),
        });
        self
    }

    /// create a child entity but don't advance to it.
    fn add_child(&mut self, child: Entity) -> &mut Self {
        let entity = self.id();
        self.commands().add(AddChild {
            child,
            parent: entity,
        });
        self
    }

    /// create multiple children each spawned with the same bundle of components.
    fn with_child_batch<I>(&mut self, bundles_iter: I) -> &mut Self
    where
        I: IntoIterator + Send + Sync + 'static,
        I::Item: Bundle,
    {
        let parent = self.id();
        self.commands().with_child_batch(parent, bundles_iter);
        self
    }
}

pub trait ChildPusher<'w, 's, 'a>: ParentCommands<'w, 's, 'a> {
    /// Add multiple children to the current entity by id.
    fn push_children(&mut self, children: &[Entity]) -> &mut Self {
        let entity = self.id();
        self.commands().entity(entity).push_children(children);
        self
    }
}

impl<'w, 's, 'a> ParentCommands<'w, 's, 'a> for EntityCommands<'w, 's, 'a> {
    fn root_id(&self) -> Entity {
        self.id()
    }

    fn id(&self) -> Entity {
        self.id()
    }

    fn commands(&mut self) -> &mut Commands<'w, 's> {
        self.commands()
    }
}

impl<'w, 's, 'a> ParentCommands<'w, 's, 'a> for RootCommands<'w, 's, 'a> {
    fn root_id(&self) -> Entity {
        self.entity
    }

    fn id(&self) -> Entity {
        self.entity
    }

    fn commands(&mut self) -> &mut Commands<'w, 's> {
        self.commands
    }
}

impl<'w, 's, 'a> ChildPusher<'w, 's, 'a> for RootCommands<'w, 's, 'a> {}
impl<'w, 's, 'a> ChildPusher<'w, 's, 'a> for ChildCommands<'w, 's, 'a> {}

impl<'w, 's, 'a> ParentCommands<'w, 's, 'a> for ChildCommands<'w, 's, 'a> {
    fn root_id(&self) -> Entity {
        self.root
    }

    fn id(&self) -> Entity {
        self.entity
    }

    fn commands(&mut self) -> &mut Commands<'w, 's> {
        self.commands
    }
}

impl<'w, 's, 'a> ChildCommands<'w, 's, 'a> {
    /// Create a new entity with bundle components and the same parent as the current entity.
    /// Then move to the new entity.
    pub fn with_sibling<T>(&mut self, bundle: T) -> &mut Self
    where
        T: Bundle,
    {
        let parent = self.parent;
        let child = self.commands.spawn_bundle(bundle).id();
        self.commands.add(AddChild { child, parent });
        self.entity = child;
        self
    }

    /// Create a new empty entity and move to it.
    pub fn with_empty_sibling<T>(&mut self) -> &mut Self {
        let parent = self.parent;
        let child = self.commands.spawn().id();
        self.commands.add(AddChild { child, parent });
        self.entity = child;
        self
    }

    /// return the id of the current entities parent.
    pub fn parent_id(&self) -> Entity {
        self.parent
    }
}

pub struct SpawnChildBatch<I>
where
    I: IntoIterator,
    I::Item: Bundle,
{
    pub parent: Entity,
    pub bundles_iter: I,
}

impl<I> Command for SpawnChildBatch<I>
where
    I: IntoIterator + Send + Sync + 'static,
    I::Item: Bundle,
{
    fn write(self, world: &mut World) {
        let es = world
            .spawn_batch(self.bundles_iter)
            .collect::<Vec<Entity>>(); // not sure how to avoid this allocation.
        world.entity_mut(self.parent).push_children(&es);
    }
}

pub trait SpawnChildBatchExt {
    fn with_child_batch<I>(&mut self, parent: Entity, iter_bundles: I) -> &mut Self
    where
        I: IntoIterator + Send + Sync + 'static,
        I::Item: Bundle;
}

impl<'w, 's> SpawnChildBatchExt for Commands<'w, 's> {
    fn with_child_batch<I>(&mut self, parent: Entity, bundles_iter: I) -> &mut Self
    where
        I: IntoIterator + Send + Sync + 'static,
        I::Item: Bundle,
    {
        self.add(SpawnChildBatch {
            parent,
            bundles_iter,
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Component)]
    struct X;

    #[derive(Default, Component)]
    struct Y;

    #[derive(Default, Bundle)]
    struct Bx {
        x: X,
    }

    #[derive(Default, Bundle)]
    struct By {
        x: Y,
    }

    struct Root(Entity);
    struct Child(Entity);

    fn spawn_hierachy_1(mut commands: Commands) {
        let root_id = commands.spawn_root(Bx::default()).id();
        commands.insert_resource(Root(root_id));
        let child_id = commands.root(root_id).with_child(By::default()).id();
        commands.insert_resource(Child(child_id));
    }

    #[test]
    fn test_hierachy_1() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_1)
            .run(&mut world);
        assert_eq!(world.entities().len(), 2);
        let root_id = world.get_resource::<Root>().unwrap().0;
        let child_id = world.get_resource::<Child>().unwrap().0;
        assert!(world.entity(root_id).get::<X>().is_some());
        assert!(world.entity(child_id).get::<Y>().is_some());
        let children = world.entity(root_id).get::<Children>().unwrap();
        assert_eq!(children[0], child_id);
        let parent = world.entity(child_id).get::<Parent>().unwrap();
        assert_eq!(parent.0, root_id);
    }

    fn spawn_hierachy_2(mut commands: Commands) {
        let root_id = commands.spawn_root(Bx::default()).id();
        commands.insert_resource(Root(root_id));
        for _ in 0..10 {
            commands
                .root(root_id)
                .with_child(By::default())
                .with_child(By::default())
                .with_child(By::default())
                .with_sibling(By::default())
                .with_sibling(By::default());
        }
    }

    #[test]
    fn test_hierachy_2() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_2)
            .run(&mut world);
        assert_eq!(world.entities().len(), 51);
        let root_id = world.get_resource::<Root>().unwrap().0;
        assert!(world.entity(root_id).get::<X>().is_some());
        assert_eq!(world.entity(root_id).get::<Children>().unwrap().len(), 10);
        assert_eq!(world.query::<&X>().iter(&world).len(), 1);
        assert_eq!(world.query::<&Y>().iter(&world).len(), 50);
        assert_eq!(world.query::<&Children>().iter(&world).len(), 21);
    }

    fn spawn_hierachy_3(mut commands: Commands) {
        let root = commands.spawn_root(Bx::default()).id();
        let a = commands.root(root).with_child(Bx::default()).id();
        let b = commands.root(root).with_child(Bx::default()).id();
        commands
            .entity(a)
            .with_children(|builder| {
                builder.spawn_bundle(By::default()).id();
                builder.spawn_bundle(By::default()).id();
            })
            .commands()
            .entity(b)
            .with_children(|builder| {
                builder.spawn_bundle(By::default()).id();
                builder.spawn_bundle(By::default()).id();
            });
    }

    #[test]
    fn test_hierachy_3() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_3)
            .run(&mut world);
        assert_eq!(world.entities().len(), 7);
        world
            .query::<&Children>()
            .for_each(&world, |children| assert_eq!(children.len(), 2));
    }

    fn spawn_hierachy_4(mut commands: Commands) {
        commands
            .spawn_root(Bx::default())
            .with_child(Bx::default())
            .with_descendants(|local_root| {
                local_root
                    .with_child(By::default())
                    .with_sibling(By::default());
            })
            .with_sibling(Bx::default())
            .with_descendants(|local_root| {
                local_root
                    .with_child(By::default())
                    .with_sibling(By::default());
            });
    }

    #[test]
    fn test_heirachy_4() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_4)
            .run(&mut world);
    }

    fn spawn_heirachy_5(mut commands: Commands) {
        let mut root = commands.spawn_root(Bx::default());

        root.with_child(Bx::default())
            .with_child(Bx::default())
            .with_sibling(Bx::default());

        root.with_child(By::default())
            .with_child(By::default())
            .with_sibling(By::default());
    }

    #[test]
    fn test_hierachy_5() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_heirachy_5)
            .run(&mut world);
        assert_eq!(world.entities().len(), 7);
        assert_eq!(world.query::<(&X, &Parent)>().iter(&world).len(), 3);
        assert_eq!(world.query::<(&Y, &Parent)>().iter(&world).len(), 3);
        assert_eq!(world.query::<(&X, &Children)>().iter(&world).len(), 2);
        assert_eq!(world.query::<(&Y, &Children)>().iter(&world).len(), 1);
    }

    fn spawn_heirachy_6(mut commands: Commands) {
        let mut root = commands.spawn_root(Bx::default());
        let root_id = root.id();
        root.with_child(By::default())
            .with_sibling(By::default())
            .commands()
            .insert_resource(Root(root_id));
    }

    #[test]
    fn test_heirarchy_6() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_heirachy_6)
            .run(&mut world);
        assert_eq!(world.entities().len(), 3);
        let root_id = world.get_resource::<Root>().unwrap().0;
        let children = world.entity(root_id).get::<Children>().unwrap();
        assert_eq!(children.len(), 2);
        for child in children.iter() {
            let parent = world.entity(*child).get::<Parent>().unwrap().0;
            assert_eq!(root_id, parent);
        }
    }

    fn spawn_hierachy_7(mut commands: Commands) {
        let root_id = commands
            .spawn_root(Bx::default())
            .with_child_batch((0..10).map(|_| By::default()))
            .root_id();
        commands.insert_resource(Root(root_id));
    }

    #[test]
    fn test_hierachy_7() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_7)
            .run(&mut world);
        assert_eq!(world.entities().len(), 11);
        let root_id = world.get_resource::<Root>().unwrap().0;
        assert!(world.entity(root_id).get::<X>().is_some());
        assert_eq!(world.entity(root_id).get::<Children>().unwrap().len(), 10);
        assert_eq!(world.query::<&Y>().iter(&world).len(), 10);
    }

    fn spawn_hierachy_8(mut commands: Commands) {
        commands
            .spawn_root(Bx::default())
            .with_child(By::default())
            .with_child_batch((0..10).map(|_| By::default()));
    }

    #[test]
    fn test_hierachy_8() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_8)
            .run(&mut world);
        assert_eq!(world.entities().len(), 12);
    }

    fn spawn_hierachy_9(mut commands: Commands) {
        commands
            .spawn_root(Bx::default())
            .with_child(By::default())
            .with_child_batch((0..10).map(|_| By::default()))
            .insert(X::default())
            .with_child_batch((0..10).map(|_| By::default()))
            .with_sibling(By::default())
            .with_child_batch((0..10).map(|_| By::default()))
            .with_sibling(By::default())
            .insert(Y::default())
            .with_child(Bx::default())
            .with_child_batch((0..10).map(|_| By::default()))
            .with_descendants(|local_root| {
                local_root.with_child_batch((0..10).map(|_| By::default()));
            })
            .with_child_batch((0..10).map(|_| By::default()));
    }

    #[test]
    fn test_hierachy_9() {
        let mut world = World::default();
        SystemStage::single_threaded()
            .add_system(spawn_hierachy_9)
            .run(&mut world);
        assert_eq!(world.entities().len(), 65);
    }
}
