use std;

fn main() {
    let mut iota_counter: i32 = 0;
    let mut iota = |reset| -> i32 {
        if reset {
            iota_counter = 0;
        }
        let result = iota_counter;
        iota_counter += 1;
        result
    };
    
    let op_push=iota(true);
    let op_plus=iota(false);
    let op_minus=iota(false);
    let op_dump=iota(false);
    let _count_ops=iota(false);

    let push(x, op_push) -> (i32,i32){
        return (op_push, x);
    };

    let plus(op_plus)-> (i32,i32){
        return (op_plus, 0);
    };
    
    let minus(op_minus) -> (i32,i32){
        return (op_minus, 0);
    };

    let dump(op_dump) -> (i32,i32){
        return (op_dump, 0);
    };



    fn run_command(cmd: str, args: &mut [str]){
        std::process::Command::new(cmd)
            .args(args)
            .spawn()
            .expect(cmd + " command failed to execute");
    }


    fn simulate_program(program: [i32]){
        let stack = [];
        for op in program{
            assert!(COUNT_OPS == 4, "Exhaustive handling of operations in simulation");
            if op[0] == op_push || {
                stack.append(op[1]);
            }else if op[0] == op_plus || {
                a = stack.pop();
                b = stack.pop();
                stack.append(a + b);
            }else if op[0] == op_minus || {
                a = stack.pop();
                b = stack.pop();
                stack.append(b - a);
            } else if op[0] == op_dump || {
                a = stack.pop();
                println!("{}", a);
            } else {
                assert!(false, "unreachable");
            }
        }
    }
    // TODO: unhardcode program
    let program=[
        push(34),
        push(35),
        plus(),
        dump(),
        push(500),
        push(80),
        minus(),
        dump(),
    ];

    fn usage(){
        println!("Usage: porth <SUBCOMMAND> [ARGS]");
        println!("SUBCOMMANDS:");
        println!("    sim        Simulate the program");
        println!("    com        Compile the program");
    }

    let argv: Vec<String> = std::env::args().collect();
    let argc: i32 = argv.len();


    if argc < 3 {
        usage();
        eprintln!("ERROR: Not enough arguments. Try rorth -h (soon).");
        return;
    }

    let sub_command = argv[1];

    if subcommand == "sim" {
        simulate_program(program);

    } else if subcommand == "com" {
        // compile_program(program, "output.asm");
        // call_cmd("nasm", ["-felf64", "output.asm"]);
        // call_cmd("ld", ["-o", "output", "output.o"]);
    } else {
        usage();
        println!("ERROR: unknown subcommand {}", subcommand);
        return;
    }
}