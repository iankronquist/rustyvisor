#include <linux/atomic.h>
#include <linux/kthread.h>
#include <linux/module.h>
#include <linux/semaphore.h>
#include <linux/slab.h>

#define MODULE_NAME "Rustyvisor"
#define KB (0x1000)
#define MB (0x1000 * KB)
#define HEAP_SIZE (256 * KB)

struct core_data {
	void *vmxon_region;
	void *vmcs;
	u64 vmxon_region_phys;
	u64 vmcs_phys;
	size_t vmxon_region_size;
	size_t vmcs_size;
	bool loaded_successfully;
};

extern int rustyvisor_core_load(struct core_data* core_data);
extern void rustyvisor_core_unload(void);
extern int rustyvisor_load(void);
extern void rustyvisor_unload(void);

int rustyvisor_loader_core_load(void *_);
static int __init rustyvisor_init(void);
static void __exit rustyvisor_exit(void);

static DEFINE_PER_CPU(struct core_data, per_core_data);
struct semaphore init_lock;
atomic_t failure_count;
const size_t vmcs_size = 0x1000;
const size_t vmx_region_size = 0x1000;

int rustyvisor_loader_core_load(void *_) {
	int err = 0;
	u32 core_load_status;
	struct core_data *core_data = get_cpu_ptr(&per_core_data);

	memset(core_data, 0, sizeof(*core_data));

	core_data->vmcs_size = vmcs_size;
	core_data->vmcs = kmalloc(vmcs_size, GFP_KERNEL);
	if (core_data->vmcs == NULL) {
		atomic_inc(&failure_count);
		err = 1;
		goto out;
	}
	core_data->vmcs_phys = virt_to_phys(core_data->vmcs);

	core_data->vmxon_region_size = vmx_region_size;
	core_data->vmxon_region = kmalloc(vmx_region_size, GFP_KERNEL);
	if (core_data->vmxon_region == NULL) {
		atomic_inc(&failure_count);
		err = 1;
		goto out;
	}
	core_data->vmxon_region_phys = virt_to_phys(core_data->vmxon_region);

	core_load_status = rustyvisor_core_load(core_data);
	if (core_load_status != 0) {
		atomic_inc(&failure_count);
		err = 1;
		goto out;
	}

out:
	put_cpu_ptr(&per_core_data);
	up(&init_lock);
	return err;
}


int rustyvisor_loader_core_unload(void *_) {
	struct core_data *core_data = get_cpu_ptr(&per_core_data);

	rustyvisor_core_unload();
	kfree(core_data->vmcs);
	kfree(core_data->vmxon_region);
	put_cpu_ptr(&per_core_data);
	up(&init_lock);
	return 0;
}

static void rustyvisor_cleanup(void) {
	int cpu;
	struct task_struct *task;
	sema_init(&init_lock, 1);

	for_each_online_cpu(cpu) {
		task = kthread_create(rustyvisor_loader_core_unload, NULL, "rustyvisor_core_unload");
		kthread_bind(task, cpu);

		down(&init_lock);

		wake_up_process(task);
	}

	down(&init_lock);

	rustyvisor_unload();
}

static int __init rustyvisor_init(void) {
	int cpu;
	int err;
	struct task_struct *task;

	rustyvisor_load();

	sema_init(&init_lock, 1);
	atomic_set(&failure_count, 0);

	for_each_online_cpu(cpu) {
		task = kthread_create(rustyvisor_loader_core_load, NULL, "rustyvisor_core_load");
		kthread_bind(task, cpu);

		down(&init_lock);

		wake_up_process(task);
	}

	down(&init_lock);

	err = atomic_read(&failure_count);
	if (err != 0) {
		printk(KERN_DEBUG "%d cores failed to load\n", err);
		rustyvisor_cleanup();
		return -1;
	}

	return 0;
}

static void __exit rustyvisor_exit(void) {
	rustyvisor_cleanup();
}

module_init(rustyvisor_init);
module_exit(rustyvisor_exit);


MODULE_AUTHOR("Ian Kronquist <iankronquist@gmail.com>");
MODULE_LICENSE("Dual MIT/GPL");
