# Outline

[← Back to MODULE](MODULE.md) | [← Back to INDEX](../../INDEX.md)

Symbol maps for 1 large files in this module.

## src/cli/hooks.rs (631 lines)

| Line | Kind | Name | Visibility |
| ---- | ---- | ---- | ---------- |
| 9 | enum | HookManager | pub |
| 17 | fn | fmt | (private) |
| 27 | const | PRE_COMMIT_HOOK | (private) |
| 40 | const | POST_CHECKOUT_HOOK | (private) |
| 55 | const | POST_MERGE_HOOK | (private) |
| 67 | fn | detect_hook_manager | pub |
| 97 | fn | install_hooks_with_manager | pub |
| 126 | fn | install_hooks | pub |
| 130 | fn | install_native_hooks | (private) |
| 149 | fn | install_husky_hooks | (private) |
| 213 | fn | install_husky_hook | (private) |
| 245 | fn | install_lefthook_hooks | (private) |
| 330 | fn | install_pre_commit_hooks | (private) |
| 385 | fn | remove_hooks | pub |
| 398 | fn | remove_native_hooks | (private) |
| 411 | fn | remove_husky_hooks | (private) |
| 445 | fn | remove_lefthook_hooks | (private) |
| 474 | fn | remove_pre_commit_hooks | (private) |
| 498 | fn | find_git_dir | (private) |
| 512 | fn | install_native_hook | (private) |
| 544 | fn | remove_native_hook | (private) |
| 588 | fn | test_find_git_dir | (private) |
| 599 | fn | test_find_git_dir_not_found | (private) |
| 606 | fn | test_detect_native | (private) |
| 612 | fn | test_detect_husky | (private) |
| 619 | fn | test_detect_lefthook | (private) |
| 626 | fn | test_detect_pre_commit | (private) |

