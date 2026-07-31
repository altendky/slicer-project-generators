#!/usr/bin/env bash

set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
    printf 'error: jq is required to check workspace metadata\n' >&2
    exit 1
fi

metadata="$(cargo metadata --locked --no-deps --format-version 1)"

if ! jq -e '
    ([.packages[].name] | sort) == [
        "slicer-project-generator-bambu-studio",
        "slicer-project-generator-orca-slicer",
        "slicer-project-generator-prusa-slicer"
    ]
    and (.packages | length == 3)
    and (.workspace_members | length == 3)
    and all(
        .packages[];
        .edition == "2024"
        and .rust_version == "1.94"
        and .publish == []
        and .license == "AGPL-3.0-only"
    )
    and all(
        .packages[] | select(.name != "slicer-project-generator-bambu-studio");
        (.dependencies | length == 0)
        and (.targets | length == 1)
        and .targets[0].kind == ["bin"]
        and .targets[0].crate_types == ["bin"]
        and .targets[0].name == .name
    )
    and any(
        .packages[];
        .name == "slicer-project-generator-bambu-studio"
        and ([.dependencies[].name] | sort) == ["quick-xml", "zip"]
        and any(.targets[]; .kind == ["lib"])
        and any(.targets[]; .kind == ["bin"] and .name == "slicer-project-generator-bambu-studio")
    )
' >/dev/null <<<"${metadata}"; then
    printf 'error: workspace metadata violates neutral scaffold invariants\n' >&2
    exit 1
fi

cargo build --workspace --locked

target_directory="$(jq -r '.target_directory' <<<"${metadata}")"
for package in \
    slicer-project-generator-bambu-studio \
    slicer-project-generator-orca-slicer \
    slicer-project-generator-prusa-slicer
do
    binary="${target_directory}/debug/${package}"
    if output="$("${binary}" 2>&1)"; then
        printf 'error: %s unexpectedly exited successfully\n' "${package}" >&2
        exit 1
    fi

    expected="${package} has no implemented capabilities"
    if [[ "${output}" != "${expected}" ]]; then
        printf 'error: unexpected diagnostic from %s: %s\n' \
            "${package}" "${output}" >&2
        exit 1
    fi
done

printf 'Workspace metadata, Bambu library, and protocol stubs are valid.\n'
