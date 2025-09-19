# Changelog

## [1.0.0](https://github.com/elsonwu/jing-lang/compare/v0.1.0...v1.0.0) (2025-09-19)


### ⚠ BREAKING CHANGES

* **simplify:** HTTP server functions are no longer available
* **http:** HTTP handler registration API changed from port-based to server handle-based. Before: http_register_handler(8080, 'GET', '/path', 'handler') After: let server = start_http_server(8080); http_register_handler(server, 'GET', '/path', 'handler')

### Features

* add I/O capabilities and comprehensive documentation structure ([#4](https://github.com/elsonwu/jing-lang/issues/4)) ([bd618f8](https://github.com/elsonwu/jing-lang/commit/bd618f800d7fc6d192f15e8d823b92d771aacf4f))
* **http:** Implement Modern Server Handle-Based HTTP Handler System ([#6](https://github.com/elsonwu/jing-lang/issues/6)) ([a9c26f3](https://github.com/elsonwu/jing-lang/commit/a9c26f3ca54db532782034abbf38ebf3eee81f78))
* implement modular architecture with trait-based builtin system ([#2](https://github.com/elsonwu/jing-lang/issues/2)) ([5f0e342](https://github.com/elsonwu/jing-lang/commit/5f0e3423fedb527f308122770da318aca38d8102))
* implement recursive function support ([3ee2d22](https://github.com/elsonwu/jing-lang/commit/3ee2d22063e94ae3f7a2ab09ad965849076eed07))
* **simplify:** remove HTTP server functionality for redesign ([771caaa](https://github.com/elsonwu/jing-lang/commit/771caaae0b8387033ef161dcd7cc14e8b804dfb7))


### Bug Fixes

* remove temporary file and improve gitignore ([#5](https://github.com/elsonwu/jing-lang/issues/5)) ([51ef5b2](https://github.com/elsonwu/jing-lang/commit/51ef5b22cfd04c9f22a6b9d66cf5e00df759bab1))

## [Unreleased]

### Added
- **File I/O Functions**: Complete file system operations support
  - `read_file(path)` - Read entire file contents as string
  - `write_file(path, content)` - Write string content to file
  - `file_exists(path)` - Check if file or directory exists
- **Interactive Input Functions**: Enhanced user interaction capabilities
  - `input(prompt)` - Display prompt and read user input
  - `readline()` - Read line from standard input without prompt
- **Recursive Function Support**: Functions can now call themselves recursively
  - Proper local scope management for function parameters
  - Stack-based call frames for recursive calls
  - Examples: factorial, fibonacci functions
- **Development Guidelines**: Comprehensive development standards ([docs/DEVELOPMENT_GUIDELINES.md](docs/DEVELOPMENT_GUIDELINES.md))
  - Documentation-first development requirements
  - Mandatory quality gates (format, lint, build, test)
  - Conventional commit message enforcement
  - Testing requirements and standards

### Changed
- **Enhanced Testing Suite**: Expanded from 77 to 79+ tests
  - Added comprehensive I/O function tests with temporary file handling
  - Added recursive function integration tests
  - All existing functionality tests still passing
- **Modular Builtin System**: Improved extensibility
  - Trait-based builtin function architecture
  - Central registry for easy function registration
  - Clear separation between I/O, math, string, and core functions
- **Documentation Updates**: Complete documentation overhaul
  - Updated README.md with I/O capabilities
  - Enhanced LANGUAGE_REFERENCE.md with all builtin functions
  - Added comprehensive examples and usage patterns

### Technical Details
- **Architecture**: Maintained clean pipeline pattern (Lexer → Parser → Compiler → VM)
- **Error Handling**: Comprehensive error messages for file operations and recursive calls
- **Performance**: Stack-based VM with efficient call frame management
- **Dependencies**: Added `tempfile` for testing infrastructure

## 0.1.0 (2025-08-24)


### Features

* add assignment expressions and comprehensive documentation updates ([a0fdb1d](https://github.com/elsonwu/jing-lang/commit/a0fdb1d3a4d459bd2d0e7f4b494c4e1d2c493c0d))


### Bug Fixes

* fix integration issues ([24ef837](https://github.com/elsonwu/jing-lang/commit/24ef8373b9ae1e1528d91a630d8873df4a3be53e))
* fix some clippy issues ([0bfe058](https://github.com/elsonwu/jing-lang/commit/0bfe05880277ba0c13144886ae688e22d38ea62a))
* resolve CI issues and enhance privacy ([87d1af6](https://github.com/elsonwu/jing-lang/commit/87d1af6d225ca635e03fc71c20b1633242426c13))
* resolve GitHub Actions compilation errors with API compatibility updates ([5605e5c](https://github.com/elsonwu/jing-lang/commit/5605e5cf0f7c7514927255708c434270eb09f8d4))
