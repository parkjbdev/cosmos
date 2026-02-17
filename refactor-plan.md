# Linux Style Refactoring

Here's my analysis and recommendations, comparing your current layout to Linux kernel conventions:

---

## Current State

Your layering is already quite clean — no circular dependencies, good type safety, trait-based abstractions. The main issues are **naming conventions** and **where responsibilities live** relative to how Linux organizes things.

## Current vs Linux-Style Layout

| Your current | Linux equivalent | Notes |
|---|---|---|
| `src/arch/` | `arch/` | Good match |
| `src/bsp/` | _(no equivalent)_ | Linux uses device tree + platform drivers instead |
| `src/drivers/` | `drivers/` | Good, but needs subsystem subdirs |
| `src/memory/` | `mm/` | Rename |
| `src/interrupt/` | `kernel/irq/` | Move into kernel/ |
| `src/sync/` | `kernel/locking/` | Move into kernel/ |
| `src/scheduler/` | `kernel/sched/` | Move into kernel/ |
| `src/console/` | `kernel/printk/` + `drivers/tty/` | Split |
| `src/print.rs` | `kernel/printk/` | Merge with console |
| `src/driver.rs` | `drivers/base/` | Move into drivers/ |
| `src/main.rs` | `init/main.c` | Consider `init/` module |

## Recommended Structure

```
src/
├── init/
│   └── mod.rs              # kernel_main(), boot orchestration (current main.rs)
│
├── arch/
│   └── aarch64/
│       ├── boot/            # entry.s, start.rs, linker script
│       │   └── dts/         # device tree sources (qemu.dts)
│       ├── mm/              # arch-specific MMU, translation tables, MAIR
│       ├── kernel/          # arch-specific exception handling, timer
│       │   ├── entry.s      # vector_table.s → renamed
│       │   └── irq.rs
│       └── include/         # arch-specific types (ExceptionState, etc.)
│
├── kernel/
│   ├── irq/                 # generic IRQ framework (current interrupt/)
│   │   ├── mod.rs           # IRQ domain, registration, dispatch
│   │   └── irqdesc.rs       # interrupt descriptor (like Linux irqdesc)
│   ├── sched/               # scheduler (current scheduler/)
│   ├── locking/             # spinlock, mutex, rwlock (current sync/)
│   ├── printk/              # print macros + console registration + log
│   └── time/                # timekeeping abstractions (generic timer interface)
│
├── mm/                      # generic memory management (current memory/)
│   ├── mod.rs
│   ├── types.rs             # Address<T>, MemoryRegion, PageAddress
│   ├── mmu.rs               # generic MMU interface
│   ├── page_alloc.rs        # page allocator
│   ├── vmalloc.rs           # virtual memory allocation (MMIO remapping)
│   └── init.rs              # kernel_map_sections, memory init
│
├── drivers/
│   ├── base/                # driver model (current driver.rs)
│   │   └── mod.rs           # DeviceDriver trait, probe/remove
│   ├── irqchip/             # interrupt controllers
│   │   └── gicv3/           # GICv3 driver
│   ├── tty/                 # serial/console drivers
│   │   └── serial/
│   │       └── pl011/       # PL011 UART
│   ├── of/                  # device tree (Open Firmware)
│   │   └── mod.rs           # device tree parsing, property access
│   └── mmio.rs              # MMIO helpers
│
├── lib/                     # utility code (alignment helpers, etc.)
│
└── main.rs                  # crate root, module declarations only
```

## Key Recommendations

### 1. Eliminate `bsp/` — use platform drivers instead

Linux doesn't have a BSP layer. Your `bsp/virt/mod.rs` does two things:
- **Loads device addresses from device tree** → this should be in each driver's `probe()` function
- **Calls driver init in the right order** → this belongs in `init/`

Each driver should know how to read its own config from the device tree, matching Linux's `of_match_table` + `probe()` pattern:

```rust
// drivers/tty/serial/pl011/mod.rs
pub fn probe(dt: &Dtb, node_path: &str) -> Result<(), &'static str> {
    let base_addr = dt.get_property(node_path, "reg")?;
    let clock = dt.get_property("/apb-pclk", "clock-frequency")?;
    // ... self-contained initialization
}
```

### 2. Create `kernel/` as the core subsystem

This is the biggest structural change. Linux's `kernel/` directory is where OS-generic core services live:

