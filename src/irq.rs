use std::collections::HashSet;
use polars::{lazy::dsl::col, prelude::*};

const INT:&str="/proc/interrupts";
static mut DEBUG:bool=false;
fn exec(cmd: &str) -> (String,String){
    let args=&["-c", cmd];
    let output = std::process::Command::new("sh")
    .args(args)
    .output()
    .expect("failed to execute cmd");
 
    let stdout=String::from_utf8(output.stdout).unwrap().trim().to_owned();
    let stderr=String::from_utf8(output.stderr).unwrap().trim().to_owned();
    
    (stdout,stderr)
 }

fn disable_service(){
    //cmd="systemctl enable irqbalance"
    //cmd="systemctl start irqbalance"
    exec("systemctl disable irqbalance");
    exec("systemctl stop irqbalance");
}

fn get_int_df(name:&str)->DataFrame{
    // gen data.df
    write_int_title(name);
    write_int_number(name);
    // use data.df
    let cmd="sed 's/^[ ]*//' data.df|sed 's/ \\{1,\\}/,/g'|sed 's/:,/,/g' > data1.df";  // |sed 's/,0,/,,/g'
    exec(cmd);
    //println!("ret:{ret}");
    /////////////////////////////////////////////
    let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("data1.df".into())).unwrap().finish().unwrap();
    let mut df = df.drop_nulls::<String>(None).unwrap();
    //println!("{:?}", df);
    for name in df.clone().get_column_names(){
        if name.starts_with("CPU"){
            let max_val=df[name].max::<i64>().unwrap().unwrap();
            if max_val==0{
                let _=df.drop_in_place(name);
            }
        }
    }
    for coln in vec!["A","B","C","D","E","F","G"]{
        if df.get_column_names().contains(&coln){
            let val= df[coln].iter().last().unwrap().to_string();
            if val.contains(name){
                df=df.lazy().with_columns([
                    col(coln).alias("DEV"),
                    col("ID").alias("IRQ"),
                    ]).collect().unwrap();
                let _=df.drop_in_place(coln);
                let _=df.drop_in_place("ID");
            }else{
                let _=df.drop_in_place(coln);
            }
        }
    }
    //let names=Option<&str>;
    //let tdf=df.transpose(None,None).unwrap();
    //println!("{:?}", tdf);
    //let df2 = tdf.drop_nulls::<String>(None).unwrap();
    //let df2=df1.lazy().filter(col("zero").eq(lit(0))).collect().unwrap();
    //let df2=tdf.lazy().filter(col("numeric").gt(lit(0))).collect().unwrap();
    //println!("{:?}", df2);
    df
}
fn get_title()->String{
    let cmd=format!("head -1 {INT}");
    exec(&cmd).0
}
fn count_title()->u32{
    let cmd=format!("head -1 {INT}|wc -w ");
    let val:u32=exec(&cmd).0.parse().unwrap();
    return val+1;
}
fn count_int(name:&str)->u32{
    let cmd=format!("grep {name} {INT}|head -1|wc -w");
    //println!("count_int cmd: {cmd}");
    let val:u32=exec(&cmd).0.parse().unwrap();
    return val;
}
fn count_cpu()->usize{
    let cmd="lscpu|grep '^CPU(s)'|awk '{{print $NF}}'";
    let val:usize=exec(&cmd).0.parse().unwrap();
    return val;
}
fn get_pcie_bus_list(gpu:usize)->Vec<String>{
    let cmd=format!("lspci -d:{gpu}|awk '{{print $1}}'");
    let bus_list:Vec<String>=exec(&cmd).0.lines().map(|v| v.to_string()).collect();
    let uniq_list: HashSet<String> = bus_list.into_iter().collect();
    let mut v: Vec<_> = uniq_list.into_iter().collect();
    v.sort();
    return v
}
fn write_int_title(name:&str)->String{
    let title_n=count_title();
    let intr_n=count_int(name);
    let title=get_title();
    let gap=intr_n-title_n;
    let append;
    if gap==1{
        append="A";
    }else if gap==2{
        append="A   B";
    }else if gap==3{
        append="A   B   C"
    }else if gap==4{
        append="A   B   C   D"
    }else if gap==5{
        append="A   B   C   D   E"
    }else if gap==6{
        append="A   B   C   D   E   F"
    }else if gap==7{
        append="A   B   C   D   E   F   G"
    }else{append="";println!("title={title}\ntitle_n={title_n} intr_n={intr_n} gap={gap}");}
    let cmd=format!("echo 'ID {title} {append}' > data.df");
    return exec(&cmd).0;
}
fn write_int_number(name:&str)->String{
    let cmd=format!("grep {name} {INT} >> data.df");
    return exec(&cmd).0;
}
fn get_dyn_int(name:&str,statc:bool)->DataFrame{
    let df1 = get_int_df(name);
    if statc{
        return df1.lazy().select(
            [col("DEV"),
            col("IRQ")
            ]).collect().unwrap()
        }
    //println!("{:?}",df1);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let df2 = get_int_df(name);
    //println!("{:?}",df2);
    let delta_df = df1.inner_join(&df2, ["DEV","IRQ"], ["DEV","IRQ"]).unwrap();
    
    unsafe {
        if DEBUG{
            println!("delta_df {:?}",delta_df);
        }
    };
    /*
┌──────┬────────────┬─────────┬─────────────────────────────┬────────────┬────────────┐
│ ID   ┆ CPU0       ┆ CPU8    ┆ D                           ┆ CPU0_right ┆ CPU8_right │
│ ---  ┆ ---        ┆ ---     ┆ ---                         ┆ ---        ┆ ---        │
│ i64  ┆ i64        ┆ i64     ┆ str                         ┆ i64        ┆ i64        │
╞══════╪════════════╪═════════╪═════════════════════════════╪════════════╪════════════╡
│ 1071 ┆ 33         ┆ 0       ┆ vfio-msix[0](0000:01:00.1)  ┆ 33         ┆ 0          │
│ 1072 ┆ 2301253882 ┆ 2848484 ┆ vfio-msix[1](0000:01:00.1)  ┆ 2301253882 ┆ 2850146    │
│ 1073 ┆ 110691612  ┆ 130763  ┆ vfio-msix[2](0000:01:00.1)  ┆ 110691612  ┆ 130850     │
│ 1074 ┆ 0          ┆ 0       ┆ vfio-msix[3](0000:01:00.1)  ┆ 0          ┆ 0          │
│ 1075 ┆ 36907411   ┆ 43595   ┆ vfio-msix[4](0000:01:00.1)  ┆ 36907411   ┆ 43623      │
│ …    ┆ …          ┆ …       ┆ …                           ┆ …          ┆ …          │
│ 1082 ┆ 0          ┆ 0       ┆ vfio-msix[11](0000:01:00.1) ┆ 0          ┆ 0          │
│ 1083 ┆ 0          ┆ 0       ┆ vfio-msix[12](0000:01:00.1) ┆ 0          ┆ 0          │
│ 1084 ┆ 0          ┆ 0       ┆ vfio-msix[13](0000:01:00.1) ┆ 0          ┆ 0          │
│ 1085 ┆ 0          ┆ 0       ┆ vfio-msix[14](0000:01:00.1) ┆ 0          ┆ 0          │
│ 1086 ┆ 0          ┆ 0       ┆ vfio-msix[15](0000:01:00.1) ┆ 0          ┆ 0          │
└──────┴────────────┴─────────┴─────────────────────────────┴────────────┴────────────┘
    */
    let mut sum_df=DataFrame::default();
    for name in delta_df.clone().get_column_names(){
        if name.starts_with("CPU")&&!name.ends_with("right"){
            //println!("{:?}",delta_df);
            let name2=format!("{name}_right");
            let cpu=name.replace("CPU", "");
            let df=delta_df.clone().lazy().with_columns([
                (col(&name2)-col(name)).alias("delta"),
                col("IRQ"),
                lit(cpu).alias("CPU"),
            ]).filter(
                col("delta").gt(0)
            ).collect().unwrap();
            //println!("{:?}",df);
            if df.shape().0<1{continue}
            else{sum_df=sum_df.vstack(&df).unwrap();}
            //let _=delta_df.drop_in_place(&name2);
            //let _=delta_df.drop_in_place(&name);
            //let _=delta_df.drop_in_place("delta");
        }
    }
    unsafe {
        if DEBUG{
            println!("sum_df {:?}",sum_df);
        }
    };
    if sum_df.shape().0<1{
        println!("No interrupts captured, maybe try --static mode");
    }else{
        sum_df=sum_df.lazy().select([col("DEV"),col("IRQ"),col("CPU")]).collect().unwrap();
    }
    sum_df
}
fn set_dyn_int(name:&str,cpu:usize,dym:bool){
    let df=get_dyn_int(&name,dym);
    println!("{:?}",df);
    let irq_vec: Vec<i64> = df["IRQ"].i64().unwrap().into_no_null_iter().collect();
    println!("set irqs {:?} -> {cpu}",irq_vec);
    for irq in irq_vec{
        let file=format!("/proc/irq/{irq}/smp_affinity_list");
        let cmd=format!("echo {cpu} > {file}");
        let (_,err)=exec(&cmd);
        if err.len()>0{
            println!("err:{err}");
        }
    }
}

