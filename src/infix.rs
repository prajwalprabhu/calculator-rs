//Solve Post fix expression
pub fn eval(eq: String) -> Result<i32,String> {
    // check(eq.clone());
    to_result(to_post(to_vec(eq)))
}
// fn check(eq: String) {
//     let eq = eq.clone();
//     let mut count = 0;
//     for i in eq.chars() {
//         if i == '(' {
//             count += 1;
//         }
//         if i == ')' {
//             count -= 1;
//         }
//     }
//     if count > 0 {
//         panic!("Braces opened but not closed");
//     }
//     if count<0{
// 		panic!("Extra closing Braces");
// 	}
// }
fn to_result(result: Vec<String>) -> Result<i32,String> {
    let mut stack: Vec<i32> = Vec::new(); //Stack
    // let mut operand: Vec<i32>
    let operator = "*/+-".to_string();
    for i in result {
        if i != "" {
            if operator.contains(&i) {
                let aa = stack.pop();
                let bb = stack.pop();
                let mut a = 0;
                let mut b = 0;
                if aa.is_some(){
                    let _a =&mut a;
                    *_a= aa.unwrap();
                }else{
                    return Err("Syntax Error".to_string());
                }
                if bb.is_some(){
                    let _b =&mut b;
                    *_b = bb.unwrap();
                }else{
                    return Err("Syntax Error".to_string());
                }
                if i == "+" {
                    stack.push(b + a);
                } else if i == "-" {
                    // if a>b{
                    //     let ris = (a-b);
                    //     stack.push(ris)
                    // }else{
                    stack.push(b - a);
                } else if i == "*" {
                    stack.push(b * a);
                } else if i == "/" {
                    stack.push(b / a);
                }
            } else {
                stack.push(i.parse().unwrap());
            }
        }
    }
    Ok(stack.pop().unwrap())
}

//Checks whether there is any power equation in given equation i.e 2^3=8 or (1+1)^3=8
// fn power(mut eq: Vec<String>) -> Vec<String> {
//     // //println!("eq = {:?}", eq);
//     let result: i32;
//     let a: String;
//     let b: String;
//     let mut eq2: Vec<String> = Vec::new();
//     if eq.contains(&'^'.to_string()) {
//         for i in 0..eq.len() - 1 {
//             if eq[i] == '^'.to_string() {
//                 a = eq[i - 1].clone();
//                 b = eq[i + 1].clone();
//                 if a == ')'.to_string() {
//                     let mut j = i - 2;
//                     let mut open = 0;
//                     eq.remove(i - 1);
//                     while j > 0 {
//                         if eq[j] == ')'.to_string() {
//                             open += 1;
//                         }
//                         if eq[j] == '('.to_string() {
//                             if open > 0 {
//                                 open -= 1;
//                             } else {
//                                 eq.remove(j);
//                                 break;
//                             }
//                         }
//                         eq2.insert(0, eq.remove(j));
//                         j -= 1;
//                     }
//                     let res2 = to_result(to_post(power(eq2.clone())));
//                     let res2 = format!("{}", res2);
//                     eq.insert(j, res2);
//                     break;
//                 } else {
//                     let a: i32 = a.parse().unwrap();
//                     let b: i32 = b.parse().unwrap();
//                     result = a.pow(b);
//                     let result = format!("{}", result);
//                     eq.remove(i - 1);
//                     eq.remove(i - 1);
//                     eq.remove(i - 1);
//                     eq.insert(i - 1, result);
//                     break;
//                 }
//             }
//         }
//     }
//     if eq.contains(&'^'.to_string()) {
//         return power(eq);
//     }
//     // to_result(to_post(eq))
//     eq
// }

fn to_vec(eq: String) -> Vec<String> {
    let mut operand = String::new();
    let operator = "*/+-()^";
    let mut result: Vec<String> = Vec::new();
    for i in eq.chars() {
        if operator.contains(i) {
            if operand != "" {
                result.push(operand.clone());
                operand = String::new();
            }
            result.push(i.to_string());
        } else {
            operand.push(i);
        }
    }
    result.push(operand);
    result
}
//Program to Convert Infix to Postfix Expression
fn to_post(eq: Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = Vec::new(); //Stack
    let mut output: Vec<String> = Vec::new(); //Output
    let operator = "*/+-";
    for i in eq {
        if i == '('.to_string() {
            stack.push(i);
        } else if operator.contains(&i) {
            loop {
                if stack.len() == 0 {
                    stack.push(i);
                    break;
                } else if stack[stack.len() - 1] == '('.to_string() {
                    stack.push(i);
                    break;
                } else if i == '*'.to_string() || i == '/'.to_string() {
                    if stack[stack.len() - 1] == '+'.to_string()
                        || stack[stack.len() - 1] == '-'.to_string()
                    {
                        stack.push(i);
                        break;
                    } else {
                        output.push(stack.pop().unwrap());
                        stack.push(i);
                    }
                    break;
                } else {
                    output.push(stack.pop().unwrap());
                }
            }
        } else if i == ')'.to_string() {
            while stack.len() != 0 {
                let a = stack.pop().unwrap();
                if a == '('.to_string() {
                    break;
                }
                output.push(a);
            }
        } else {
            output.push(i);
        }
        //println!(" Stack = {:?} , output is {:?}", stack, output);
    }
    for _ in 0..stack.len() {
        output.push(stack.pop().unwrap());
    }
    //println!(" Stack = {:?} , output is {:?}", stack, output);
    output
}
