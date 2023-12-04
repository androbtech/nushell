use crate::database::{SQLiteDatabase, MEMORY_DB};
use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Span, SyntaxShape,
    Type, Value,
};

#[derive(Clone)]
pub struct StorExport;

impl Command for StorExport {
    fn name(&self) -> &str {
        "stor export"
    }

    fn signature(&self) -> Signature {
        Signature::build("stor export")
            .input_output_types(vec![(Type::Nothing, Type::Table(vec![]))])
            .required_named(
                "file-name",
                SyntaxShape::String,
                "file name to export the sqlite in-memory database to",
                Some('f'),
            )
            .allow_variants_without_examples(true)
            .category(Category::Math)
    }

    fn usage(&self) -> &str {
        "Export the in-memory sqlite database to a sqlite database file"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["sqlite", "save", "database", "saving", "file"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Export the in-memory sqlite database",
            example: "stor export --file-name nudb.sqlite",
            result: None,
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let span = call.head;
        let file_name_opt: Option<String> = call.get_flag(engine_state, stack, "file-name")?;
        let file_name = match file_name_opt {
            Some(file_name) => file_name,
            None => {
                return Err(ShellError::MissingParameter {
                    param_name: "please supply a file name with the --file-name parameter".into(),
                    span,
                })
            }
        };

        // Open the in-mem database
        let db = Box::new(SQLiteDatabase::new(std::path::Path::new(MEMORY_DB), None));

        if let Ok(conn) = db.open_connection() {
            // This uses vacuum. I'm not really sure if this is the best way to do this.
            // I also added backup in the sqlitedatabase impl. If we have problems, we could switch to that.
            db.export_in_memory_database_to_file(&conn, file_name)
                .map_err(|err| {
                    ShellError::GenericError(
                        "Failed to open SQLite connection in memory from export".into(),
                        err.to_string(),
                        Some(Span::test_data()),
                        None,
                        Vec::new(),
                    )
                })?;
        }
        // dbg!(db.clone());
        Ok(Value::custom_value(db, span).into_pipeline_data())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(StorExport {})
    }
}