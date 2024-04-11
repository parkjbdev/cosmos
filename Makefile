CPU := cortex-a76
CPU_CORE := 4
RAM_SIZE := 8192

KERNEL := ./target/aarch64-unknown-none/debug/cosmos

DISK_IMG := disk.img
DISK_FORMAT := qcow2
DISK_SIZE := 8G

DTB_NAME := qemu

build:
	cargo build

${KERNEL}: 
	cargo build

${DISK_IMG}:
	qemu-img create -f ${DISK_FORMAT} ${DISK_IMG} ${DISK_SIZE}

run: el2

el3: ${DISK_IMG} ${KERNEL}
	qemu-system-aarch64 -M virt  \
			-machine virt,gic-version=3,secure=true  \
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

el2: ${DISK_IMG} ${KERNEL}
	qemu-system-aarch64 -M virt  \
			-machine virt,gic-version=3,virtualization=true  \
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

el1: ${DISK_IMG} ${KERNEL}
	qemu-system-aarch64 -M virt  \
			-machine virt,gic-version=3  \
      -cpu ${CPU} -smp ${CPU_CORE} -m ${RAM_SIZE}           \
      -device virtio-scsi-pci,id=scsi0              \
      -object rng-random,filename=/dev/urandom,id=rng0      \
      -device virtio-rng-pci,rng=rng0               \
      -device virtio-net-pci,netdev=net0                \
      -netdev user,id=net0,hostfwd=tcp::8022-:22            \
      -semihosting \
      -display none \
      -device loader,file=${KERNEL} \
      -drive if=virtio,format=${DISK_FORMAT},file=${DISK_IMG}          \
      -nographic

${DTB_NAME}.dtb:
	qemu-system-aarch64 -M virt  \
      -machine virtualization=true -machine virt,gic-version=4  \
			-machine dumpdtb=${DTB_NAME}.dtb \
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

dts: ${DTB_NAME}.dtb
	dtc -I dtb -O dts ${DTB_NAME}.dtb -o ${DTB_NAME}.dts
	cat ${DTB_NAME}.dts

clean: 
	rm ${DISK_IMG}
	rm -rf target/

dts-clean:
	rm ${DTB_NAME}.dts
	rm ${DTB_NAME}.dtb

.PHONE: all run clean dts-clean