- `kernel/irq/` — Your `interrupt/` module is only 3 lines. It needs a proper generic IRQ layer that decouples the GIC from interrupt consumers. Linux has `irq_desc`, `irq_domain`, `request_irq()`. You should have at least:
  - A generic `request_irq(irq_num, handler, name)` API
  - The GIC driver registers itself as an `irq_chip`
  - Consumers don't know about GIC at all

- `kernel/locking/` — Your `sync/` is fine code, just rename/move it

- `kernel/printk/` — Merge `print.rs` + `console/mod.rs` + `console/log.rs` here. The `Console` trait stays as the interface drivers implement; `printk` is the kernel-side API.

- `kernel/time/` — Extract the generic timer interface from `arch/aarch64/timer/`. The arch provides the clocksource; `kernel/time/` provides `uptime()`, `delay()`, etc.

### 3. Rename `memory/` → `mm/` and flatten

Linux uses `mm/` for all generic memory management. Your `memory/types/` subdirectory with 7 files for basic types is over-structured. Consider:

```
mm/
├── types.rs        # Address<T>, PageAddress, MemoryRegion, Granule (merge all)
├── mmu.rs          # MMU trait + enable logic
├── page_alloc.rs   # page allocator
├── vmalloc.rs      # MMIO remap / virtual allocation
└── init.rs         # kernel section mapping
```

### 4. Restructure `drivers/` by subsystem

Linux organizes drivers by subsystem, not by chip name:

```
drivers/
├── irqchip/gicv3/     # not drivers/gicv3/
├── tty/serial/pl011/  # not drivers/pl011/
├── of/                # not drivers/devicetree/
└── base/              # driver model framework
```

### 5. Separate arch boot from arch kernel

In Linux, `arch/arm64/` has:
- `boot/` — early boot, linker scripts, DTS files
- `kernel/` — exception entry, IRQ handling, signal delivery
- `mm/` — page tables, MAIR, TLB management

Your `arch/aarch64/` mixes boot code (`start.rs`, `entry.s`, `kernel.ld` is in bsp), exception handling, and MMU code at the same level. Subdirectories help.

### 6. Missing layers you'll need

| Layer | Linux path | Purpose |
|---|---|---|
| `kernel/fork.rs` | `kernel/fork.c` | Task/process creation |
| `kernel/signal/` | `kernel/signal.c` | Signal delivery |
| `kernel/syscall/` | `arch/*/kernel/syscall.c` | System call table + dispatch |
| `mm/slab.rs` | `mm/slab.c` | Kernel heap allocator (you have `embedded-alloc` but no integration) |
| `mm/vmalloc.rs` | `mm/vmalloc.c` | Dynamic kernel virtual mappings |
| `fs/` | `fs/` | VFS + filesystem drivers |
| `ipc/` | `ipc/` | Pipes, shared memory, message queues |

## Priority Order

1. **Create `kernel/`** and move `irq`, `sync`, `printk`, `sched`, `time` into it
2. **Eliminate `bsp/`** — distribute into `init/` (boot ordering) and drivers (self-probe)
3. **Rename `memory/` → `mm/`** and flatten the types
4. **Reorganize `drivers/`** by subsystem
5. **Split arch boot/kernel/mm** subdirectories
6. **Build a proper generic IRQ layer** — this is the most impactful code change (not just file moves)

The good news: your abstractions (traits, type safety, no circular deps) are already solid. Most of this is reorganization, not rewriting.

# Rust Grammar
Now I have a full picture. Here are concrete Rust-specific refactoring recommendations.

---

## 1. Your IRQ subsystem needs a generic `IrqChip` trait

This is the biggest problem. Your exception handler in `handlers.rs:40` directly calls `GicV3::get_and_acknowledge_interrupt()` and indexes into `gicv3::INTERRUPTS`. The arch layer is hardcoded to a specific driver. Linux solves this with `struct irq_chip`.

**Current (tightly coupled):**
```rust
// arch/aarch64/exception/handlers.rs — knows about GicV3 directly
use crate::drivers::gicv3::INTERRUPTS;
use arm_gic::gicv3::GicV3;

fn handle_interrupt(state: ExceptionState) -> *mut usize {
    if let Some(irqid) = GicV3::get_and_acknowledge_interrupt() {
        let id: u32 = irqid.into();
        let irq = INTERRUPTS.lock()[id as usize].unwrap();
        irq.handle_irq(state);
        GicV3::end_interrupt(irqid);
    }
    core::ptr::null_mut()
}
```

