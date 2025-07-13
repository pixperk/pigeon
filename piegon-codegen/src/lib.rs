use pigeon_core::schema::{FieldMap, Schema};

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
            }
            chars.into_iter().collect::<String>()
        })
        .collect()
}

fn map_field_type(field_type: &str) -> &str {
    match field_type {
        "string" => "String",
        "integer" => "i32", 
        "boolean" => "bool",
        "float" => "f64",
        _ => "String", // default fallback
    }
}

pub fn generate_struct(
    rpc_name: &str,
    suffix: &str,
    fields: &FieldMap,
) -> anyhow::Result<String> {
    let struct_name = format!("{}{}", to_pascal_case(rpc_name), suffix);

    let mut struct_body = format!(
        "#[derive(Debug, serde::Serialize, serde::Deserialize, validator::Validate)]\npub struct {} {{\n",
        struct_name
    );

    for (name, field_def) in fields {
        // Convert FieldDef to Field
        let field = field_def.clone().into_field();
        let rust_type = map_field_type(&field.r#type);
        let is_optional = field.optional.unwrap_or(false);
        let field_line = if is_optional {
            format!("    pub {}: Option<{}>,\n", name, rust_type)
        } else {
            format!("    pub {}: {},\n", name, rust_type)
        };

        struct_body.push_str(&field_line);
    }

    struct_body.push_str("}\n");
    Ok(struct_body)
}

pub fn generate_structs(schema: &Schema) -> anyhow::Result<String> {
    let mut output = String::new();

    for rpc in &schema.rpcs {
        let req_struct = generate_struct(&rpc.name, "Request", &rpc.request)?;
        let res_struct = generate_struct(&rpc.name, "Response", &rpc.response)?;
        output.push_str(&req_struct);
        output.push('\n');
        output.push_str(&res_struct);
        output.push('\n');
    }

    Ok(output)
}

pub fn write_structs_to_file(schema: &Schema, path: &str) -> anyhow::Result<()> {
    let code = generate_structs(schema)?;
    std::fs::write(path, code)?;
    Ok(())
}
