/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use super::super::ArtifactGeneratedTypes;
use super::content_section::{
    CommentAnnotationsSection, ContentSection, ContentSections, DocblockSection, GenericSection,
};
use crate::config::{Config, ProjectConfig};
use common::NamedItem;
use graphql_ir::{FragmentDefinition, OperationDefinition};
use relay_codegen::{build_request_params, Printer, QueryID, TopLevelStatement, CODEGEN_CONSTANTS};
use relay_transforms::{
    is_operation_preloadable, ReactFlightLocalComponentsMetadata, RelayClientComponentMetadata,
    RelayDataDrivenDependencyMetadata, ASSIGNABLE_DIRECTIVE,
};
use relay_typegen::{
    generate_fragment_type_exports_section, generate_named_validator_export,
    generate_operation_type_exports_section, generate_split_operation_type_exports_section,
    TypegenConfig, TypegenLanguage,
};
use schema::SDLSchema;
use signedsource::SIGNING_TOKEN;
use std::fmt::{Error as FmtError, Result as FmtResult, Write};
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
pub fn generate_updatable_query(
    config: &Config,
    project_config: &ProjectConfig,
    printer: &mut Printer<'_>,
    schema: &SDLSchema,
    reader_operation: &OperationDefinition,
    typegen_operation: &OperationDefinition,
    source_hash: String,
    skip_types: bool,
) -> Result<Vec<u8>, FmtError> {
    let operation_fragment = FragmentDefinition {
        name: reader_operation.name,
        variable_definitions: reader_operation.variable_definitions.clone(),
        selections: reader_operation.selections.clone(),
        used_global_variables: Default::default(),
        directives: reader_operation.directives.clone(),
        type_condition: reader_operation.type_,
    };

    let mut content_sections = ContentSections::default();

    // -- Begin Docblock Section --
    content_sections.push(ContentSection::Docblock(generate_docblock_section(
        config,
        project_config,
        vec![],
    )?));
    // -- End Docblock Section --

    // -- Begin Disable Lint Section --
    content_sections.push(ContentSection::Generic(generate_disable_lint_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Disable Lint Section --

    // -- Begin Use Strict Section --
    content_sections.push(ContentSection::Generic(generate_use_strict_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Use Strict Section --

    // -- Begin Types Section --
    let mut section = GenericSection::default();
    let generated_types =
        ArtifactGeneratedTypes::from_updatable_query(typegen_operation, skip_types);

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "/*::")?;
    }

    write_import_type_from(
        &project_config.typegen_config.language,
        &mut section,
        generated_types.imported_types,
        "relay-runtime",
    )?;

    if !skip_types {
        write!(
            section,
            "{}",
            generate_operation_type_exports_section(
                typegen_operation,
                reader_operation,
                schema,
                project_config,
            )
        )?;
    }

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "*/")?;
    }

    content_sections.push(ContentSection::Generic(section));
    // -- End Types Section --

    // -- Begin Query Node Section --
    let mut section = GenericSection::default();
    let request = printer.print_updatable_query(schema, &operation_fragment);
    write_variable_value_with_type(
        &project_config.typegen_config.language,
        &mut section,
        "node",
        generated_types.ast_type,
        &request,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Query Node Section --

    // -- Begin Query Node Hash Section --
    let mut section = GenericSection::default();
    write_source_hash(
        config,
        &project_config.typegen_config.language,
        &mut section,
        &source_hash,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Query Node Hash Section --

    // -- Begin Export Query Node Section --
    let mut section = GenericSection::default();
    write_export_generated_node(
        &project_config.typegen_config,
        &mut section,
        "node",
        generated_types.exported_type,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Export Query Node Section --

    content_sections.into_signed_bytes()
}

#[allow(clippy::too_many_arguments)]
pub fn generate_operation(
    config: &Config,
    project_config: &ProjectConfig,
    printer: &mut Printer<'_>,
    schema: &SDLSchema,
    normalization_operation: &OperationDefinition,
    reader_operation: &OperationDefinition,
    typegen_operation: &OperationDefinition,
    source_hash: String,
    text: &Option<String>,
    id_and_text_hash: &Option<QueryID>,
    skip_types: bool,
) -> Result<Vec<u8>, FmtError> {
    let mut request_parameters = build_request_params(normalization_operation);
    if id_and_text_hash.is_some() {
        request_parameters.id = id_and_text_hash;
    } else {
        request_parameters.text = text.clone();
    };
    let operation_fragment = FragmentDefinition {
        name: reader_operation.name,
        variable_definitions: reader_operation.variable_definitions.clone(),
        selections: reader_operation.selections.clone(),
        used_global_variables: Default::default(),
        directives: reader_operation.directives.clone(),
        type_condition: reader_operation.type_,
    };

    let mut content_sections = ContentSections::default();

    // -- Begin Docblock Section --
    let v = match id_and_text_hash {
        Some(QueryID::Persisted { text_hash, .. }) => vec![format!("@relayHash {}", text_hash)],
        _ => vec![],
    };
    content_sections.push(ContentSection::Docblock(generate_docblock_section(
        config,
        project_config,
        v,
    )?));
    // -- End Docblock Section --

    // -- Begin Disable Lint Section --
    content_sections.push(ContentSection::Generic(generate_disable_lint_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Disable Lint Section --

    // -- Begin Use Strict Section --
    content_sections.push(ContentSection::Generic(generate_use_strict_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Use Strict Section --

    // -- Begin Metadata Annotations Section --
    let mut section = CommentAnnotationsSection::default();
    if let Some(QueryID::Persisted { id, .. }) = &request_parameters.id {
        writeln!(section, "@relayRequestID {}", id)?;
    }
    if project_config.variable_names_comment {
        write!(section, "@relayVariables")?;
        for variable_definition in &normalization_operation.variable_definitions {
            write!(section, " {}", variable_definition.name.item)?;
        }
        writeln!(section)?;
    }
    let data_driven_dependency_metadata =
        RelayDataDrivenDependencyMetadata::find(&operation_fragment.directives);
    if let Some(data_driven_dependency_metadata) = data_driven_dependency_metadata {
        write_data_driven_dependency_annotation(&mut section, data_driven_dependency_metadata)?;
    }
    if let Some(flight_metadata) =
        ReactFlightLocalComponentsMetadata::find(&operation_fragment.directives)
    {
        write_react_flight_server_annotation(&mut section, flight_metadata)?;
    }
    let relay_client_component_metadata =
        RelayClientComponentMetadata::find(&operation_fragment.directives);
    if let Some(relay_client_component_metadata) = relay_client_component_metadata {
        write_react_flight_client_annotation(&mut section, relay_client_component_metadata)?;
    }
    content_sections.push(ContentSection::CommentAnnotations(section));
    // -- End Metadata Annotations Section --

    // -- Begin Types Section --
    let mut section = GenericSection::default();
    let generated_types = ArtifactGeneratedTypes::from_operation(
        typegen_operation,
        skip_types,
        request_parameters.is_client_request(),
    );

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "/*::")?;
    }

    write_import_type_from(
        &project_config.typegen_config.language,
        &mut section,
        generated_types.imported_types,
        "relay-runtime",
    )?;

    if !skip_types {
        write!(
            section,
            "{}",
            generate_operation_type_exports_section(
                typegen_operation,
                normalization_operation,
                schema,
                project_config,
            )
        )?;
    }

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "*/")?;
    }
    content_sections.push(ContentSection::Generic(section));
    // -- End Types Section --

    // -- Begin Top Level Statements Section --
    let mut section = GenericSection::default();
    let mut top_level_statements = Default::default();
    if let Some(provided_variables) =
        printer.print_provided_variables(schema, normalization_operation, &mut top_level_statements)
    {
        let mut provided_variable_text = String::new();
        write_variable_value_with_type(
            &project_config.typegen_config.language,
            &mut provided_variable_text,
            CODEGEN_CONSTANTS.provided_variables_definition.lookup(),
            relay_typegen::PROVIDED_VARIABLE_TYPE,
            &provided_variables,
        )
        .unwrap();
        top_level_statements.insert(
            CODEGEN_CONSTANTS.provided_variables_definition.to_string(),
            TopLevelStatement::VariableDefinition(provided_variable_text),
        );
    }

    let request = printer.print_request(
        schema,
        normalization_operation,
        &operation_fragment,
        request_parameters,
        &mut top_level_statements,
    );

    write!(section, "{}", &top_level_statements)?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Top Level Statements Section --

    // -- Begin Query Node Section --
    let mut section = GenericSection::default();
    write_variable_value_with_type(
        &project_config.typegen_config.language,
        &mut section,
        "node",
        generated_types.ast_type,
        &request,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Query Node Section --

    // -- Begin Query Node Hash Section --
    let mut section = GenericSection::default();
    write_source_hash(
        config,
        &project_config.typegen_config.language,
        &mut section,
        &source_hash,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Query Node Hash Section --

    // -- Begin PreloadableQueryRegistry Section --
    let mut section = GenericSection::default();
    if is_operation_preloadable(normalization_operation) && id_and_text_hash.is_some() {
        match project_config.typegen_config.language {
            TypegenLanguage::Flow => {
                writeln!(
                    section,
                    "require('relay-runtime').PreloadableQueryRegistry.set((node.params/*: any*/).id, node);",
                )?;
            }
            TypegenLanguage::JavaScript => {
                writeln!(
                    section,
                    "require('relay-runtime').PreloadableQueryRegistry.set(node.params.id, node);",
                )?;
            }
            TypegenLanguage::TypeScript => {
                writeln!(
                    section,
                    "import {{ PreloadableQueryRegistry }} from 'relay-runtime';",
                )?;
                writeln!(
                    section,
                    "PreloadableQueryRegistry.set(node.params.id, node);",
                )?;
            }
        }
    }
    content_sections.push(ContentSection::Generic(section));
    // -- End PreloadableQueryRegistry Section --

    // -- Begin Export Section --
    let mut section = GenericSection::default();
    write_export_generated_node(
        &project_config.typegen_config,
        &mut section,
        "node",
        generated_types.exported_type,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Export Section --

    content_sections.into_signed_bytes()
}

pub fn generate_split_operation(
    config: &Config,
    project_config: &ProjectConfig,
    printer: &mut Printer<'_>,
    schema: &SDLSchema,
    normalization_operation: &OperationDefinition,
    typegen_operation: &Option<Arc<OperationDefinition>>,
    source_hash: &str,
) -> Result<Vec<u8>, FmtError> {
    let mut content_sections = ContentSections::default();

    // -- Begin Docblock Section --
    content_sections.push(ContentSection::Docblock(generate_docblock_section(
        config,
        project_config,
        vec![],
    )?));
    // -- End Docblock Section --

    // -- Begin Disable Lint Section --
    content_sections.push(ContentSection::Generic(generate_disable_lint_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Disable Lint Section --

    // -- Begin Use Strict Section --
    content_sections.push(ContentSection::Generic(generate_use_strict_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Use Strict Section --

    // -- Begin Types Section --
    let mut section = GenericSection::default();
    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "/*::")?;
    }
    write_import_type_from(
        &project_config.typegen_config.language,
        &mut section,
        "NormalizationSplitOperation",
        "relay-runtime",
    )?;
    writeln!(section)?;

    if let Some(typegen_operation) = typegen_operation {
        writeln!(
            section,
            "{}",
            generate_split_operation_type_exports_section(
                typegen_operation,
                normalization_operation,
                schema,
                project_config,
            )
        )?;
    }

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "*/")?;
    }
    content_sections.push(ContentSection::Generic(section));
    // -- End Types Section --

    // -- Begin Top Level Statements Section --
    let mut section = GenericSection::default();
    let mut top_level_statements = Default::default();
    let operation =
        printer.print_operation(schema, normalization_operation, &mut top_level_statements);

    write!(section, "{}", &top_level_statements)?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Top Level Statements Section --

    // -- Begin Operation Node Section --
    let mut section = GenericSection::default();
    write_variable_value_with_type(
        &project_config.typegen_config.language,
        &mut section,
        "node",
        "NormalizationSplitOperation",
        &operation,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Operation Node Section --

    // -- Begin Operation Node Hash Section --
    let mut section = GenericSection::default();
    write_source_hash(
        config,
        &project_config.typegen_config.language,
        &mut section,
        source_hash,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Operation Node Hash Section --

    // -- Begin Export Section --
    let mut section = GenericSection::default();
    write_export_generated_node(&project_config.typegen_config, &mut section, "node", None)?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Export Section --

    content_sections.into_signed_bytes()
}

#[allow(clippy::too_many_arguments)]
pub fn generate_fragment(
    config: &Config,
    project_config: &ProjectConfig,
    printer: &mut Printer<'_>,
    schema: &SDLSchema,
    reader_fragment: &FragmentDefinition,
    typegen_fragment: &FragmentDefinition,
    source_hash: &str,
    skip_types: bool,
) -> Result<Vec<u8>, FmtError> {
    let is_assignable_fragment = typegen_fragment
        .directives
        .named(*ASSIGNABLE_DIRECTIVE)
        .is_some();
    if is_assignable_fragment {
        generate_assignable_fragment(config, project_config, schema, typegen_fragment, skip_types)
    } else {
        generate_read_only_fragment(
            config,
            project_config,
            printer,
            schema,
            reader_fragment,
            typegen_fragment,
            source_hash,
            skip_types,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_read_only_fragment(
    config: &Config,
    project_config: &ProjectConfig,
    printer: &mut Printer<'_>,
    schema: &SDLSchema,
    reader_fragment: &FragmentDefinition,
    typegen_fragment: &FragmentDefinition,
    source_hash: &str,
    skip_types: bool,
) -> Result<Vec<u8>, FmtError> {
    let mut content_sections = ContentSections::default();

    // -- Begin Docblock Section --
    content_sections.push(ContentSection::Docblock(generate_docblock_section(
        config,
        project_config,
        vec![],
    )?));
    // -- End Docblock Section --

    // -- Begin Disable Lint Section --
    content_sections.push(ContentSection::Generic(generate_disable_lint_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Disable Lint Section --

    // -- Begin Use Strict Section --
    content_sections.push(ContentSection::Generic(generate_use_strict_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Use Strict Section --

    // -- Begin Metadata Annotations Section --
    let mut section = CommentAnnotationsSection::default();
    if let Some(data_driven_dependency_metadata) =
        RelayDataDrivenDependencyMetadata::find(&reader_fragment.directives)
    {
        write_data_driven_dependency_annotation(&mut section, data_driven_dependency_metadata)?;
    }
    if let Some(flight_metadata) =
        ReactFlightLocalComponentsMetadata::find(&reader_fragment.directives)
    {
        write_react_flight_server_annotation(&mut section, flight_metadata)?;
    }
    let relay_client_component_metadata =
        RelayClientComponentMetadata::find(&reader_fragment.directives);
    if let Some(relay_client_component_metadata) = relay_client_component_metadata {
        write_react_flight_client_annotation(&mut section, relay_client_component_metadata)?;
    }
    content_sections.push(ContentSection::CommentAnnotations(section));
    // -- End Metadata Annotations Section --

    // -- Begin Types Section --
    let mut section = GenericSection::default();
    let generated_types = ArtifactGeneratedTypes::from_fragment(typegen_fragment, skip_types);

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "/*::")?;
    }

    write_import_type_from(
        &project_config.typegen_config.language,
        &mut section,
        generated_types.imported_types,
        "relay-runtime",
    )?;

    if !skip_types {
        write!(
            section,
            "{}",
            generate_fragment_type_exports_section(typegen_fragment, schema, project_config)
        )?;
    }

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "*/")?;
    }
    content_sections.push(ContentSection::Generic(section));
    // -- End Types Section --

    // -- Begin Top Level Statements Section --
    let mut section = GenericSection::default();
    let mut top_level_statements = Default::default();
    let fragment = printer.print_fragment(schema, reader_fragment, &mut top_level_statements);

    write!(section, "{}", &top_level_statements)?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Top Level Statements Section --

    // -- Begin Fragment Node Section --
    let mut section = GenericSection::default();
    write_variable_value_with_type(
        &project_config.typegen_config.language,
        &mut section,
        "node",
        generated_types.ast_type,
        &fragment,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Fragment Node Section --

    // -- Begin Fragment Node Hash Section --
    let mut section = GenericSection::default();
    write_source_hash(
        config,
        &project_config.typegen_config.language,
        &mut section,
        source_hash,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Fragment Node Hash Section --

    // -- Begin Fragment Node Export Section --
    let mut section = GenericSection::default();
    write_export_generated_node(
        &project_config.typegen_config,
        &mut section,
        "node",
        generated_types.exported_type,
    )?;
    content_sections.push(ContentSection::Generic(section));
    // -- End Fragment Node Export Section --

    content_sections.into_signed_bytes()
}

fn generate_assignable_fragment(
    config: &Config,
    project_config: &ProjectConfig,
    schema: &SDLSchema,
    typegen_fragment: &FragmentDefinition,
    skip_types: bool,
) -> Result<Vec<u8>, FmtError> {
    let mut content_sections = ContentSections::default();

    // -- Begin Docblock Section --
    content_sections.push(ContentSection::Docblock(generate_docblock_section(
        config,
        project_config,
        vec![],
    )?));
    // -- End Docblock Section --

    // -- Begin Disable Lint Section --
    content_sections.push(ContentSection::Generic(generate_disable_lint_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Disable Lint Section --

    // -- Begin Use Strict Section --
    content_sections.push(ContentSection::Generic(generate_use_strict_section(
        &project_config.typegen_config.language,
    )?));
    // -- End Use Strict Section --

    // -- Begin Types Section --
    let mut section = GenericSection::default();
    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "/*::")?;
    }

    if !skip_types {
        write!(
            section,
            "{}",
            generate_fragment_type_exports_section(typegen_fragment, schema, project_config)
        )?;
    }

    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "*/")?;
    }
    content_sections.push(ContentSection::Generic(section));
    // -- End Types Section --

    // -- Begin Export Section --
    let mut section = GenericSection::default();
    // Assignable fragments should never be passed to useFragment, and thus, we
    // don't need to emit a reader fragment.
    // Instead, we only need a named validator export, i.e.
    // module.exports.validator = ...
    let named_validator_export =
        generate_named_validator_export(typegen_fragment, schema, project_config);
    writeln!(section, "{}", named_validator_export).unwrap();
    content_sections.push(ContentSection::Generic(section));
    // -- End Export Section --

    content_sections.into_signed_bytes()
}

fn write_variable_value_with_type(
    language: &TypegenLanguage,
    section: &mut dyn Write,
    variable_name: &str,
    type_: &str,
    value: &str,
) -> FmtResult {
    match language {
        TypegenLanguage::JavaScript => writeln!(section, "var {} = {};", variable_name, value),
        TypegenLanguage::Flow => {
            writeln!(section, "var {}/*: {}*/ = {};", variable_name, type_, value)
        }
        TypegenLanguage::TypeScript => {
            writeln!(section, "const {}: {} = {};", variable_name, type_, value)
        }
    }
}

fn generate_disable_lint_section(language: &TypegenLanguage) -> Result<GenericSection, FmtError> {
    let mut section = GenericSection::default();
    match language {
        TypegenLanguage::TypeScript => {
            writeln!(section, "/* tslint:disable */")?;
            writeln!(section, "/* eslint-disable */")?;
            writeln!(section, "// @ts-nocheck")?;
        }
        TypegenLanguage::Flow | TypegenLanguage::JavaScript => {
            writeln!(section, "/* eslint-disable */")?;
        }
    }
    Ok(section)
}

fn generate_use_strict_section(language: &TypegenLanguage) -> Result<GenericSection, FmtError> {
    let mut section = GenericSection::default();
    match language {
        TypegenLanguage::TypeScript => {}
        TypegenLanguage::Flow | TypegenLanguage::JavaScript => {
            writeln!(section, "'use strict';")?;
        }
    }
    Ok(section)
}

fn write_import_type_from(
    language: &TypegenLanguage,
    section: &mut dyn Write,
    type_: &str,
    from: &str,
) -> FmtResult {
    match language {
        TypegenLanguage::JavaScript => Ok(()),
        TypegenLanguage::Flow => writeln!(section, "import type {{ {} }} from '{}';", type_, from),
        TypegenLanguage::TypeScript => writeln!(section, "import {{ {} }} from '{}';", type_, from),
    }
}

fn write_export_generated_node(
    typegen_config: &TypegenConfig,
    section: &mut dyn Write,
    variable_node: &str,
    forced_type: Option<String>,
) -> FmtResult {
    if typegen_config.eager_es_modules {
        writeln!(section, "export default {};", variable_node)
    } else {
        match (typegen_config.language, forced_type) {
            (TypegenLanguage::Flow, None) | (TypegenLanguage::JavaScript, _) => {
                writeln!(section, "module.exports = {};", variable_node)
            }
            (TypegenLanguage::Flow, Some(forced_type)) => writeln!(
                section,
                "module.exports = (({}/*: any*/)/*: {}*/);",
                variable_node, forced_type
            ),
            (TypegenLanguage::TypeScript, _) => {
                writeln!(section, "export default {};", variable_node)
            }
        }
    }
}

fn generate_docblock_section(
    config: &Config,
    project_config: &ProjectConfig,
    extra_annotations: Vec<String>,
) -> Result<DocblockSection, FmtError> {
    let mut section = DocblockSection::default();
    if !config.header.is_empty() {
        for header_line in &config.header {
            writeln!(section, "{}", header_line)?;
        }
        writeln!(section)?;
    }
    writeln!(section, "{}", SIGNING_TOKEN)?;
    for annotation in extra_annotations {
        writeln!(section, "{}", annotation)?;
    }
    if project_config.typegen_config.language == TypegenLanguage::Flow {
        writeln!(section, "@flow")?;
    }
    writeln!(section, "@lightSyntaxTransform")?;
    writeln!(section, "@nogrep")?;
    if let Some(codegen_command) = &config.codegen_command {
        writeln!(section, "@codegen-command: {}", codegen_command)?;
    }
    Ok(section)
}

fn write_source_hash(
    config: &Config,
    language: &TypegenLanguage,
    section: &mut dyn Write,
    source_hash: &str,
) -> FmtResult {
    if let Some(is_dev_variable_name) = &config.is_dev_variable_name {
        writeln!(section, "if ({}) {{", is_dev_variable_name)?;
        match language {
            TypegenLanguage::Flow => {
                writeln!(section, "  (node/*: any*/).hash = \"{}\";", source_hash)?
            }
            TypegenLanguage::JavaScript => writeln!(section, "  node.hash = \"{}\";", source_hash)?,
            TypegenLanguage::TypeScript => {
                writeln!(section, "  (node as any).hash = \"{}\";", source_hash)?
            }
        };
        writeln!(section, "}}")?;
    } else {
        match language {
            TypegenLanguage::Flow => {
                writeln!(section, "(node/*: any*/).hash = \"{}\";", source_hash)?
            }
            TypegenLanguage::JavaScript => writeln!(section, "node.hash = \"{}\";", source_hash)?,
            TypegenLanguage::TypeScript => {
                writeln!(section, "(node as any).hash = \"{}\";", source_hash)?
            }
        };
    }

    Ok(())
}

fn write_data_driven_dependency_annotation(
    section: &mut CommentAnnotationsSection,
    data_driven_dependency_metadata: &RelayDataDrivenDependencyMetadata,
) -> FmtResult {
    for (key, value) in data_driven_dependency_metadata
        .direct_dependencies
        .iter()
        .flatten()
    {
        writeln!(section, "@dataDrivenDependency {} {}", key, value)?;
    }
    for (key, value) in data_driven_dependency_metadata
        .indirect_dependencies
        .iter()
        .flatten()
    {
        writeln!(section, "@indirectDataDrivenDependency {} {}", key, value)?;
    }
    Ok(())
}

fn write_react_flight_server_annotation(
    section: &mut CommentAnnotationsSection,
    flight_local_components_metadata: &ReactFlightLocalComponentsMetadata,
) -> FmtResult {
    for item in &flight_local_components_metadata.components {
        writeln!(section, "@ReactFlightServerDependency {}", item)?;
    }
    Ok(())
}

fn write_react_flight_client_annotation(
    section: &mut CommentAnnotationsSection,
    relay_client_component_metadata: &RelayClientComponentMetadata,
) -> FmtResult {
    for value in &relay_client_component_metadata.split_operation_filenames {
        writeln!(section, "@ReactFlightClientDependency {}", value)?;
    }
    Ok(())
}
