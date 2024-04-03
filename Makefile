CPU := cortex-a76
CPU_CORE := 4
RAM_SIZE := 8192

DISK_IMG := disk.img
DISK_FORMAT := qcow2
DISK_SIZE := 8G

all:
	cargo build
clean:
	rm -rf target/

disk:
	qemu-img create -f ${DISK_FORMAT} ${DISK_IMG} ${DISK_SIZE}

run: disk
	qemu-system-aarch64 -M virt  \
      -machine virtualization=true -machine virt,gic-version=3  \
      -cpu ${CPU} -smp ${CPU_CORE} -m ${RAM_SIZE}           \
      -device virtio-scsi-pci,id=scsi0              \
      -object rng-random,filename=/dev/urandom,id=rng0      \
      -device virtio-rng-pci,rng=rng0               \
      -device virtio-net-pci,netdev=net0                \
      -netdev user,id=net0,hostfwd=tcp::8022-:22            \
      -semihosting \
      -display none \
      -kernel ./target/aarch64-unknown-none/debug/cosmos \
      -drive if=virtio,format=qcow2,file=${DISK_IMG}          \
      -nographic

