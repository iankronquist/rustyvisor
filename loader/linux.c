#include <linux/module.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <linux/tboot.h>

#include <asm/msr.h>
#include <asm/processor.h>
#include <asm/tlbflush.h>
#include <asm/virtext.h>


#define MODULE_NAME "Rustyvisor"

extern void entry(void);


static int __init hype_init(void) {

	entry();

	return 0;
}


static void __exit hype_exit(void) {
}


module_init(hype_init);
module_exit(hype_exit);


MODULE_AUTHOR("Ian Kronquist <iankronquist@gmail.com>");
MODULE_LICENSE("Dual MIT/GPL");
