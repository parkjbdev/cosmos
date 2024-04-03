CPU := cortex-a76
CPU_CORE := 4
RAM_SIZE := 8192

KERNEL := ./target/aarch64-unknown-none/debug/cosmos

DISK_IMG := disk.img
DISK_FORMAT := qcow2
DISK_SIZE := 8G

${KERNEL}: 
	cargo build

${DISK_IMG}:
	qemu-img create -f ${DISK_FORMAT} ${DISK_IMG} ${DISK_SIZE}

run: ${DISK_IMG} ${KERNEL}
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
      -kernel ${KERNEL} \
      -drive if=virtio,format=${DISK_FORMAT},file=${DISK_IMG}          \
      -nographic

clean:
	rm -rf target/

.PHONE: all run clean

