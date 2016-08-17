use std::io::prelude::*;
use std::fs::File;

pub mod hdl_parser;
pub mod ast;

macro_rules! hdl {
    ( $( $x:tt )* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(stringify!($x));
            )*
            let concat = temp_vec.join(" ");

            println!("Parsing {:?}", concat);
            let res = hdl_parser::parse_Module(&concat);

            res.unwrap()
        }
    };
}

fn indent(input: &str) -> String {
    format!("{}    ", input)
}

pub trait ToVerilog {
    fn to_verilog(&self, ind: &str) -> String;
}

impl ToVerilog for ast::Ident {
    fn to_verilog(&self, _: &str) -> String {
        self.0.clone()
    }
}

impl ToVerilog for ast::Dir {
    fn to_verilog(&self, _: &str) -> String {
        (match *self {
            ast::Dir::In => "input",
            ast::Dir::Out => "output",
        }).to_string()
    }
}

impl ToVerilog for ast::Op {
    fn to_verilog(&self, _: &str) -> String {
        (match *self {
            ast::Op::Add => "+",
            ast::Op::Sub => "-",
            ast::Op::Mul => "*",
            ast::Op::Div => "/",
            ast::Op::Eq => "==",
        }).to_string()
    }
}

impl ToVerilog for ast::Decl {
    fn to_verilog(&self, ind: &str) -> String {
        match *self {
            ast::Decl::Reg(ref i) => {
                format!("{ind}reg {name} = 0;", ind=ind, name=i.to_verilog(ind))
            }
            ast::Decl::RegArray(ref i, ref e) => {
                format!("{ind}reg [{len}:0] {name} = 0;",
                    ind=ind,
                    len=e.to_verilog(ind),
                    name=i.to_verilog(ind))
            }
            ast::Decl::On(ref edge, ref block) => {
                format!("{ind}always @(posedge clk) begin\n{body}{ind}end",
                    ind=ind,
                    body=block.to_verilog(&indent(ind)))
            }
            ast::Decl::Always(ref block) => block.to_verilog(ind),
        }
    }
}

impl ToVerilog for ast::SeqBlock {
    fn to_verilog(&self, ind: &str) -> String {
        self.0.iter().map(|x| x.to_verilog(ind)).collect::<Vec<_>>().join("")
    }
}

impl ToVerilog for ast::Seq {
    fn to_verilog(&self, ind: &str) -> String {
        match *self {
            ast::Seq::If(ref c, ref t, ref f) => {
                format!("{ind}if ({cond}) begin\n{body}{ind}end\n{f}",
                    ind=ind,
                    cond=c.to_verilog(ind),
                    body=t.to_verilog(&indent(ind)),
                    f=f.as_ref().map_or("".to_string(), |e| {
                        format!("{ind}else begin\n{body}{ind}end\n",
                            ind=ind,
                            body=e.to_verilog(&indent(ind)))
                    }))
            },
            ast::Seq::Set(ref id, ref value) => {
                format!("{ind}{name} <= {value};\n",
                    ind=ind,
                    name=id.to_verilog(ind),
                    value=value.to_verilog(ind))
            }
        }
    }
}

impl ToVerilog for ast::CombBlock {
    fn to_verilog(&self, ind: &str) -> String {
        self.0.iter().map(|x| x.to_verilog(ind)).collect::<Vec<_>>().join("")
    }
}

impl ToVerilog for ast::Comb {
    fn to_verilog(&self, ind: &str) -> String {
        match *self {
            ast::Comb::If(ref c, ref t, ref f) => {
                format!("{ind}if ({cond}) begin\n{body}{ind}end\n{f}",
                    ind=ind,
                    cond=c.to_verilog(ind),
                    body=t.to_verilog(&indent(ind)),
                    f=f.as_ref().map_or("".to_string(), |e| {
                        format!("{ind}else begin\n{body}{ind}end\n",
                            ind=ind,
                            body=e.to_verilog(&indent(ind)))
                    }))
            },
            ast::Comb::Assign(ref id, ref value) => {
                format!("{ind}assign {name} = {value};\n",
                    ind=ind,
                    name=id.to_verilog(ind),
                    value=value.to_verilog(ind))
            }
        }
    }
}

impl ToVerilog for ast::Expr {
    fn to_verilog(&self, ind: &str) -> String {
        match *self {
            ast::Expr::Num(v) => format!("{}", v),
            ast::Expr::Ref(ref id) => id.to_verilog(ind),
            ast::Expr::Slice(ref id, ref from, ref to) => {
                format!("{}[{}{}]", id.to_verilog(ind), from.to_verilog(ind),
                    to.as_ref().map_or("".to_string(), |x| {
                        format!(":{}", x.to_verilog(ind))
                    }))
            }
            ast::Expr::Concat(ref body) => {
                format!("{{{}}}", body.iter().map(|x| {
                    x.to_verilog(ind)
                }).collect::<Vec<_>>().join(", "))
            }
            ast::Expr::Arith(ref op, ref l, ref r) => {
                format!("{left} {op} {right}",
                    left=l.to_verilog(ind),
                    op=op.to_verilog(ind),
                    right=r.to_verilog(ind))
            }
        }
    }
}

impl ToVerilog for ast::Module {
    fn to_verilog(&self, ind: &str) -> String {
        format!("{ind}module rename ({args});\n{body}{ind}endmodule\n",
            ind=ind,
            args=self.0.iter().map(|x| {
                format!("{} {}", x.1.to_verilog(ind), x.0.to_verilog(ind))
            }).collect::<Vec<_>>().join(", "),
            body=self.1.iter().map(|x| {
                x.to_verilog(&indent(ind))
            }).collect::<Vec<_>>().join("\n"))
    }
}

fn main() {
    let code = hdl! {

module (clk: in, LED1: out, LED2: out, LED3: out, LED4: out, LED5: out) {
    reg rot[3];
    reg ready;
    reg divider[20];

    on (clk.posedge) {
        if ready {
            if divider == 1200000 - 1 {
                divider <= 0;
                rot <= {rot[3-1:0], rot[3]};
            } else {
                divider <= divider + 1;
            }
        } else {
            ready <= 1;
            rot <= 0b0001;
            divider <= 0;
        }
    }

    always {
        LED1 = rot[0];
        LED2 = rot[1];
        LED3 = rot[2];
        LED4 = rot[3];
        LED5 = 1;
    }
}

};

    println!("");
    println!("AST: {:?}", code);

    let verilog = code.to_verilog("");

    println!("");
    println!("Verilog:\n{}", verilog);

    let mut f = File::create("out/test.v").unwrap();
    f.write_all(verilog.as_bytes()).unwrap();

    println!("");
    println!("File written as out/test.v");
}
