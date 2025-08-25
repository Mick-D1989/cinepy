.PHONY: list
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

build:
	CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend

build-wheels:
	maturin buil -m crates/cine_py/Cargo.toml --release

python-dev:
	maturin develop --uv -m crates/cine-py/Cargo.toml

python-release:
	maturin develop --uv -m crates/cine-py/Cargo.toml --release

python-test:python-dev
	uv pip install -r crates/cine-py/python/tests/requirements.txt
	pytest crates/cine-py/python/tests

python-test-release:python-release
	uv pip install -r crates/cine-py/python/tests/requirements.txt
	pytest crates/cine-py/python/tests
	./benchmark.sh > benchmark_result_$$(date +"%Y-%m-%d_%H:%M:%S")
	
clean:
	rm -rf target
	uv pip uninstall cine-py
