/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <limits>
#include <cstdint> // For uintptr_t

namespace folly {

// Define a sentinel value for insert_or_assign_and_get_old.
// Using UINTPTR_MAX is generally safe as valid pointers are unlikely to have this value.
constexpr uintptr_t kConcurrentHashMapNotFoundSentinel = std::numeric_limits<uintptr_t>::max();

} // namespace folly
