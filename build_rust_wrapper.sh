#!/bin/zsh

# Exit immediately if a command exits with a non-zero status.
set -e
# Treat unset variables as an error when substituting.
set -u
# Pipelines fail if any command fails, not just the last one.
set -o pipefail

# --- Configuration ---
# Assume this script is run from the folly project root
FOLLY_ROOT_DIR=$(pwd)
# Scratch path is one directory up from the folly source tree
SCRATCH_PATH="${FOLLY_ROOT_DIR}/../_folly_getdeps_scratch"
RUST_WRAPPER_DIR="${FOLLY_ROOT_DIR}/rust_chm_wrapper"
GETDEPS_LOG_FILE="${FOLLY_ROOT_DIR}/folly_build.log" # Log file for getdeps
RUST_BUILD_LOG_FILE="${RUST_WRAPPER_DIR}/rust_build.log"

echo "========================================================================"
echo " Folly Root:    ${FOLLY_ROOT_DIR}"
echo " Scratch Path:  ${SCRATCH_PATH}"
echo " Rust Wrapper:  ${RUST_WRAPPER_DIR}"
echo " Getdeps Log:   ${GETDEPS_LOG_FILE}"
echo " Rust Build Log:${RUST_BUILD_LOG_FILE}"
echo "========================================================================"
echo "Script configuration complete."

# --- Step 1: Build Folly using getdeps.py ---
echo "\n---> Preparing to build Folly and dependencies using getdeps.py..."
# Clean the log file
echo "---> Cleaning log file: ${GETDEPS_LOG_FILE}"
echo "---> Checking log file status before deletion:"
ls -l "${GETDEPS_LOG_FILE}" || echo "---> Log file does not exist yet or cannot be accessed."
echo "---> Deleting log file (if it exists)..."
rm -f "${GETDEPS_LOG_FILE}"
echo "---> Log file deleted successfully."

# Run getdeps.py, tee output to log file and stdout
echo "---> Testing date command:"
date
echo "---> Executing getdeps.py command at $(date)..."
python3 -u "${FOLLY_ROOT_DIR}/build/fbcode_builder/getdeps.py" \
    --scratch-path "$SCRATCH_PATH" \
    build \
    --build-type Release \
    --no-tests \
    -v \
    folly 2>&1 | tee -a "${GETDEPS_LOG_FILE}"
GETDEPS_EXIT_CODE=${PIPESTATUS[0]} # Use PIPESTATUS to get exit code of python3, not tee
echo "---> getdeps.py command finished at $(date) with exit code: ${GETDEPS_EXIT_CODE}"

# Explicitly check exit status
if [ ${GETDEPS_EXIT_CODE} -ne 0 ]; then
    echo "\n!!! ERROR: getdeps.py build failed with exit code ${GETDEPS_EXIT_CODE}. Check log: ${GETDEPS_LOG_FILE}"
    exit 1
fi
echo "\n---> Folly build successful."
# echo "\n---> Skipping Folly build (assuming already built in ${SCRATCH_PATH})."


# --- Step 2: Build Rust Wrapper ---
echo "\n---> Building Rust wrapper..."
if [ ! -d "${RUST_WRAPPER_DIR}" ]; then
    echo "\n!!! ERROR: Rust wrapper directory not found: ${RUST_WRAPPER_DIR}"
    exit 1
fi

# Clean the log file
> "${RUST_BUILD_LOG_FILE}"

# Set environment variable for build.rs
# Set environment variable for build.rs and run cargo build inside a subshell
(
    cd "${RUST_WRAPPER_DIR}" && \
    export FOLLY_GETDEPS_SCRATCH_PATH="${SCRATCH_PATH}" && \
    echo "---> FOLLY_GETDEPS_SCRATCH_PATH set to: ${FOLLY_GETDEPS_SCRATCH_PATH} (inside subshell)" && \
    echo "---> Cleaning Rust target directory..." && \
    cargo clean && \
    echo "---> Building Rust wrapper with cargo..." && \
    cargo build
) 2>&1 | tee -a "${RUST_BUILD_LOG_FILE}"

# Check exit status of the subshell/cargo build
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "\n!!! ERROR: cargo build failed. Check log: ${RUST_BUILD_LOG_FILE}"
    exit 1
fi

echo "\n---> Rust wrapper build successful."
echo "\n========================================================================"
echo " Build Complete!"
echo "========================================================================"

exit 0