**Proposed — define a trait in `kernel/irq/`:**
```rust
// kernel/irq/mod.rs
pub type IrqHandler = fn(irq: u32, state: &ExceptionState) -> bool;

pub trait IrqChip: Send + Sync {
    /// Acknowledge the highest-priority pending interrupt.
    /// Returns None if spurious.
    fn ack(&self) -> Option<u32>;

    /// Signal end-of-interrupt.
    fn eoi(&self, irq: u32);

    /// Enable/disable a specific interrupt line.
    fn enable(&self, irq: u32, enabled: bool);

    /// Set trigger mode.
    fn set_trigger(&self, irq: u32, trigger: IrqTrigger);

    /// Set priority.
    fn set_priority(&self, irq: u32, priority: u8);
}

#[derive(Clone, Copy)]
pub enum IrqTrigger { Edge, Level }

/// The global IRQ dispatch table — arch-independent
static IRQ_CHIP: Mutex<Option<&'static dyn IrqChip>> = Mutex::new(None);
static IRQ_TABLE: Spinlock<[Option<IrqDesc>; 1024]> = /* ... */;

pub struct IrqDesc {
    pub handler: IrqHandler,
    pub name: &'static str,
}

/// Called by arch exception handler — no knowledge of GIC
pub fn handle_irq(state: &ExceptionState) {
    let chip = IRQ_CHIP.lock().unwrap();
    if let Some(irq) = chip.ack() {
        if let Some(desc) = &IRQ_TABLE.lock()[irq as usize] {
            (desc.handler)(irq, state);
        }
        chip.eoi(irq);
    }
}

/// Linux-style request_irq() — drivers call this
pub fn request_irq(
    irq: u32,
    handler: IrqHandler,
    trigger: IrqTrigger,
    priority: u8,
    name: &'static str,
) -> Result<(), &'static str> {
    let chip = IRQ_CHIP.lock().unwrap();
    chip.set_trigger(irq, trigger);
    chip.set_priority(irq, priority);
    chip.enable(irq, true);
    IRQ_TABLE.lock()[irq as usize] = Some(IrqDesc { handler, name });
    Ok(())
}
```

Then the GICv3 driver just implements the trait:
```rust
// drivers/irqchip/gicv3.rs
impl IrqChip for GicV3Wrapper {
    fn ack(&self) -> Option<u32> {
        GicV3::get_and_acknowledge_interrupt().map(|id| id.into())
    }
    fn eoi(&self, irq: u32) {
        GicV3::end_interrupt(IntId::from(irq));
    }
    // ...
}
```

And the arch exception handler becomes generic:
```rust
// arch/aarch64/exception/handlers.rs — no more GIC imports
fn handle_el1h_irq(state: ExceptionState) -> *mut usize {
    crate::kernel::irq::handle_irq(&state);
    core::ptr::null_mut()
}
```

---

## 2. `DeviceDriver` trait is too thin — add Linux-style probe/remove

Your current trait:
```rust
pub trait DeviceDriver {
    fn init(&self) -> Result<(), &'static str> { Ok(()) }
}
```

This doesn't help with anything. Linux's driver model has `probe()` (called when device is found), `remove()`, and a `compatible` string for device tree matching.

**Proposed:**
```rust
pub trait DeviceDriver: Send + Sync {
    /// Device tree compatible strings this driver handles
    fn compatible(&self) -> &'static [&'static str];

    /// Called when a matching device tree node is found.
    /// `node` provides access to the DT properties for this device.
    fn probe(&self, node: &DeviceNode) -> Result<(), &'static str>;

    /// Called on shutdown/removal
    fn remove(&self) -> Result<(), &'static str> { Ok(()) }

    fn name(&self) -> &'static str;
}

/// Parsed device tree node passed to probe()
pub struct DeviceNode<'a> {
    pub path: &'a str,
    pub compatible: &'a str,
    // helper methods to read reg, interrupts, clocks, etc.
}

impl<'a> DeviceNode<'a> {
    pub fn reg(&self) -> impl Iterator<Item = (usize, usize)> { /* ... */ }
    pub fn interrupts(&self) -> impl Iterator<Item = (u32, u32, u32)> { /* ... */ }
    pub fn property_u32(&self, name: &str) -> Option<u32> { /* ... */ }
}
```

