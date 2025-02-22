/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::{
    compiler_state::{CompilerState, ProjectName},
    config::ProjectConfig,
    docblocks::extract_schema_from_docblock_sources,
    GraphQLAsts,
};
use common::DiagnosticsResult;
use fnv::FnvHashMap;
use graphql_syntax::TypeSystemDefinition;
use schema::SDLSchema;
use std::sync::Arc;

pub fn build_schema(
    compiler_state: &CompilerState,
    project_config: &ProjectConfig,
    graphql_asts_map: &FnvHashMap<ProjectName, GraphQLAsts>,
) -> DiagnosticsResult<Arc<SDLSchema>> {
    let schema = compiler_state.schema_cache.get(&project_config.name);
    match schema {
        Some(schema) if !compiler_state.project_has_pending_schema_changes(project_config.name) => {
            Ok(schema.clone())
        }
        _ => {
            let mut extensions = vec![];
            if let Some(project_extensions) = compiler_state.extensions.get(&project_config.name) {
                extensions.extend(project_extensions.get_sources_with_location());
            }
            if let Some(base_project_name) = project_config.base {
                if let Some(base_project_extensions) =
                    compiler_state.extensions.get(&base_project_name)
                {
                    extensions.extend(base_project_extensions.get_sources_with_location());
                }
            }
            let mut schema_sources = Vec::new();
            schema_sources.extend(
                compiler_state.schemas[&project_config.name]
                    .get_sources_with_location()
                    .into_iter()
                    .map(|(schema, location_key)| (schema.as_str(), location_key)),
            );
            let mut schema =
                relay_schema::build_schema_with_extensions(&schema_sources, &extensions)?;

            if project_config.feature_flags.enable_relay_resolver_transform {
                let mut projects = vec![project_config.name];
                projects.extend(project_config.base);

                let docblock_ast_sources = projects.iter().map(|project_name| {
                    (
                        compiler_state.docblocks.get(project_name),
                        graphql_asts_map.get(project_name),
                    )
                });

                for docblock_ast in docblock_ast_sources {
                    if let (Some(docblocks), Some(graphql_asts)) = docblock_ast {
                        for (file_path, docblock_sources) in &docblocks.get_all() {
                            let executable_definitions =
                                graphql_asts.get_executable_definitions_for_file(file_path);

                            for schema_document in extract_schema_from_docblock_sources(
                                file_path,
                                docblock_sources,
                                &schema,
                                executable_definitions,
                            )? {
                                for definition in schema_document.definitions {
                                    match definition {
                                        TypeSystemDefinition::ObjectTypeExtension(extension) => {
                                            schema.add_object_type_extension(
                                                extension,
                                                schema_document.location.source_location(),
                                                true,
                                            )?
                                        }
                                        TypeSystemDefinition::InterfaceTypeExtension(extension) => {
                                            schema.add_interface_type_extension(
                                                extension,
                                                schema_document.location.source_location(),
                                                true,
                                            )?
                                        }
                                        _ => panic!(
                                            "Expected docblocks to only expose object and interface extensions"
                                        ),
                                    }
                                }
                            }
                        }
                    } else {
                        panic!("Expected to have access to AST and docblock sources.");
                    }
                }
            }

            Ok(Arc::new(schema))
        }
    }
}
