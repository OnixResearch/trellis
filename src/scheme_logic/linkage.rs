//! Finite module export, import, and closure admission.

const GRAPH_ITEM_LIMIT: u32 = 4_096;

/// Finite module identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleId(pub u32);

/// Finite binding-name identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameId(pub u32);

/// One declared module and its provided names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleRecord {
    pub id: ModuleId,
    pub exports: Vec<NameId>,
}

/// One explicit required binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportEdge {
    pub from: ModuleId,
    pub target: ModuleId,
    pub name: NameId,
}

/// Complete finite require/provide graph.
// r[impl scheme-logic.module-graph]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleGraph {
    pub modules: Vec<ModuleRecord>,
    pub imports: Vec<ImportEdge>,
}

/// Deterministic transitive module closure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportClosure {
    pub modules: Vec<ModuleId>,
    pub consumed_budget: u32,
}

/// Stable module-graph rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleFault {
    TooManyModules,
    TooManyExports,
    TooManyImports,
    DuplicateModule,
    DuplicateExport,
    DuplicateImport,
    MissingSourceModule,
    MissingTargetModule,
    UnprovidedImport,
    BudgetTooLarge,
    BudgetExhausted,
}

/// Deterministic module-graph rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModuleFaultSite {
    pub class: ModuleFault,
    pub index: Option<u32>,
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn reject<T>(class: ModuleFault, index: Option<u32>) -> Result<T, ModuleFaultSite> {
    Err(ModuleFaultSite { class, index })
}

fn module(graph: &ModuleGraph, id: ModuleId) -> Option<&ModuleRecord> {
    graph.modules.iter().find(|candidate| candidate.id == id)
}

/// Admits unique modules, unique exports, and provided imports only.
// r[impl scheme-logic.module-graph]
pub fn admit_modules(graph: &ModuleGraph) -> Result<(), ModuleFaultSite> {
    let module_count = u32::try_from(graph.modules.len()).map_err(|_| ModuleFaultSite {
        class: ModuleFault::TooManyModules,
        index: None,
    })?;
    if module_count > GRAPH_ITEM_LIMIT {
        return reject(ModuleFault::TooManyModules, None);
    }
    let import_count = u32::try_from(graph.imports.len()).map_err(|_| ModuleFaultSite {
        class: ModuleFault::TooManyImports,
        index: None,
    })?;
    if import_count > GRAPH_ITEM_LIMIT {
        return reject(ModuleFault::TooManyImports, None);
    }

    let mut export_count = 0u32;
    for (index, record) in graph.modules.iter().enumerate() {
        let at = position(index);
        if graph.modules[..index].iter().any(|prior| prior.id == record.id) {
            return reject(ModuleFault::DuplicateModule, at);
        }
        let local_count = u32::try_from(record.exports.len()).map_err(|_| ModuleFaultSite {
            class: ModuleFault::TooManyExports,
            index: at,
        })?;
        export_count = export_count.checked_add(local_count).ok_or(ModuleFaultSite {
            class: ModuleFault::TooManyExports,
            index: at,
        })?;
        if export_count > GRAPH_ITEM_LIMIT {
            return reject(ModuleFault::TooManyExports, at);
        }
        for (name_index, name) in record.exports.iter().enumerate() {
            if record.exports[..name_index].contains(name) {
                return reject(ModuleFault::DuplicateExport, at);
            }
        }
    }

    for (index, import) in graph.imports.iter().enumerate() {
        let at = position(index);
        if graph.imports[..index].contains(import) {
            return reject(ModuleFault::DuplicateImport, at);
        }
        if module(graph, import.from).is_none() {
            return reject(ModuleFault::MissingSourceModule, at);
        }
        let Some(target) = module(graph, import.target) else {
            return reject(ModuleFault::MissingTargetModule, at);
        };
        if !target.exports.contains(&import.name) {
            return reject(ModuleFault::UnprovidedImport, at);
        }
    }
    Ok(())
}

/// Computes a visited-set closure within explicit module-expansion fuel.
// r[impl scheme-logic.module-graph]
pub fn import_closure(graph: &ModuleGraph, start: ModuleId, budget: u32) -> Result<ImportClosure, ModuleFaultSite> {
    admit_modules(graph)?;
    if module(graph, start).is_none() {
        return reject(ModuleFault::MissingSourceModule, None);
    }
    if budget > GRAPH_ITEM_LIMIT {
        return reject(ModuleFault::BudgetTooLarge, None);
    }

    let mut visited = Vec::with_capacity(graph.modules.len());
    visited.push(start);
    let mut cursor = 0usize;
    let mut consumed_budget = 0u32;
    while cursor < visited.len() {
        if consumed_budget >= budget {
            return reject(ModuleFault::BudgetExhausted, position(cursor));
        }
        let current = visited[cursor];
        for import in graph.imports.iter().filter(|edge| edge.from == current) {
            if !visited.contains(&import.target) {
                visited.push(import.target);
            }
        }
        cursor += 1;
        consumed_budget += 1;
    }
    Ok(ImportClosure {
        modules: visited,
        consumed_budget,
    })
}
