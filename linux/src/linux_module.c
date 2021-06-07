#include <asm/io.h>
#include <linux/atomic.h>
#include <linux/kthread.h>
#include <linux/module.h>
#include <linux/semaphore.h>
#include <linux/slab.h>

#define MODULE_NAME "Rustyvisor"

struct semaphore init_lock;
atomic_t failure_count;


extern int rustyvisor_linux_core_load(void *_);
extern int rustyvisor_load(void);

extern int rustyvisor_core_unload(void *_);
extern int rustyvisor_unload(void);


void *rustyvisor_linux_kmalloc(uintptr_t bytes) {
	void *ptr = kmalloc(bytes, GFP_KERNEL);
    if (ptr != NULL) {
        memset(ptr, 0, bytes);
    }
    return ptr;
}

uintptr_t rustyvisor_linux_virt_to_phys(void *virt) {
    return virt_to_phys(virt);
}

static void rustyvisor_linux_unload_all_cores(void) {
	int cpu;
	struct task_struct *task;
	sema_init(&init_lock, 1);

	for_each_online_cpu(cpu) {
		task = kthread_create(rustyvisor_core_unload, NULL, "rustyvisor_core_unload");
		kthread_bind(task, cpu);

		down(&init_lock);

		wake_up_process(task);
	}

	down(&init_lock);

	rustyvisor_unload();
}

static void __exit rustyvisor_exit(void) {
	rustyvisor_linux_unload_all_cores();
}

static int __init rustyvisor_init(void) {
	int cpu;
	int err;
	struct task_struct *task;

	rustyvisor_load();

	sema_init(&init_lock, 1);
	atomic_set(&failure_count, 0);

	for_each_online_cpu(cpu) {
		task = kthread_create(rustyvisor_linux_core_load, NULL, "rustyvisor_linux_core_load");
		kthread_bind(task, cpu);

		down(&init_lock);

		wake_up_process(task);
	}

	down(&init_lock);

	err = atomic_read(&failure_count);
	if (err != 0) {
		printk(KERN_DEBUG "%d cores failed to load\n", err);
		rustyvisor_linux_unload_all_cores();
		return -1;
	}

	return 0;
}

module_init(rustyvisor_init);
module_exit(rustyvisor_exit);


MODULE_AUTHOR("Ian Kronquist <iankronquist@gmail.com>");
MODULE_LICENSE("Dual MIT/GPL");
