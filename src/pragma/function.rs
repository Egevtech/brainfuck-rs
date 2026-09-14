use crate::optimizations::OptTokens;

pub struct Function {
    pub ind: String,
    pub argcount: u8,
}

impl From<String> for Function {
    /* exfn<name, argcount> */
    fn from(mut value: String) -> Self {
        if !value.starts_with("<") || !value.ends_with(">") {
            panic!("Invalid function defenition. Use `#exfn <fn_name, arg_count>`");
        }

        value.remove(0);
        value.remove(value.len() - 1);

        let parts = value.split(',').collect::<Vec<&str>>();
        let name = parts[0].to_string();

        let count = parts[1]
            .parse::<u8>()
            .expect(&("Failed to parse argcount in function ".to_string() + &name));

        Function {
            ind: name,
            argcount: count,
        }
    }
}

impl Function {
    pub fn codegen(self, content: OptTokens) -> String {
        let mut res = String::new();

        res += &format!("int {}(int argnum, ...) {{", self.argcount);

        

        res += &format!("\treturn vec->data[vec->current];\n");

        String::new()
    }
}
