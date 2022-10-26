use super::{Tool, ToolBuilder, ToolFunc};

pub(crate) struct Other(String);

impl ToolBuilder for Other {
    fn build(name: impl ToString) -> Tool {
        Tool::Other(Other(name.to_string()))
    }
}

impl ToolFunc for Other {
    fn get_info(&self) -> String {
        self.0.to_string()
    }

    fn run() {
        println!("敬请期待......");
    }
}
