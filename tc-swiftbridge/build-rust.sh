#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

# Build the project for the desired platforms:
BUILD_SWIFT=1 cargo build --target x86_64-apple-darwin
BUILD_SWIFT=1 cargo build --target aarch64-apple-darwin

BUILD_SWIFT=1 cargo build --target aarch64-apple-ios
BUILD_SWIFT=1 cargo build --target x86_64-apple-ios
BUILD_SWIFT=1 cargo build --target aarch64-apple-ios-sim

