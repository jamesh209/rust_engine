pub trait Component {}

pub struct ECS<T: Component> {
    entities: Vec<i64>,
    components: Vec<(i64, T)>,
}

//TODO: This can be optimised to iterate over components more effectively
impl<T: Component> ECS<T> {
    pub fn new() -> ECS<T> {
        let entities = Vec::new();
        let components = Vec::new();

        Self {
            entities,
            components,
        }
    }

    pub fn add_entity(&mut self, entity: i64) {
        self.entities.push(entity);
    }

    pub fn add_component(&mut self, entity: i64, component: T) {
        self.components.push((entity, component));
    }
}

pub trait System<T: Component> {
    fn execute(&mut self, ecs: &mut ECS<T>);
}

#[cfg(test)]
mod tests {
    use crate::mercury_ecs;
    use crate::mercury_ecs::System;

    struct TestIntComponent {
        integer: i32,
    }

    struct TestStringComponent {
        string: String,
    }

    enum TestComponentType {
        TestIntComponent(TestIntComponent),
        TestStringComponent(TestStringComponent),
    }

    impl mercury_ecs::Component for TestIntComponent {}
    impl mercury_ecs::Component for TestStringComponent {}
    impl mercury_ecs::Component for TestComponentType {}

    struct TestSystem {
        component_sum: i32,
    }

    impl mercury_ecs::System<TestComponentType> for TestSystem {
        fn execute(&mut self, ecs: &mut mercury_ecs::ECS<TestComponentType>) {
            self.component_sum = 0;
            for (_, component) in ecs.components.iter() {
                match component {
                    TestComponentType::TestIntComponent(test_int_component) => {
                        self.component_sum += test_int_component.integer;
                    }
                    _ => (),
                }
            }
        }
    }

    #[test]
    fn check_ecs() {
        let mut ecs = mercury_ecs::ECS::<TestComponentType>::new();

        let entity = 1;

        ecs.add_entity(entity);
        ecs.add_component(
            entity,
            TestComponentType::TestIntComponent(TestIntComponent { integer: 1 }),
        );
        ecs.add_component(
            entity,
            TestComponentType::TestStringComponent(TestStringComponent {
                string: String::from("tset"),
            }),
        );
        ecs.add_component(
            entity,
            TestComponentType::TestIntComponent(TestIntComponent { integer: 2 }),
        );

        let mut system = TestSystem { component_sum: 0 };
        system.execute(&mut ecs);
        assert_eq!(system.component_sum, 3);
    }
}
