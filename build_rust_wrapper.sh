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
GETDEPS_LOG_FILE="${FOLLY_ROOT_DIR}/folly_build.log" # Log file for getdeps (now unused)
RUST_BUILD_LOG_FILE="${RUST_WRAPPER_DIR}/rust_build.log"

echo "========================================================================"
echo " Folly Root:    ${FOLLY_ROOT_DIR}"
echo " Scratch Path:  ${SCRATCH_PATH}"
echo " Rust Wrapper:  ${RUST_WRAPPER_DIR}"
echo " Getdeps Log:   ${GETDEPS_LOG_FILE}"
echo " Rust Build Log:${RUST_BUILD_LOG_FILE}"
echo "========================================================================"
echo "Script configuration complete."

# --- Step 1: Build Folly using getdeps.py (SKIPPED) ---
# echo "\n---> Preparing to build Folly and dependencies using getdeps.py..."
# # Clean the log file
# echo "---> Cleaning log file: ${GETDEPS_LOG_FILE}"
# > "${GETDEPS_LOG_FILE}"
#
# # Run getdeps.py, tee output to log file and stdout
# echo "---> Executing getdeps.py command..."
# python3 -u "${FOLLY_ROOT_DIR}/build/fbcode_builder/getdeps.py" \
#     --scratch-path "$SCRATCH_PATH" \
#     build \
#     --build-type Release \
#     --no-tests \
#     -v \
#     folly 2>&1 | tee -a "${GETDEPS_LOG_FILE}"
#
# # Check exit status (redundant with set -e and pipefail, but explicit)
# if [ ${PIPESTATUS[0]} -ne 0 ]; then
#     echo "\n!!! ERROR: getdeps.py build failed. Check log: ${GETDEPS_LOG_FILE}"
#     exit 1
# fi
# echo "\n---> Folly build successful."
echo "\n---> Skipping Folly build (assuming already built in ${SCRATCH_PATH})."


# --- Step 2: Build Rust Wrapper ---
echo "\n---> Building Rust wrapper..."
if [ ! -d "${RUST_WRAPPER_DIR}" ]; then
    echo "\n!!! ERROR: Rust wrapper directory not found: ${RUST_WRAPPER_DIR}"
    exit 1
fi

# Clean the log file
> "${RUST_BUILD_LOG_FILE}"

# Set environment variable for build.rs
export FOLLY_GETDEPS_SCRATCH_PATH="${SCRATCH_PATH}"

# Run cargo build from the wrapper directory, tee output
(cd "${RUST_WRAPPER_DIR}" && cargo build) 2>&1 | tee -a "${RUST_BUILD_LOG_FILE}"

# Check exit status
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "\n!!! ERROR: cargo build failed. Check log: ${RUST_BUILD_LOG_FILE}"
    exit 1
fi

echo "\n---> Rust wrapper build successful."
echo "\n========================================================================"
echo " Build Complete!"
echo "========================================================================"

exit 0
