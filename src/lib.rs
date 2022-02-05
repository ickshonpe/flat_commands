pub mod anti_features;

use bevy::ecs::system::*;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct FlatCommands<'w, 's> {
    commands: Commands<'w, 's>
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

pub trait GetFlatCommands<'w, 's> {
    fn flat_commands(self) -> FlatCommands<'w, 's>;
}

impl <'w, 's> GetFlatCommands<'w, 's> for Commands<'w, 's> {
    fn flat_commands(self) -> FlatCommands<'w, 's> {
        FlatCommands {
            commands: self
        }
    }
}

pub trait ParentCommander<'w, 's, 'a> {
    fn root_id(&self) -> Entity;
    fn id(&self) -> Entity;

    fn commands(&mut self) -> &mut Commands<'w, 's>;

    fn insert(&mut self, component: impl Component) -> &mut Self {
        let entity = self.id();
        self.commands().add(Insert {
            entity,
            component,
        });
        self
    }

    fn insert_bundle(&mut self, bundle: impl Bundle) -> &mut Self {
        let entity = self.id();
        self.commands().add(InsertBundle {
            entity,
            bundle,
        });
        self
    }

    fn spawn_child<T>(&mut self, bundle: T) -> ChildCommands<'w, 's, '_> 
    where
        T: Bundle
    {
        let child = self.commands().spawn_bundle(bundle).id();
        self.add_child(child);
        ChildCommands { 
            root: self.root_id(),
            parent: self.id(), 
            entity: child, 
            commands: self.commands()
        }
    }

    fn spawn_empty_child<T>(&mut self) -> ChildCommands<'w, 's, '_> {
        let child = self.commands().spawn().id();
        self.add_child(child);
        ChildCommands { 
            root: self.root_id(),
            parent: self.id(), 
            entity: child, 
            commands: self.commands()
        }
    }

    fn spawn_child_batch<I>(&mut self, bundles_iter: I) -> &mut Self 
    where
        I: IntoIterator + Send + Sync + 'static,
        I::Item: Bundle,
        <I as IntoIterator>::IntoIter: Send + Sync
    {
        let parent = self.id();
        self.commands().add( SpawnChildBatch {
            parent,
            bundles_iter,
        });
        self
    }

    fn with_descendants(&mut self, local_root: impl FnOnce(&mut RootCommands)) -> &mut Self {
        local_root(&mut RootCommands { entity: self.id(), commands: self.commands() });
        self
    }

    fn push_children(&mut self, children: &[Entity]) -> &mut Self {
        let entity = self.id();
        self.commands().entity(entity).push_children(children);
        self
    }

    fn add_child(&mut self, child: Entity) -> &mut Self {
        let entity = self.id();
        self.commands().add(AddChild { child, parent: entity });
        self
    }

    
}

impl<'w, 's> FlatCommands<'w, 's> {
    pub fn entity(&mut self, entity: Entity) -> RootCommands<'w, 's, '_> {
        RootCommands { 
            entity, 
            commands: &mut self.commands
        }
    }

    pub fn spawn_root<T>(&mut self, bundle: T) -> RootCommands<'w, 's, '_> 
    where 
        T: Bundle,
    {
        let entity = self.commands.spawn_bundle(bundle).id();
        RootCommands {
            entity,
            commands: &mut self.commands
        }
    }

    pub fn spawn_empty_root<T>(&mut self) -> RootCommands<'w, 's, '_> {
        let entity = self.commands.spawn().id();
        RootCommands {
            entity,
            commands: &mut self.commands
        }
    }

    pub fn insert_resource<T: Resource>(&mut self, resource: T) {
        self.commands.insert_resource(resource);
    }

    pub fn remove_resource<T: Resource>(&mut self) {
        self.commands.remove_resource::<T>();
    }

    pub fn take_commands(self) -> Commands<'w, 's> {
        self.commands
    }

    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }
}

impl <'w, 's, 'a> ParentCommander<'w, 's, 'a> for RootCommands<'w, 's, 'a> {
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

impl <'w, 's, 'a> ParentCommander<'w, 's, 'a> for ChildCommands<'w, 's, 'a> {
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
    pub fn with_sibling<T>(&mut self, bundle: T) -> ChildCommands<'w, 's, '_> 
    where 
        T: Bundle,
    {
        let parent = self.parent;
        let child = self.commands.spawn_bundle(bundle).id();
        self.commands.add(AddChild { child, parent });
        ChildCommands { 
            root: self.root,
            parent, 
            entity: child, 
            commands: &mut self.commands
        }
    }

    pub fn spawn_empty_sibling<T>(&mut self) -> ChildCommands<'w, 's, '_> {
        let parent = self.parent;
        let child = self.commands.spawn().id();
        self.commands.add(AddChild { child, parent });
        ChildCommands { 
            root: self.root,
            parent, 
            entity: child, 
            commands: &mut self.commands
        }
    }

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
    pub bundles_iter: I
}

// I: IntoIterator + Send + Sync + 'static,
// I::Item: Bundle,
// <I as IntoIterator>::IntoIter: Send + Sync

