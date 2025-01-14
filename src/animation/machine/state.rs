use crate::{
    animation::{
        machine::{EvaluatePose, ParameterContainer, PoseNode},
        AnimationContainer, AnimationPose,
    },
    core::algebra::Vector2,
    core::{
        pool::{Handle, Pool},
        reflect::prelude::*,
        visitor::prelude::*,
    },
};
use std::cell::Ref;

/// State is a final "container" for animation pose. It has backing pose node which provides a
/// set of values.
#[derive(Default, Debug, Visit, Clone, Reflect, PartialEq)]
pub struct State {
    pub position: Vector2<f32>,
    pub name: String,
    #[reflect(hidden)]
    pub root: Handle<PoseNode>,
}

impl State {
    /// Creates new instance of state with a given pose.
    pub fn new(name: &str, root: Handle<PoseNode>) -> Self {
        Self {
            position: Default::default(),
            name: name.to_owned(),
            root,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pose<'a>(&self, nodes: &'a Pool<PoseNode>) -> Option<Ref<'a, AnimationPose>> {
        nodes.try_borrow(self.root).map(|root| root.pose())
    }

    pub(super) fn update(
        &mut self,
        nodes: &Pool<PoseNode>,
        params: &ParameterContainer,
        animations: &AnimationContainer,
        dt: f32,
    ) {
        if let Some(root) = nodes.try_borrow(self.root) {
            root.eval_pose(nodes, params, animations, dt);
        }
    }
}
