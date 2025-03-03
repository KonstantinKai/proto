mod utils;

use starbase_sandbox::predicates::prelude::*;
use utils::*;

mod plugin_search {
    use super::*;

    #[test]
    fn errors_if_no_query() {
        let sandbox = create_empty_proto_sandbox();

        let assert = sandbox
            .run_bin(|cmd| {
                cmd.arg("plugin").arg("search");
            })
            .failure();

        assert.stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ));
    }

    #[test]
    fn errors_if_no_results() {
        let sandbox = create_empty_proto_sandbox();

        let assert = sandbox
            .run_bin(|cmd| {
                cmd.arg("plugin").arg("search").arg("gibberish");
            })
            .failure();

        assert.stdout(predicate::str::contains(
            "no plugins found in the registry for the query gibberish",
        ));
    }

    #[test]
    fn returns_matching_results() {
        let sandbox = create_empty_proto_sandbox();

        let assert = sandbox
            .run_bin(|cmd| {
                cmd.arg("plugin").arg("search").arg("zig");
            })
            .success();

        assert.stdout(predicate::str::contains("Search results for: zig"));
    }

    #[test]
    fn returns_json_data() {
        let sandbox = create_empty_proto_sandbox();

        let assert = sandbox
            .run_bin(|cmd| {
                cmd.arg("plugin").arg("search").arg("zig").arg("--json");
            })
            .success();

        assert.stdout(predicate::str::starts_with("["));
    }

    #[test]
    fn caches_results_in_store() {
        let sandbox = create_empty_proto_sandbox();

        sandbox
            .run_bin(|cmd| {
                cmd.arg("plugin").arg("search").arg("zig");
            })
            .success();

        assert!(
            sandbox
                .path()
                .join(".proto/cache/registry/external-plugins.json")
                .exists()
        );
    }
}
