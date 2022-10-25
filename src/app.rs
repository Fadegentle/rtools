use std::io;

enum Tools {
    WorkTimeCalculator(WTC),
    Other(Ot),
}
struct WTC;
struct Ot;

pub(crate) fn run() {
    info!("应用启动成功");
    loop {
        home();
        let mut tool_n = String::new();
        match io::stdin().read_line(&mut tool_n) {
            Err(e) => {
                println!("输入错误，请检查是否为合适数字");
                error!("读取数字失败{}", e)
            }
            Ok(_) => match tool_n.trim_end().parse::<usize>() {
                Err(e) => {
                    error!("解析工具号失败 {}", e);
                }
                Ok(tool_n) => {
                    Tools::get_tool(tool_n);
                }
            },
        }
    }
}

fn home() {
    println!(">>>>>>  请选择你想使用的工具  <<<<<<");
    Tools::display_info();
}

impl Tools {
    fn display_info() {
        WTC::get_info();
        Ot::get_info();
    }

    fn get_tool(tool_n: usize) {
        match tool_n {
            1 => WTC::run(),
            _ => Ot::run(),
        };
    }
}

trait Tool {
    fn get_info();
    fn run();
}

impl Tool for WTC {
    fn get_info() {
        println!("1. 工时计算器");
    }

    fn run() {
        println!("你的上班时间（格式：8:30）是：");
        let mut punch_time = String::new();
        match io::stdin().read_line(&mut punch_time) {
            Err(e) => {
                println!("输入错误，请检查输入是否正确");
                error!("解析失败{}", e)
            }
            Ok(_) => {
                let (h, m) = parse_time(punch_time).unwrap();
                // TODO 12 times calculator
                let closing_h = h + 8;
                let closing_m = m;
                let closing_time = format!("{}:{}", closing_h, closing_m);
                println!("你的下班时间是：{}", closing_time);
            }
        }
    }
}

// TODO use result
fn parse_time(time: impl ToString) -> Option<(usize, usize)> {
    let t = time.to_string();
    let res: Vec<&str> = t.trim().split(':').collect();
    // TODO dont unwrap
    if let Ok(h) = res.get(0).unwrap().parse::<usize>() {
        if let Ok(m) = res.get(1).unwrap().parse::<usize>() {
            return Some((h, m));
        }
    }
    error!("解析时间失败");
    None
}

impl Tool for Ot {
    fn get_info() {
        println!("0. 其他");
    }

    fn run() {
        println!("敬请期待......");
    }
}
