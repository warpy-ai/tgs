mod path;

pub fn run_process(vars: &Vec<String>, commang: &str) -> Recult<(), ()> {
    let bin = path::find_binary(commang, &vars[0]);
    OK(())
}