impl <I> Command for SpawnChildBatch<I> 
where
    I: IntoIterator + Send + Sync + 'static,
    I::Item: Bundle,
{
    fn write(self, world: &mut World) {
        let es = world.spawn_batch(self.bundles_iter).collect::<Vec<Entity>>();     
            // not sure how to get around this allocation.
        world.entity_mut(self.parent).push_children(&es);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    #[derive(Component)]
    struct X;

    #[derive(Default)]
    #[derive(Component)]
    struct Y;

    #[derive(Default)]
    #[derive(Bundle)]
    struct Bx { x: X }

    #[derive(Default)]
    #[derive(Bundle)]
    struct By { x: Y }

    struct Root(Entity);
    struct Child(Entity);

    fn spawn_hierachy_1(mut flat_commands: FlatCommands) {
        let root_id = flat_commands.spawn_root(Bx::default()).id();
        let mut commands = flat_commands.take_commands();
        commands.insert_resource(Root(root_id));
        let mut flat_commands = commands.flat_commands();
        let child_id = flat_commands.entity(root_id).spawn_child(By::default()).id();        
        flat_commands.commands().insert_resource(Child(child_id));
    }

    #[test]
    fn test_hierachy_1() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_1).run(&mut world);
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

    fn spawn_hierachy_2(mut flat_commands: FlatCommands) {
        let root_id = flat_commands.spawn_root(Bx::default()).id();
        flat_commands.commands().insert_resource(Root(root_id));
        for _ in 0..10 {
            flat_commands.entity(root_id)
            .spawn_child(By::default()) 
            .spawn_child(By::default())  
            .spawn_child(By::default())  
            .with_sibling(By::default()) 
            .with_sibling(By::default()); 
        }
    }

    #[test]
    fn test_hierachy_2() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_2).run(&mut world);
        assert_eq!(world.entities().len(), 51);
        let root_id = world.get_resource::<Root>().unwrap().0;
        assert!(world.entity(root_id).get::<X>().is_some());
        assert_eq!(world.entity(root_id).get::<Children>().unwrap().len(), 10);
        assert_eq!(world.query::<&X>().iter(&world).len(), 1);
        assert_eq!(world.query::<&Y>().iter(&world).len(), 50);
        assert_eq!(world.query::<&Children>().iter(&world).len(), 21);
    }

    fn spawn_hierachy_3(mut flat_commands: FlatCommands) {
        let root = flat_commands.spawn_root(Bx::default()).id();
        let a = flat_commands.entity(root).spawn_child(Bx::default()).id();
        let b = flat_commands.entity(root).spawn_child(Bx::default()).id();
        flat_commands.commands().entity(a)
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
        SystemStage::single_threaded().add_system(spawn_hierachy_3).run(&mut world);
        assert_eq!(world.entities().len(), 7);
        world.query::<&Children>().for_each(&world, |children| assert_eq!(children.len(), 2));
    }

    fn spawn_hierachy_4(mut flat_commands: FlatCommands) {
        flat_commands
        .spawn_root(Bx::default())
        .spawn_child(Bx::default())
        .with_descendants(|local_root| {
            local_root
            .spawn_child(By::default())
            .with_sibling(By::default());
        })
        .with_sibling(Bx::default())
        .with_descendants(|local_root| {
            local_root
            .spawn_child(By::default())
            .with_sibling(By::default());
        });
    }

    #[test]
    fn test_heirachy_4() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_4).run(&mut world);
    }

    fn spawn_heirachy_5(mut flat_commands: FlatCommands) {
        let mut root = flat_commands.spawn_root(Bx::default());
        
        root
        .spawn_child(Bx::default())
        .spawn_child(Bx::default())
        .with_sibling(Bx::default());

        root
        .spawn_child(By::default())
        .spawn_child(By::default())
        .with_sibling(By::default());
    }

    #[test]
    fn test_hierachy_5() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_heirachy_5).run(&mut world);
        assert_eq!(world.entities().len(), 7);
        assert_eq!(world.query::<(&X, &Parent)>().iter(&world).len(), 3);
        assert_eq!(world.query::<(&Y, &Parent)>().iter(&world).len(), 3);
        assert_eq!(world.query::<(&X, &Children)>().iter(&world).len(), 2);
        assert_eq!(world.query::<(&Y, &Children)>().iter(&world).len(), 1);
    }

    fn spawn_heirachy_6(mut flat_commands: FlatCommands) {
        let mut root = flat_commands.spawn_root(Bx::default());
        let root_id = root.id();
        root
        .spawn_child(By::default())
        .with_sibling(By::default())
        .commands().insert_resource(Root(root_id));
    }

    #[test]
    fn test_heirarchy_6() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_heirachy_6).run(&mut world);
        assert_eq!(world.entities().len(), 3);
        let root_id = world.get_resource::<Root>().unwrap().0;
        let children = world.entity(root_id).get::<Children>().unwrap();
        assert_eq!(children.len(), 2);
        for child in children.iter() {
            let parent = world.entity(*child).get::<Parent>().unwrap().0;
            assert_eq!(root_id, parent);

        }
    }

    fn spawn_hierachy_7(mut flat_commands: FlatCommands) {
        let root_id = flat_commands.spawn_root(Bx::default())
            .spawn_child_batch((0..10).map(|_| By::default()))
            .root_id();
        flat_commands.commands().insert_resource(Root(root_id));
    }

    #[test]
    fn test_hierachy_7() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_7).run(&mut world);
        assert_eq!(world.entities().len(), 11);
        let root_id = world.get_resource::<Root>().unwrap().0;
        assert!(world.entity(root_id).get::<X>().is_some());
        assert_eq!(world.entity(root_id).get::<Children>().unwrap().len(), 10);
        assert_eq!(world.query::<&Y>().iter(&world).len(), 10);
    }
}