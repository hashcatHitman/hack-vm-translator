# SPDX-FileCopyrightText: Copyright © 2025 hashcatHitman
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

#!/usr/bin/env bash

# Create .cargo directory if it does not exist and copy the local config to it.
mkdir -p ./.cargo && cp ./maint/suggested_config.toml ./.cargo/config.toml;

# Copy the toolchain file.
cp ./maint/suggested_toolchain.toml ./rust-toolchain.toml;
