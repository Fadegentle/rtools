use std::io;

use super::{Tool, ToolBuilder, ToolFunc};

pub(crate) struct WorkTimeCalculator(String);

impl ToolBuilder for WorkTimeCalculator {
    fn build(name: impl ToString) -> Tool {
        Tool::WorkTimeCalculator(WorkTimeCalculator(name.to_string()))
    }
}

impl ToolFunc for WorkTimeCalculator {
    fn get_info(&self) -> String {
        self.0.to_string()
    }

    fn run() {
        println!("\n你的上班时间（格式：8:30）是：");
        let mut punch_time = String::new();
        match io::stdin().read_line(&mut punch_time) {
            Err(e) => {
                println!("输入错误，请检查输入是否正确");
                error!("解析失败{}", e)
            },
            Ok(_) => {
                let (h, m) = parse_time(punch_time).unwrap();
                // TODO 12 times calculator
                let closing_h = h + 8;
                let closing_m = m;
                let closing_time = format!("{}:{}", closing_h, closing_m);
                println!("\n你的下班时间是：{}", closing_time);
            },
        }
    }
}

// TODO use result
fn parse_time(time: impl ToString) -> Option<(usize, usize)> {
    let t = time.to_string();
    let res: Vec<&str> = t.trim().split(':').collect();
    // TODO dont unwrap
    if let Ok(h) = res.first().unwrap().parse::<usize>() {
        if let Ok(m) = res.get(1).unwrap().parse::<usize>() {
            return Some((h, m));
        }
    }
    error!("解析时间失败");
    None
}
