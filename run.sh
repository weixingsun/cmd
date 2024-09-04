# export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
# export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo build --release
target/release/cmd --mode=set_all_32c
cp target/release/cmd tune_irq_arm
exit
#############################################
#IRQ=old_irq
#ssh 192.168.29.77 /root/irq_set_vm1.sh
#ssh 192.168.29.88 /root/irq_set_host.sh

IRQ=new_irq
scp $IRQ/irq_set_vm1.sh 192.168.29.77:/root/
#set all irq to bind cpu
ssh 192.168.29.77 /root/irq_set_vm1.sh set
#set idx irq to bind cpu
ssh 192.168.29.77 /root/irq_set_vm1.sh set 1

scp $IRQ/irq_set_host.sh 192.168.29.88:/root/
#set all irq to bind cpu
ssh 192.168.29.88 /root/irq_set_host.sh set
#set idx irq to bind cpu
ssh 192.168.29.88 /root/irq_set_host.sh set 1
