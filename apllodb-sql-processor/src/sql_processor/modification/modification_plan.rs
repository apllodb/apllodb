pub(crate) mod modification_plan_tree;

use self::modification_plan_tree::ModificationPlanTree;

/// Modification plan from which an executor can do its work deterministically.
#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct ModificationPlan {
    pub(crate) plan_tree: ModificationPlanTree,
}
