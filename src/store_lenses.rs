//! Dioxus Store lenses for SeaORM nested active relations.
//! although the concept is my own, this file was heavily AI-assisted.
//! TODO: either via AI or by hand, i need to refactor this god-awful, though functioning, file

use dioxus::prelude::*;
use sea_orm::{ActiveHasMany, ActiveHasOne, ActiveHasOneStoreExt, ActiveModelTrait, EntityTrait};

#[store(pub)]
impl<Lens, E: EntityTrait + 'static> Store<ActiveHasMany<E>, Lens>
where
    E::ActiveModelEx: 'static,
    Lens: Writable<Target = ActiveHasMany<E>, Storage = UnsyncStorage> + Copy + 'static,
{
    /// Iterate over the relation as independently reactive child stores.
    ///
    /// A `NotSet` relation becomes an empty, non-destructive `Append` relation.
    /// Creating the iterator tracks the collection length; each yielded store
    /// then tracks only its own child model.
    fn model(self) -> impl Iterator<Item = Store<E::ActiveModelEx>> {
        let mut relation = self;
        if matches!(&*relation.peek(), ActiveHasMany::NotSet) {
            relation.write().as_mut_vec();
        }

        let map: fn(&ActiveHasMany<E>) -> &Vec<E::ActiveModelEx> = |relation| match relation {
            ActiveHasMany::Replace(models) | ActiveHasMany::Append(models) => models,
            ActiveHasMany::NotSet => panic!("an ActiveHasMany model lens was reset to NotSet"),
            _ => panic!("unsupported ActiveHasMany variant"),
        };
        let map_mut: fn(&mut ActiveHasMany<E>) -> &mut Vec<E::ActiveModelEx> =
            ActiveHasMany::as_mut_vec;
        let models: Store<Vec<E::ActiveModelEx>, _> =
            relation.into_selector().map(map, map_mut).into();

        models
            .iter()
            .map(|model| {
                model
                    .into_selector()
                    .map_writer(|lens| lens.boxed_mut())
                    .into()
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[store(pub)]
impl<Lens, E: EntityTrait + 'static> Store<ActiveHasOne<E>, Lens>
where
    E::ActiveModelEx: 'static,
    Lens: Writable<Target = ActiveHasOne<E>, Storage = UnsyncStorage> + Copy + 'static,
{
    /// Return the related model as an independently reactive child store.
    fn model(self) -> Option<Store<E::ActiveModelEx>> {
        let model = self.set()?.as_deref()?;
        Some(
            model
                .into_selector()
                .map_writer(|lens| lens.boxed_mut())
                .into(),
        )
    }

    /// Auto-vivify: Return the related model, automatically initializing it
    /// to its Default state if it does not currently exist.
    fn model_or_default(self) -> Store<E::ActiveModelEx> {
        // Rebind `self` to be mutable inside the function body!
        let mut store = self;

        // If the relation doesn't exist yet, mutate the parent store to create it!
        if matches!(
            &*store.peek(),
            ActiveHasOne::NotSet | ActiveHasOne::Set(None)
        ) {
            *store.write() = ActiveHasOne::Set(Some(Box::new(E::ActiveModelEx::default())));
        }

        // Now we absolutely guarantee it exists, so we can unwrap safely.
        store
            .model()
            .expect("model_or_default failed to auto-initialize")
    }
}
