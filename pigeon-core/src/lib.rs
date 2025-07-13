pub mod schema;
pub mod registry;

#[cfg(test)]
mod tests {
    use crate::schema::Schema;
    use std::fs;

    #[test]
    fn test_parse_helloworld_schema() {
        let content = fs::read_to_string("../examples/helloworld/pigeon.yaml")
            .expect("Failed to read pigeon.yaml");

        let schema: Schema = serde_yaml::from_str(&content)
            .expect("Failed to parse YAML");

        assert_eq!(schema.service, "HelloService");
        assert_eq!(schema.version, "v1");
        assert_eq!(schema.rpcs.len(), 1);

        let rpc = &schema.rpcs[0];
        assert_eq!(rpc.name, "greet");
        assert_eq!(rpc.method, "POST");
        assert_eq!(rpc.path, "/greet");
        assert_eq!(rpc.request.get("name").unwrap(), "string");
        assert_eq!(rpc.response.get("message").unwrap(), "string");
    }
}
