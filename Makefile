CPU := cortex-a76
CPU_CORE := 1
RAM_SIZE := 8G 

KERNEL := ./target/aarch64-unknown-none-softfloat/debug/cosmos

DISK_IMG := disk.img
DISK_FORMAT := qcow2
DISK_SIZE := 8G

DTB_NAME := qemu

${KERNEL}: 
	cargo build

${DISK_IMG}:
	qemu-img create -f ${DISK_FORMAT} ${DISK_IMG} ${DISK_SIZE}

run: ${DISK_IMG} ${KERNEL}
	@qemu-system-aarch64 \
		-machine virt,gic-version=3,virtualization=true  \
		-cpu ${CPU} -smp ${CPU_CORE} -m ${RAM_SIZE}           \
		-semihosting \
		-kernel ${KERNEL} \
		-drive if=virtio,format=${DISK_FORMAT},file=${DISK_IMG}          \
		-nographic -serial mon:stdio

dbg: ${DISK_IMG} ${KERNEL}
	@qemu-system-aarch64 \
		-machine virt,gic-version=3,virtualization=true  \
		-cpu ${CPU} -smp ${CPU_CORE} -m ${RAM_SIZE}           \
		-semihosting \
		-kernel ${KERNEL} \
		-drive if=virtio,format=${DISK_FORMAT},file=${DISK_IMG}          \
		-nographic -serial mon:stdio -s -S

lldb:
	lldb -o "gdb-remote localhost:1234" ${KERNEL}

${DTB_NAME}.dtb:
	@qemu-system-aarch64 \
			-machine virt,gic-version=3,virtualization=true  \
			-machine dumpdtb=${DTB_NAME}.dtb \
      -cpu ${CPU} -smp ${CPU_CORE} -m ${RAM_SIZE}           \
      -semihosting \
      -kernel ${KERNEL} \
      -drive if=virtio,format=${DISK_FORMAT},file=${DISK_IMG}          \
			-nographic -serial mon:stdio

dts: ${DTB_NAME}.dtb
	dtc -I dtb -O dts ${DTB_NAME}.dtb -o ${DTB_NAME}.dts
	cat ${DTB_NAME}.dts

clean: 
	rm -rf target/
	rm ${DISK_IMG}

dts-clean:
	rm ${DTB_NAME}.dts
	rm ${DTB_NAME}.dtb

.PHONE: all run clean dts-clean

