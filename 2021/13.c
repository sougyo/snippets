#include <linux/module.h>
#include <linux/seq_file.h>
#include <linux/proc_fs.h>

MODULE_LICENSE("GPL");

#define BUFSIZE 100
#define ARRAYSIZE 2000000

struct page_entry {
	unsigned long addr;
	int order;
};

int cnt = 0;
struct page_entry page_list[ARRAYSIZE];

static ssize_t
proc_write(struct file *file,
           const char __user * buffer,
           size_t count, loff_t * ppos) {
	int order;
	int len = (count > BUFSIZE - 1) ? (BUFSIZE - 1) : count;
	char strbuf[BUFSIZE];

	if (copy_from_user(strbuf, buffer, len))
		return -EFAULT;

	strbuf[len] = '\0';
	sscanf(strbuf, "%d", &order);

	if (order < 11) {
		unsigned long addr = __get_free_pages(GFP_KERNEL, order);
		if (cnt < ARRAYSIZE && addr != 0) {
			page_list[cnt].addr  = addr;
			page_list[cnt].order = order;
			cnt++;
		}
	}

	return count;
}

static const struct proc_ops proc_example_operations = {
	.proc_write = proc_write,
};

static int __init my_module_init(void) {
	proc_create("example", S_IRUGO | S_IWUGO, NULL, &proc_example_operations);

	return 0;
}

static void __exit my_module_exit(void) {
	int i;
	for (i = cnt - 1; i >= 0; i--) {
		free_pages(page_list[i].addr, page_list[i].order);
	}
	remove_proc_entry("example", NULL);
}

module_init(my_module_init);
module_exit(my_module_exit);
