use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use eyre::Context;

pub(crate) struct CompileUnit<'a> {
    pub(crate) output_module_name: &'a str,
    pub(crate) source: &'a str,
    pub(crate) source_label: String,
}

pub(crate) struct CompileOutput {
    pub(crate) output_module_name: String,
    pub(crate) erl_source: Option<String>,
    pub(crate) had_errors: bool,
}

pub(crate) fn compile_unit(
    unit: &CompileUnit<'_>,
    module_exports: &HashMap<String, Vec<String>>,
    analysis: &mondc::ProjectAnalysis,
    emit_warnings: bool,
) -> CompileOutput {
    let resolved = mondc::resolve_imports_for_source(unit.source, module_exports, analysis);
    let report = mondc::compile_with_imports_report_with_private_records(
        unit.output_module_name,
        unit.source,
        &unit.source_label,
        resolved.imports,
        module_exports,
        resolved.module_aliases,
        &resolved.imported_type_decls,
        &resolved.imported_extern_types,
        &resolved.imported_field_indices,
        &resolved.imported_private_records,
        &resolved.imported_schemes,
    );
    mondc::session::emit_compile_report_with_color(
        &report,
        emit_warnings,
        crate::ui::diagnostic_color_choice(),
    );

    let had_errors = report.has_errors() || report.output.is_none();
    let erl_source = if had_errors { None } else { report.output };

    CompileOutput {
        output_module_name: unit.output_module_name.to_string(),
        erl_source,
        had_errors,
    }
}

pub(crate) fn compile_units(
    units: &[CompileUnit<'_>],
    module_exports: &HashMap<String, Vec<String>>,
    analysis: &mondc::ProjectAnalysis,
    emit_warnings: bool,
) -> (Vec<CompileOutput>, bool) {
    let mut had_error = false;
    let mut outputs = Vec::with_capacity(units.len());

    for unit in units {
        let output = compile_unit(unit, module_exports, analysis, emit_warnings);
        had_error |= output.had_errors;
        outputs.push(output);
    }

    (outputs, had_error)
}

pub(crate) fn write_erl_output(
    erl_dir: &Path,
    output_module_name: &str,
    erl_source: &str,
) -> eyre::Result<PathBuf> {
    let erl_path = erl_dir.join(format!("{output_module_name}.erl"));
    std::fs::write(&erl_path, erl_source)
        .with_context(|| format!("could not write {}", erl_path.display()))?;
    Ok(erl_path)
}

pub(crate) fn dependency_module_exports(
    dependency_mods: &[(String, String, String)],
) -> HashMap<String, Vec<String>> {
    dependency_mods
        .iter()
        .map(|(user_name, _, source)| (user_name.clone(), mondc::exported_names(source)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_unit_returns_erl_for_valid_module() {
        let module_exports = HashMap::new();
        let analysis = mondc::build_project_analysis(&[], &[]).expect("analysis");
        let unit = CompileUnit {
            output_module_name: "main",
            source: "(let main {} 1)",
            source_label: "main.mond".to_string(),
        };

        let output = compile_unit(&unit, &module_exports, &analysis, true);
        assert!(!output.had_errors);
        assert!(output.erl_source.is_some());
    }

    #[test]
    fn compile_unit_reports_errors_for_invalid_module() {
        let module_exports = HashMap::new();
        let analysis = mondc::build_project_analysis(&[], &[]).expect("analysis");
        let unit = CompileUnit {
            output_module_name: "main",
            source: "(let main {} unknown)",
            source_label: "main.mond".to_string(),
        };

        let output = compile_unit(&unit, &module_exports, &analysis, true);
        assert!(output.had_errors);
        assert!(output.erl_source.is_none());
    }

    #[test]
    fn dependency_module_exports_scans_exported_names() {
        let dependency_mods = vec![(
            "io".to_string(),
            "mond_io".to_string(),
            "(pub let println {x} x)".to_string(),
        )];
        let exports = dependency_module_exports(&dependency_mods);
        assert_eq!(exports.get("io"), Some(&vec!["println".to_string()]));
    }
}