Then driver registration + auto-probing:
```rust
// init sequence
static DRIVERS: &[&dyn DeviceDriver] = &[
    &Pl011Driver,
    &GicV3Driver,
];

pub fn probe_all(dt: &Dtb) {
    for node in dt.all_nodes() {
        let compat = node.compatible();
        for drv in DRIVERS {
            if drv.compatible().iter().any(|c| compat.contains(c)) {
                drv.probe(&node).unwrap();
            }
        }
    }
}
```

This eliminates your entire `bsp/virt/mod.rs` — drivers self-discover from the device tree.

---

## 3. Split `Console` trait — decouple IRQ from I/O

Your current Console forces every console to also be an IRQHandler:
```rust
pub trait Console: Write + Read + Statistics + Echo + IRQHandler {}
```

This is a layering violation. A console is an I/O abstraction; interrupt handling is a driver detail. Framebuffer console has no IRQ. Network console has a different IRQ model.

**Proposed — separate concerns:**
```rust
// kernel/printk/mod.rs  (was console/)
pub trait Console: Send + Sync {
    fn write_str(&self, s: &str);
    fn read_char(&self) -> Option<char>;  // Option, not blocking
    fn flush(&self);
}

// The IRQ handler is registered separately by the driver, not via the Console trait.
// In pl011's probe():
//   register_console(&PL011);
//   request_irq(uart_irq, pl011_irq_handler, ...);
```

Drop `Echo` and `Statistics` as trait requirements — those are driver-internal.

---

## 4. Generic `Clocksource` / `Timer` trait

Your timer code in `arch/aarch64/timer/` mixes arch-specific register access with generic concepts (uptime, duration, timeout). Linux separates these into `clocksource` (read the counter) and `clock_event_device` (schedule next interrupt).

```rust
// kernel/time/mod.rs
pub trait Clocksource: Send + Sync {
    fn read(&self) -> u64;        // raw counter value
    fn frequency(&self) -> u64;   // ticks per second
}

pub trait ClockEvent: Send + Sync {
    fn set_next_event(&self, ticks: u64);
    fn enable(&self, enabled: bool);
}

// Generic functions built on the trait
pub fn uptime() -> Duration {
    let cs = clocksource();
    let cnt = cs.read();
    let freq = cs.frequency();
    Duration::new(cnt / freq, ((cnt % freq) * 1_000_000_000 / freq) as u32)
}
```

Then `arch/aarch64/` provides the implementation:
```rust
struct ArmGenericTimer;

impl Clocksource for ArmGenericTimer {
    fn read(&self) -> u64 { CNTPCT_EL0.get() }
    fn frequency(&self) -> u64 { CNTFRQ_EL0.get() }
}

impl ClockEvent for ArmGenericTimer {
    fn set_next_event(&self, ticks: u64) {
        CNTP_TVAL_EL0.set(ticks);
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
    }
    // ...
}
```

---

## 5. Workspace with separate crates

Right now everything is one crate. As the project grows, a Cargo workspace lets you:
- Enforce layering at compile time (a crate can't depend on what it doesn't declare)
- Parallel compilation
- Reuse crates (e.g., your memory types) in tests or tools

```
cosmos/
├── Cargo.toml              # [workspace]
├── cosmos-kernel/           # the main binary
│   ├── Cargo.toml          # depends on all below
│   └── src/
│       └── main.rs         # kernel_main
├── cosmos-arch-aarch64/     # arch-specific crate
│   ├── Cargo.toml          # depends on cosmos-kernel-api
│   └── src/
├── cosmos-kernel-api/       # trait definitions (IrqChip, Console, Clocksource, DeviceDriver, MMU)
│   ├── Cargo.toml          # no_std, no dependencies on other cosmos crates
│   └── src/
├── cosmos-mm/               # generic memory management
│   ├── Cargo.toml          # depends on cosmos-kernel-api
│   └── src/
└── cosmos-drivers/          # device drivers
    ├── Cargo.toml          # depends on cosmos-kernel-api
    └── src/
```

