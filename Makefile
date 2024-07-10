all:
	cargo +nightly build -Z build-std=core,alloc --target aarch64-unknown-none.json

launch:
	qemu-system-aarch64 \
    -machine virt,virtualization=on -cpu cortex-a53 \
    -smp 1 -m 1G -nographic -serial mon:stdio \
    -kernel target/aarch64-unknown-none/debug/hypervisor
