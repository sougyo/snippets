#include <linux/module.h>
#include <linux/seq_file.h>
#include <linux/proc_fs.h>

MODULE_LICENSE("GPL");

static void *my_seq_start(struct seq_file *s, loff_t *pos) {
	static unsigned long counter = 0;
	if (*pos == 0) {
		return &counter;
	} else {
		counter = 0;
		*pos = 0;
		return NULL;
	}
}

static void *my_seq_next(struct seq_file *s, void *v, loff_t *pos) {
	unsigned long *tmp_v = (unsigned long *)v;
	(*tmp_v)++;
	(*pos)++;

	if (*pos >= 10) {
		return NULL;
	}
	return v;
}

static void my_seq_stop(struct seq_file *s, void *v) {
}

static int my_seq_show(struct seq_file *s, void *v) {
	loff_t *spos = (loff_t *) v;
	
	seq_printf(s, "%Ld\n", *spos);
	return 0;
}

static struct seq_operations my_seq_ops = {
	.start = my_seq_start,
	.next  = my_seq_next,
	.stop  = my_seq_stop,
	.show  = my_seq_show
};

static int my_open(struct inode *inode, struct file *file) {
	return seq_open(file, &my_seq_ops);
};

static const struct proc_ops proc_example_operations = {
	.proc_open    = my_open,
	.proc_read    = seq_read,
	.proc_lseek   = seq_lseek,
	.proc_release = seq_release
};

static int __init my_module_init(void) {
	proc_create("example", S_IRUGO | S_IWUGO, NULL, &proc_example_operations);

	return 0;
}

static void __exit my_module_exit(void) {
	remove_proc_entry("example", NULL);
}

module_init(my_module_init);
module_exit(my_module_exit);
