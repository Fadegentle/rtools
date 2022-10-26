mod other;
mod work_time_calculator;

use std::{collections::HashMap, io};

use lazy_static::lazy_static;
use other::Other;
use work_time_calculator::WorkTimeCalculator;

pub(crate) fn run() {
    info!("应用启动成功");
    loop {
        home();
        let mut tool_n = String::new();
        match io::stdin().read_line(&mut tool_n) {
            Err(e) => {
                println!("输入错误，请检查是否为合适数字");
                error!("读取数字失败{}", e)
            },
            Ok(_) => match tool_n.trim_end().parse::<usize>() {
                Err(e) => {
                    error!("解析工具号失败 {}", e);
                },
                Ok(tool_n) => {
                    get_tool(tool_n);
                },
            },
        }
    }
}

fn home() {
    println!("\n>>>>>>  请选择你想使用的工具  <<<<<<");
    display_info();
}

#[rustfmt::skip]
lazy_static! {
    static ref TOOLS: HashMap<usize, Tool> = {
        HashMap::from([
            (0, Other::build("其他")),
            (1, WorkTimeCalculator::build("工时计算器")),
        ])
    };
}

enum Tool {
    WorkTimeCalculator(WorkTimeCalculator),
    Other(Other),
}

fn display_info() {
    TOOLS.iter().for_each(|(n, t)| println!("{}.{}", n, t.get_info()))
}

fn get_tool(tool_n: usize) {
    match tool_n {
        1 => WorkTimeCalculator::run(),
        _ => Other::run(),
    };
}

impl Tool {
    fn get_info(&self) -> String {
        match self {
            Tool::WorkTimeCalculator(wtc) => wtc.get_info(),
            Tool::Other(ot) => ot.get_info(),
        }
    }
}

trait ToolBuilder {
    fn build(name: impl ToString) -> Tool;
}

trait ToolFunc {
    fn get_info(&self) -> String;
    fn run(); // TODO Result
}
