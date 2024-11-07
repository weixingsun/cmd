import warnings
warnings.filterwarnings('ignore')
import subprocess,shlex,argparse
DEBUG=False

def exec_cmd(cmd):
    p=subprocess.run([cmd], shell=True, check=False, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    out=p.stdout.decode('utf-8').strip()
    return out

def get_interrupt(name):
    cmd="grep processor /proc/cpuinfo |wc -l"
    n_cpu=int(exec_cmd(cmd))
    cmd="cat /proc/interrupts|grep {}".format(name)
    output=exec_cmd(cmd)
    if DEBUG:
        print(cmd)
    #print(output)
    lines=output.splitlines()
    irqs=[]
    for line in lines:
        cols=line.split()
        name=cols[0][:-1]
        irqs.append(name)
        if DEBUG:
            cpus=[]
            for idx,vlu in enumerate(cols):
                if idx<1:
                    continue
                if idx>n_cpu-1:
                    continue
                if vlu > '0':
                    cpus.append(idx)
            print("IRQ",name,cpus)
    return irqs

def set_interrupt(irq,cpu):
    cmd="echo {} > /proc/irq/{}/smp_affinity_list".format(cpu,irq)
    if DEBUG:
        print(cmd)
    exec_cmd(cmd)

def main():
    parser = argparse.ArgumentParser(description="Auto-Trader Remote Client")
    parser.add_argument("-d", "--debug",  type=bool,default=False,          help="DEBUG mode")
    parser.add_argument("-A", "--action", type=str, default="get",          help="")
    parser.add_argument("-N", "--name",   type=str, default="virtio5.input",help="NIC name")
    parser.add_argument("-C", "--cpus",   type=str, default="0-31",         help="CPU for NIC irq")
    args = parser.parse_args()
    global DEBUG
    DEBUG=args.debug
    if args.action=="get":
        irqs=get_interrupt(args.name)
        print(len(irqs),"irqs ", irqs[0], "~", irqs[-1])
    elif args.action=="set":
        cpu_list=args.cpus.split(",")
        if "-" in args.cpus:
            cpus=args.cpus.split("-")
            cpu_list=range(int(cpus[0]), int(cpus[1])+1)
        irqs=get_interrupt(args.name)
        if len(irqs)!=len(cpu_list):
            print("cpu ",len(cpu_list)," needs same size list of irq ",len(irqs))
            return
        for idx,irq in enumerate(irqs):
            set_interrupt(irq,cpu_list[idx])
main()