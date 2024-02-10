use ram_machine::interpreter::RamMachine;
use ram_machine::parser::RamCode;

use std::fs::read_to_string;
use std::str::FromStr;

#[test]
fn three_sum() {
    let path = "./examples/three_sum.ram";
    let file = read_to_string(path).unwrap();
    let code = RamCode::from_str(&file).unwrap();
    let input1: Vec<i64> = vec![1, 3, 2];
    let input2: Vec<i64> = vec![-1232323, 34324, 92384];
    let input3: Vec<i64> = vec![324, 546, 8023];
    let input4: Vec<i64> = vec![3209847, 16879823, 27034];
    let input5: Vec<i64> = vec![0, 0, 0];
    assert_eq!(
        RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
        vec![input1.iter().sum()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
        vec![input2.iter().sum()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
        vec![input3.iter().sum()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
        vec![input4.iter().sum()]
    );
    assert_eq!(
        RamMachine::new(code, input5.clone()).run().unwrap(),
        vec![input5.iter().sum()]
    );
}

#[test]
fn square() {
    let path = "./examples/square.ram";
    let file = read_to_string(path).unwrap();
    let code = RamCode::from_str(&file).unwrap();
    let input1 = vec![36];
    let input2 = vec![0];
    let input3 = vec![1_000_000_000];
    let input4 = vec![978314014];
    let input5 = vec![32423];
    assert_eq!(
        RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
        vec![input1[0] * input1[0]]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
        vec![input2[0] * input2[0]]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
        vec![input3[0] * input3[0]]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
        vec![input4[0] * input4[0]]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
        vec![input5[0] * input5[0]]
    );
}

#[test]
fn sequence_length() {
    let path = "./examples/sequence_length.ram";
    let file = read_to_string(path).unwrap();
    let code = RamCode::from_str(&file).unwrap();
    let input1 = vec![1, 2, 4, 6, 8, 9, 10, 0];
    let input2 = vec![1, 5, 6, 7, 0];
    let mut input3 = vec![66; 66];
    input3.push(0);
    let input4 = vec![0];
    let mut input5 = vec![40; 1_000_000];
    input5.push(0);
    assert_eq!(
        RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
        vec![(input1.len() - 1).try_into().unwrap()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
        vec![(input2.len() - 1).try_into().unwrap()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
        vec![(input3.len() - 1).try_into().unwrap()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
        vec![(input4.len() - 1).try_into().unwrap()]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
        vec![(input5.len() - 1).try_into().unwrap()]
    );
}

#[test]
fn log() {
    let path = "./examples/log.ram";
    let file = read_to_string(path).unwrap();
    let code = RamCode::from_str(&file).unwrap();
    let input1 = vec![2, 2 << 31];
    let input2 = vec![5, 1];
    let input3 = vec![3, 55];
    let input4 = vec![3, 999_999_999_999_999_999];
    let input5 = vec![40, 1_000_000_000];
    assert_eq!(
        RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
        vec![32]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
        vec![0]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
        vec![3]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
        vec![37]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
        vec![5]
    );
}

#[test]
fn unit_digit() {
    let path = "./examples/unit_digit.ram";
    let file = read_to_string(path).unwrap();
    let code = RamCode::from_str(&file).unwrap();
    let input1 = vec![320423789];
    let input2 = vec![-234234235];
    let input3 = vec![999_999_999_999_999_991];
    let input4 = vec![0];
    let input5 = vec![576];
    assert_eq!(
        RamMachine::new(code.clone(), input1.clone()).run().unwrap(),
        vec![9]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input2.clone()).run().unwrap(),
        vec![5]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input3.clone()).run().unwrap(),
        vec![1]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input4.clone()).run().unwrap(),
        vec![0]
    );
    assert_eq!(
        RamMachine::new(code.clone(), input5.clone()).run().unwrap(),
        vec![6]
    );
}
