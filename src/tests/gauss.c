static long (*bpf_trace_printk)(const char* fmt, int fmt_size, long p1, long p2, long p3) = (void*)6;

int main()
{
    char fmt[] = "hello from ebpf, the sum from 1 to 100 is : {}\n";
    int sum = 0;
    for (int i = 1; i < 101; i++)
        sum += i;
    bpf_trace_printk(fmt, 48, sum, 0, 0);
    return sum;
}
