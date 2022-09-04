use std::any::Any;
use std::marker::PhantomData;

use anyhow::{bail, Result};
use bevy::prelude::*;

use crate::prelude::{LayoutAnchor, LayoutConstraint, LayoutData, LayoutSize};

#[derive(Debug)]
pub struct ViewEntity<TE, T>
where
    TE: LayoutEnv,
    T: View<TE>,
{
    env: PhantomData<TE>,
    pub entity: Entity,
    pub view: T,
}

#[derive(Debug)]
pub struct DoLayoutEvent<TE, T>
where
    TE: LayoutEnv,
    T: View<TE>,
{
    env: PhantomData<TE>,
    pub entity: Entity,
    pub view: T,
    pub layout: LayoutData,
}
impl<TE, T> DoLayoutEvent<TE, T>
where
    TE: LayoutEnv + Send + Sync + 'static,
    T: View<TE> + Component,
{
    pub fn new(entity: Entity, view: &T, layout: &LayoutData) -> Self {
        Self {
            env: PhantomData,
            entity,
            view: view.clone(),
            layout: layout.clone(),
        }
    }
    pub fn on_layout_changed(query: LayoutChangedQuery<T>, mut evts: EventWriter<Self>) {
        for (entity, view, layout) in query.iter() {
            if layout.size.width > 0.0 && layout.size.height > 0.0 {
                if view.log_layout_changed() {
                    println!(
                        "<{}>::on_layout_changed({:#?})",
                        std::any::type_name::<T>(),
                        layout
                    );
                }
                evts.send(Self::new(entity, view, layout))
            }
        }
    }
    pub fn setup(app: &mut App) {
        app.add_event::<Self>();
        app.add_system(Self::on_layout_changed);
    }
}

pub type LayoutQuery<'w, 's, 'd, 't> = Query<'w, 's, (&'d mut LayoutData, &'t mut Transform)>;
pub type LayoutChangedQuery<'w, 's, 'v, 'd, T> =
    Query<'w, 's, (Entity, &'v T, &'d LayoutData), Changed<LayoutData>>;
pub type LayoutChangedWithChildrenQuery<'w, 's, 'v, 'd, 'c, T> =
    Query<'w, 's, (Entity, &'v T, &'d LayoutData, &'c Children), Changed<LayoutData>>;
pub type ViewQuery<'w, 's, 'p, 'v, T> = Query<'w, 's, (&'p Parent, Entity, &'v T)>;
pub type ViewAddedQuery<'w, 's, 'p, 'v, T> = Query<'w, 's, (&'p Parent, Entity, &'v T), Added<T>>;
pub type ViewRootQuery<'w, 's, 'v, T> = Query<'w, 's, (Entity, &'v T)>;
pub type ViewRootAddedQuery<'w, 's, 'v, T> = Query<'w, 's, (Entity, &'v T), Added<T>>;

pub trait LayoutEnv {
    fn query_child<TE, T>(
        &self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Result<ViewEntity<TE, T>>
    where
        TE: LayoutEnv,
        T: View<TE>,
    {
        for (parent, child, view) in view_query.iter() {
            if parent.get() == entity {
                return Ok(ViewEntity {
                    env: PhantomData,
                    entity: child,
                    view: view.clone(),
                });
            }
        }
        bail!("View Not Found: <{}>", std::any::type_name::<T>());
    }
    fn get_child<TE, T>(
        &self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Option<ViewEntity<TE, T>>
    where
        TE: LayoutEnv,
        T: View<TE>,
    {
        let result = self.query_child(view_query, entity);
        if let Err(err) = result {
            println!(
                "<LayoutEnv>.get_child<{}>() Not Found: {:?}",
                std::any::type_name::<T>(),
                err
            );
            return None;
        }
        result.ok()
    }
    fn get_children<TE, T>(
        &self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Vec<ViewEntity<TE, T>>
    where
        TE: LayoutEnv,
        T: View<TE>,
    {
        let mut children = Vec::new();
        for (parent, child, view) in view_query.iter() {
            if parent.get() == entity {
                children.push(ViewEntity {
                    env: PhantomData,
                    entity: child,
                    view: view.clone(),
                })
            }
        }
        children
    }
}

pub trait View<TE: LayoutEnv>: Any + Component + Clone + ToString {
    fn is_root(&self) -> bool {
        false
    }
    fn log_set_layout(&self) -> bool {
        false
    }
    fn log_layout_changed(&self) -> bool {
        false
    }
    fn pivot(&self) -> LayoutAnchor {
        if self.is_root() {
            LayoutAnchor::ROOT
        } else {
            LayoutAnchor::default()
        }
    }
    #[allow(unused_variables)]
    fn calc_size(&self, engine: &TE, constraint: LayoutConstraint) -> LayoutSize {
        constraint.max
    }
    fn calc_root_layout(&self, engine: &TE, constraint: LayoutConstraint) -> LayoutData {
        let size = self.calc_size(engine, constraint);
        let pivot = LayoutAnchor::ROOT;
        LayoutData::new(0, size, pivot, pivot, Vec2::ZERO)
    }
    fn set_layout_data(&self, layout_query: &mut LayoutQuery, entity: Entity, data: LayoutData) {
        if self.is_root() {
            println!("Should NOT call set_layout_data() for root views! {}", data);
            return;
        }
        let pivot = self.pivot();
        let need_adjust = pivot != data.pivot;
        let adjusted = if need_adjust {
            data.change_pivot(pivot)
        } else {
            data
        };
        if self.log_set_layout() {
            if need_adjust {
                println!(
                    "{}.set_layout_data(\n\t{} {} ->\n\t{}\n)",
                    self.to_string(),
                    data.pivot,
                    data.offset,
                    adjusted
                );
            } else {
                println!("{}.set_layout_data(\n\t{}\n)", self.to_string(), adjusted);
            }
        }
        match layout_query.get_mut(entity) {
            Ok((mut layout_data, mut transform)) => {
                *layout_data = adjusted;
                *transform = adjusted.transform();
            }
            Err(err) => {
                println!(
                    "{}.set_layout_data() Query Failed: {:?}",
                    self.to_string(),
                    err
                );
            }
        }
    }
}

impl<TE: LayoutEnv, T: View<TE>> ViewEntity<TE, T> {
    pub fn set_layout_data(&self, layout_query: &mut LayoutQuery, data: LayoutData) {
        self.view.set_layout_data(layout_query, self.entity, data)
    }
}
