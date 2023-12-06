use di::ServiceCollection;


pub struct ServiceBuilder {
    pub services: ServiceCollection,
}

impl ServiceBuilder {
    pub fn new() -> Self {
        Self {
            services: ServiceCollection::new(),
        }
    }

    pub fn build(self) -> ServiceCollection {
        self.services
    }
}