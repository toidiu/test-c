.PHONY:test

# DYLD_LIBRARY_PATH and LD_LIBRARY_PATH are environment variables used by dynamic linkers to search for shared libraries.
# DYLD_LIBRARY_PATH is used on macOS, while LD_LIBRARY_PATH is used on Linux/Unix systems.
test:
	DYLD_LIBRARY_PATH="../../../build/lib" LD_LIBRARY_PATH="../../../build/lib" cargo test