fn get_options() -> (bool,bool,String,String,usize,usize) {
    let matches = clap::Command::new("Interrupt Binding Tool")
        .version("v0.0.1 20240701")
        .author("Weixing Sun <weixing.sun@gmail.com>")
        .about("Perf Toolbox")
        .arg(clap::arg!(--debug).required(false).help("debug mode, default: false"))
        .arg(clap::arg!(--mode <VALUE>).required(true).help("[get/set]_all_32c"))
        .arg(clap::arg!(--static).required(false).help("scan type, default: dynamic"))
        .arg(clap::arg!(--name <VALUE>).required(false).help("unique name like: 0000:01:00.1"))
        .arg(clap::arg!(--cpu <VALUE>).required(false).help("bind cpu, default: 0"))
        .arg(clap::arg!(--gpu <VALUE>).required(false).help("gpu type, default: 200"))
        .get_matches();
    let debug = *matches.get_one::<bool>("debug").unwrap();
    let statc = *matches.get_one::<bool>("static").unwrap();
    let mode = matches.get_one::<String>("mode").unwrap().to_owned();
    let name = matches.get_one::<String>("name");
    let name = if name.is_none() {"0000:01:00.1"} else {name.unwrap()};
    let cpu = matches.get_one::<String>("cpu");
    let cpu = if cpu.is_none() {0} else {cpu.unwrap().parse().unwrap()};
    let gpu = matches.get_one::<String>("gpu");
    let gpu = if gpu.is_none() {200} else {gpu.unwrap().parse().unwrap()};
    return (debug,statc,mode,name.to_owned(),cpu,gpu)
}

fn main() {
    let (debug,statc,mode,name,cpu,gpu)=get_options();
    unsafe { DEBUG=debug };
    std::env::set_var("POLARS_FMT_MAX_COLS", "28");
    disable_service();
    if mode.eq("get"){
        let df=get_dyn_int(&name,statc);
        println!("{:?}",df);
    }else if mode.eq("set"){
        set_dyn_int(&name,cpu,statc);
    }else if mode.contains("all_32c"){
        let bus_list=get_pcie_bus_list(gpu);
        println!("{:?}",bus_list);
        let n_pci=bus_list.len();  // 8*4=32
        let n_cpu=count_cpu();     // 256
        let ngroup=n_cpu/n_pci;    // 256/32=8
        for i in 0..n_pci{
            let pci=&bus_list[i];
            let cpu_idx=i*ngroup;
            let step=4;
            let step_idx=step*ngroup*(cpu_idx/(ngroup*step));
            println!("pci={pci} cpu={step_idx}");
            if mode.contains("set"){
                set_dyn_int(pci,step_idx,statc);
            }else{
                let df=get_dyn_int(pci,statc);
                println!("{:?}",df);
            }
        }
    }
}
