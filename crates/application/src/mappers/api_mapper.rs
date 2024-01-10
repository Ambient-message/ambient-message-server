pub trait ApiMapper<Entity, Presenter, Payload, Error> {
    // Map an Entity to a Presenter
    fn to_api(entity: Entity) -> Presenter;

    // Map a Payload to an Entity
    async fn to_entity(&self, payload: Payload) -> Result<Option<Entity>, Error>;
}
