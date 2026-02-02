#!/usr/bin/env bash

# Script to open the Criterion benchmark HTML report

REPORT_PATH="target/criterion/report/index.html"

if [ ! -f "$REPORT_PATH" ]; then
    echo "Error: Benchmark report not found at $REPORT_PATH"
    echo "Please run 'cargo bench' first to generate the reports."
    exit 1
fi

echo "Opening benchmark report: $REPORT_PATH"

# Detect OS and open the report accordingly
case "$(uname -s)" in
Darwin*)
    open "$REPORT_PATH"
    ;;
Linux*)
    if command -v xdg-open >/dev/null; then
        xdg-open "$REPORT_PATH"
    else
        echo "Please open the following file in your browser:"
        echo "$(pwd)/$REPORT_PATH"
    fi
    ;;
CYGWIN* | MINGW* | MSYS*)
    start "$REPORT_PATH"
    ;;
*)
    echo "Please open the following file in your browser:"
    echo "$(pwd)/$REPORT_PATH"
    ;;
esac
