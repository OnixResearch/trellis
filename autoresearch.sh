#!/usr/bin/env bash
set -euo pipefail
count=$(find openspec/changes -mindepth 1 -maxdepth 1 -type d ! -name archive | wc -l | tr -d ' ')
echo "METRIC active_changes=${count}"