The key crate is **`cosmos-kernel-api`** (like Linux's `include/linux/`). It contains only trait definitions and common types — zero implementations. Every other crate depends on it but not on each other. This enforces the layering at the dependency graph level:

```
cosmos-kernel-api  (traits only: IrqChip, Console, MMU, DeviceDriver, Clocksource)
       ▲
       │
  ┌────┼──────────┬───────────────┐
  │    │          │               │
cosmos-arch    cosmos-mm    cosmos-drivers
  │    │          │               │
  └────┼──────────┴───────────────┘
       │
       ▼
  cosmos-kernel  (wires everything together)
```

You don't need to do this right away. It's the endgame structure. For now, you can emulate it within a single crate by being disciplined about what each module imports.

---

## 6. Replace global statics pattern with registration functions

You currently have 5+ `static` globals scattered across modules (`PL011_UART`, `GIC`, `INTERRUPTS`, `DEVICE_TREE`, `CONSOLE`). Each uses a different mechanism (`OnceCell`, `Mutex<Option<>>`, `Spinlock`).

**Standardize on one pattern:**
```rust
// A reusable "register once, use forever" pattern
pub struct StaticCell<T: Send + Sync> {
    inner: OnceCell<RawSpinlock, T>,
    name: &'static str,
}

impl<T: Send + Sync> StaticCell<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { inner: OnceCell::new(), name }
    }

    pub fn init(&self, val: T) {
        self.inner.set(val).unwrap_or_else(|_| {
            panic!("{} already initialized", self.name);
        });
    }

    pub fn get(&self) -> &T {
        self.inner.get().unwrap_or_else(|| {
            panic!("{} not yet initialized", self.name);
        })
    }
}
```

Usage:
```rust
static CONSOLE: StaticCell<&'static dyn Console> = StaticCell::new("console");
static IRQ_CHIP: StaticCell<&'static dyn IrqChip> = StaticCell::new("irq_chip");
static CLOCKSOURCE: StaticCell<&'static dyn Clocksource> = StaticCell::new("clocksource");
```

---

## 7. Use Rust's type system to enforce init ordering

Your boot sequence has a critical invariant: no printing between MMU enable and UART remap. This is a comment. You can make it a compile-time guarantee with typestate:

```rust
pub struct BootPhase<S> { _state: PhantomData<S> }

pub struct PreMmu;
pub struct PostMmu;      // MMU on, but no MMIO yet
pub struct Ready;        // drivers remapped, safe to print

impl BootPhase<PreMmu> {
    pub fn enable_mmu(self, tables: Address<Physical>) -> BootPhase<PostMmu> {
        memory::mmu::init(tables).unwrap();
        BootPhase { _state: PhantomData }
    }
}

impl BootPhase<PostMmu> {
    // println! is simply not available here — Console isn't registered yet
    pub fn remap_drivers(self) -> BootPhase<Ready> {
        memory::mmu::init_mmio_allocator();
        bsp::init_drivers(false);
        BootPhase { _state: PhantomData }
    }
}

impl BootPhase<Ready> {
    pub fn console(&self) -> &'static dyn Console {
        console::console()
    }
}
```

This is optional/advanced, but it's a Rust-native way to express what Linux does with `__init` section ordering.

---

## 8. Use `#[derive]` and reduce boilerplate in Interrupt

Your `Interrupt` struct does registration inside `new()` (side effect in constructor) and uses raw u32s for everything. Rust's enums and builder pattern help:

```rust
pub enum IrqTrigger { Edge, Level }

pub struct IrqRequest {
    pub irq: u32,
    pub trigger: IrqTrigger,
    pub priority: u8,
    pub name: &'static str,
    pub handler: fn(irq: u32, state: &ExceptionState) -> bool,
}

// Registration is explicit, not a side effect of construction
pub fn request_irq(req: IrqRequest) -> Result<(), &'static str> {
    // ...
}
```

This separates "describing an interrupt" from "registering it with hardware" — a common Rust pattern (builder → execute).

---

## Summary — priority order

| Priority | Change | Effort | Impact |
|---|---|---|---|
| 1 | `IrqChip` trait + generic IRQ layer | Medium | Decouples arch from drivers |
| 2 | `DeviceDriver` probe() + DT matching | Medium | Eliminates BSP |
| 3 | Split Console from IRQHandler | Small | Cleaner layering |
| 4 | `Clocksource` / `ClockEvent` traits | Small | Portable timer |
| 5 | Standardize global registration pattern | Small | Consistency |
| 6 | `request_irq()` API (separate desc from registration) | Small | Cleaner driver code |
| 7 | Cargo workspace split | Large | Compile-time layer enforcement |
| 8 | Typestate boot phases | Small | Safety, but optional |

Start with 1-4. Those give you Linux-like layering with Rust's trait system doing what C function pointers + vtables do in Linux, but with compile-time type checking.
