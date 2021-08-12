#include <linux/module.h>
#include <linux/proc_fs.h>

MODULE_LICENSE("GPL");

static int myparam = 0;
module_param(myparam, int, 0660);
#define cpu_to_node(n) myparam

static int __init my_module_init(void) {
	printk(KERN_INFO "%d\n", cpu_to_node(0));
	return 0;
}

static void __exit my_module_exit(void) {
}

module_init(my_module_init);
module_exit(my_module_exit);

