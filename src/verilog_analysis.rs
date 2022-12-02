use std::fs::File;
use same_file::Handle;
use std::io::{Error};
use std::path::{Path};
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

pub struct AnalysisData {
    pub module_name : Vec<String>, 
    pub inputs : HashMap<String, i32>,
    pub outputs : HashMap<String, i32>,
    pub wires : HashMap<String, i32>,
    pub regs : HashMap<String, i32>,
    pub cont_assigns : HashMap<String, i32>,
    pub always_assigns : HashMap<String, String>
}

pub fn verilog_analysis(path : &Path) -> AnalysisData {
    let path_to_read = Path::new(path);
    Handle::stdout().unwrap();
    Handle::from_path(path_to_read).unwrap();

    let mut data = AnalysisData {
        module_name : Vec::new(),
        inputs : HashMap::new(),
        outputs : HashMap::new(),
        wires : HashMap::new(),
        regs : HashMap::new(),
        cont_assigns : HashMap::new(),
        always_assigns : HashMap::new()        
    };

    let file = File::open(&path_to_read).unwrap();
    let buf_file = BufReader::new(file);
    for line in buf_file.lines() {
        let mut token : Vec<String> = Vec::new();
        let liner = line.unwrap();
        token.push(liner);

        // data.module_name = Vec::new();
        let premodule_name = get_module_name(&token[0]);
        if premodule_name != "".to_string() {
            data.module_name.push(premodule_name);
        }

        let (input, dimension) = get_inputs(&token[0]);
        if input != "".to_string() {
            data.inputs.insert(input, dimension);
        }

        let (output, dimension) = get_outputs(&token[0]);
        if output != "".to_string() {
            data.outputs.insert(output, dimension);
        }

        let (wire, dimension) = get_wires(&token[0]);
        if wire != "".to_string() {
            data.wires.insert(wire, dimension);
        }
        
        let (reg, dimension) = get_regs(&token[0]);
        if reg != "".to_string() {
            data.regs.insert(reg, dimension);
        }

        let (always_event, always_assign) = get_alwayses(&token[0]);
        data.always_assigns.insert(always_event, always_assign);

        let (pre_signals, pre_lut_cmd) = get_assign(&token[0]);
        if pre_signals != ""{
            data.cont_assigns.insert(pre_signals, pre_lut_cmd);
        }
    }

    // verilog_synthesis::synthesis(
    //     module_name,
    //     inputs,
    //     outputs,
    //     wires,
    //     regs,
    //     cont_assigns,
    //     always_assigns
    // );

    return data;
}

fn get_module_name(line_r : &String) -> String {
    if Regex::new(r"\s*module \w+\s*").unwrap().is_match(&line_r.to_string())  {
        let regex = Regex::new(r"(\w+)\($").unwrap();
        let module: Vec<&str> = regex.find_iter(&line_r).map(|x| x.as_str()).collect();
        return (module[0]).to_string().replace("(", "");
    }
    return "".to_string();
}
fn get_inputs(line_r : &String) -> (String, i32) {
    let input_regex = Regex::new(r"(input( reg|wire)?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)").unwrap();
    
    if input_regex.is_match(&line_r.to_string()) {
        let io_name: Vec<&str> = Regex::new(r"([a-z]+,?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

        if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
            let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            return (
                io_name[0].to_string().replace(",", ""),
                dimension[0].parse::<i32>().unwrap() + 1
            );
        }
        else {
            return(
                io_name[0].to_string().replace(",", ""),
                1
            );
        }
    
    }
    return ("".to_string(), 0);
}

fn get_outputs(line_r : &String) -> (String, i32) {
    let output_regex = Regex::new(r"(output( reg|wire)?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)").unwrap();
    
    if output_regex.is_match(&line_r.to_string()) {
        let io_name: Vec<&str> = Regex::new(r"([a-z]+,?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

        if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
            let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            return (
                io_name[0].to_string().replace(",", ""),
                dimension[0].parse::<i32>().unwrap() + 1
            );
        }
        else {
            return(
                io_name[0].to_string().replace(",", ""),
                1
            );
        }
    
    }
    return ("".to_string(), 0);
}

fn get_wires(line_r : &String) -> (String, i32) {
    let wire_regex = Regex::new(r"^(wire?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)+;").unwrap();
    if wire_regex.is_match(&line_r.to_string()) {
        let wire_name: Vec<&str> = Regex::new(r"([a-z]+,?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

        if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
            let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            return (
                wire_name[0].to_string().replace(",", ""),
                dimension[0].parse::<i32>().unwrap() + 1
            );
        }
        else {
            return(
                wire_name[0].to_string().replace(",", ""),
                1
            );
        }
    
    }
    return ("".to_string(), 0);
}

fn get_regs(line_r : &String) -> (String, i32) {
    let reg_regex = Regex::new(r"^(reg?\s+(\[[0-9]:[0-9]?\])?\s*(\w+),?)+;").unwrap();
    if reg_regex.is_match(&line_r.to_string()) {
        let reg_name: Vec<&str> = Regex::new(r"([a-z]+,?)$").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();

        if Regex::new(r"(\[[0-9]:[0-9]?\])").unwrap().is_match(&line_r.to_string()) {
            let dimension: Vec<&str> = Regex::new(r"\d").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            return (
                reg_name[0].to_string().replace(",", ""),
                dimension[0].parse::<i32>().unwrap() + 1
            );
        }
        else {
            return(
                reg_name[0].to_string().replace(",", ""),
                1
            );
        }
    
    }
    return ("".to_string(), 0);
}

fn get_assign(line_r : &String) -> (String, i32) {
    let assign_regex = Regex::new(r"^\s+(assign ((\w+ = \w+\s+[&|+-]+\s+\w+)|(\w+ [&|+-]+= \w+));\s?)").unwrap();

    if assign_regex.is_match(&line_r.to_string()) {
        let signals: Vec<&str> = Regex::new(r"\w+ = \w+\s+[&|+-]\s+\w+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
        let operation: Vec<&str> = Regex::new(r"[&|+-]+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
        let lut_cmd: i32 = match operation[0] {
            "&&" => 6,
            "||" => 5,
            "&" => 4,
            "|" => 3,
            "+" => 2,
            "-" => 1,
            _   => 0
        };
        return (
            signals[0].to_string(),
            lut_cmd
        );
    }
    
    return ("".to_string(), 0);
}

fn get_alwayses(line_r : &String) -> (String, String) {
    let always_regex = Regex::new(r"^\s*(always(_ff)? @\((pos|neg)edge \w+\) begin\s*)").unwrap();
    let mut event : String = "".to_string();
    let mut always_detect:bool = false;

    if !always_detect && always_regex.is_match(&line_r.to_string()) {
        event = Regex::new(r"edge\s+(\w+)\s*").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
        always_detect = true;
    }
    else if always_detect && Regex::new(r"^\s*end\s*").unwrap().is_match(&line_r.to_string()){
        always_detect = false;
    }
    else if always_detect{
        if Regex::new(r"^\s*((\w+ <?= \w+\s+[&|+-]+\s+\w+;\s?)|(\w+ [&|+-]+= \w+))").unwrap().is_match(&line_r.to_string()) {
            let signals: Vec<&str> = Regex::new(r"\w+ <?= \w+\s+[&|+-]\s+\w+").unwrap().find_iter(&line_r).map(|x| x.as_str()).collect();
            return (
                event.replace("edge", ""),
                signals[0].to_string(),
            );
        
        }
    }
    return ("".to_string(), "".to_string());
}
