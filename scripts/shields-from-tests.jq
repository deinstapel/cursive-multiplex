{
    # filter for "passed" and "test_count" in input objects with `"type": "suite"`
    # and accumulate stats from all tests
    "passed": map(select(.type == "suite" and has("passed")) | .passed) | add,
    "total": map(select(.type == "suite" and has("test_count")) | .test_count) | add
} | . + {
    # calculate ratio of passed tests
    "factor": (.passed / .total)
} | {
    # calculate color from test factor
    "color": (
        if .factor < 0.33 then
            "red"
        elif .factor < 0.66 then
            "orange"
        elif .factor < 1.0 then
            "yellow"
        else
            "brightgreen"
        end
    ),
    "isError": true,
    "label": "cargo test",
    # interpolate the shield label
    "message": "\(.passed) / \(.total) tests",
    "schemaVersion": 1
}
