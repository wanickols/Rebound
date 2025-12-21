struct World {
    entities: Vec<State>,
    entity_index: HashMap<EntityId, usize>,

    players: Vec<EntityId>, // stable, fast access
}

impl World {
    fn remove_entity(&mut self, id: EntityId) {
        if let Some(&index) = self.entity_index.get(&id) {
            let last = self.entities.len() - 1;
            self.entities.swap_remove(index);

            self.entity_index.remove(&id);

            if index != last {
                let moved_id = self.entities[index].entity_id;
                self.entity_index.insert(moved_id, index);
            }
        }
    }
}
