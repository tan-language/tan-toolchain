// #todo Move to other crate.

use tan::context::Context;
use tancore::setup_lib;

// #todo Find a better name for this function.
pub fn import_prelude(context: &mut Context) {
    setup_lib(context);

    // Setup prelude as the ultimate scope.

    // let prelude_path = format!("{}/std/prelude", context.root_path);
    // let prelude = context.module_registry.get(&prelude_path).unwrap();

    // #todo Just use `prelude` scope as top-level scope.

    // #todo Could use a non-mut version of require_module.
    let prelude = context
        .get_module("prelude")
        .expect("prelude should be defined")
        .clone();

    // #todo Reuse `use` code here or extract helper!
    let bindings = prelude.scope.bindings.read().expect("poisoned lock");
    for (name, value) in bindings.iter() {
        context.top_scope.insert(name, value.clone());
    }

    // #todo Nasty, temp hack, makes older api functions work, CLEANUP!
    // #todo We need scope-stack visualization.
    // #todo Do we really need this intermediate scope? for some reason this is needed! investigate why!
    // context.scope = Arc::new(Scope::new(context.top_scope.clone()));
}
