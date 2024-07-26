import numpy as np
import re
import os
import argparse
from collections import OrderedDict

class Cell:
    def __init__(self, name, area, delay, function):
        self.name = name
        self.area = area
        self.delay = delay
        self.function = function

    def __repr__(self):
        return f"GATE {self.name} {self.area} {self.delay} {self.function};"
    
def convert_upper_camel(name):
  return "".join(["_"+i.capitalize() if i!='' and i[0] in [str(i) for i in range(10)] else i.capitalize() for i in name.capitalize().split("_")])

def find_matching_parenthesis(expression, start_index):
    """找到与指定位置的左括号匹配的右括号索引。"""
    stack = 0
    for i in range(start_index, len(expression)):
        if expression[i] == '(':
            stack += 1
        elif expression[i] == ')':
            stack -= 1
            if stack == 0:
                return i
    return -1  # 如果没有找到匹配的右括号，返回-1

def format_prefix_operations(expression, prefix_ops):
    """格式化前缀操作符后的表达式。"""
    # 构建一个正则表达式来匹配操作符后面直接跟随的左括号
    pattern = rf"({'|'.join(re.escape(op) for op in prefix_ops)})(\s*)([a-zA-Z0-9_]+|\()"

    def replacer(match):
        op = match.group(1)
        spacing = match.group(2)
        following_paren = match.start(3)  # 获取左括号的位置
        if match.group(3) != "(":
          return f"({op} {match.group(3)})"  # 如果没有找到匹配的右括号，不做更改
        # 返回格式化的字符串，包括整个括号内的表达式
        else:
          return op+spacing+match.group(3)

    # 使用正则表达式替换
    formatted_expression = re.sub(pattern, replacer, expression)
    return formatted_expression

def further_process(expression, prefix_ops):
  pattern = rf"({'|'.join(re.escape(op) for op in prefix_ops)})(\()"
  matches = re.finditer(pattern, expression)
  matches_data=[(i.group(1),i.start(),i.end()) for i in matches]
  while len(matches_data)>0:
    match_data=matches_data.pop(0)
    start=match_data[1]
    left=match_data[2]-1
    right=find_matching_parenthesis(expression, left)
    inner=expression[left+1:right]
    op=match_data[0]
    expression = expression[:start]+"("+op+" ("+inner+"))"+expression[right+1:]
    matches = re.finditer(pattern, expression)
    matches_data=[(i.group(1),i.start(),i.end()) for i in matches]
  return expression

def remove_redundant_parentheses(expression):
    # 识别所有成对的括号
    def find_matching_parenthesis(expression, start_index):
        stack = []
        for i in range(start_index, len(expression)):
            if expression[i] == '(':
                stack.append(i)
            elif expression[i] == ')':
                if not stack:
                    return -1, -1
                start = stack.pop()
                if not stack:  # 如果 stack 为空，说明找到了一个完整的括号对
                    return start, i
        return -1, -1  # 如果没有找到匹配的右括号

    # 检查括号内是否只包含空白或其他括号
    def is_redundant(sub_expr):
        # 去掉所有括号和空白字符后，查看是否还有内容
        if ' ' not in sub_expr:
          return True
        else:
          sub_expr=sub_expr.replace(' ', '')
          pair=find_matching_parenthesis(sub_expr, 0)
          return sub_expr.startswith('(') and sub_expr.endswith(')') and pair[1]==len(sub_expr)-1  # 如果没有内容，则返回 True

    # 递归移除多余的括号
    def process_expression(expression):
        start, end = find_matching_parenthesis(expression, 0)
        while start != -1:
            inner_expr = expression[start+1:end]  # 获取括号内的表达式
            if is_redundant(inner_expr):
                # 如果括号内是多余的，去除整个括号部分
                expression = expression[:start] + inner_expr + expression[end + 1:]
                # 重置搜索位置
                start, end = find_matching_parenthesis(expression, 0)
            else:
                # 否则只处理括号内的表达式
                expression = expression[:start + 1] + process_expression(inner_expr) + expression[end:]
                # 继续寻找下一个括号
                start, end = find_matching_parenthesis(expression, end + 1)
        return expression

    return process_expression(expression)



def infix_to_prefix(expression,prefix_ops,op):
    def kernelize(tokens):
      if tokens[0] in prefix_ops:
        for i in tokens[1:]:
          if i in prefix_ops:
            raise ValueError("Format Error")
        assert(len(tokens)==2)
        if isinstance(tokens[1],str):
          return "(" + " ".join(tokens)+ ")"
        elif isinstance(tokens[1],list):
          return "(" + tokens[0] + " " + kernelize(tokens[1])+")"
        else:
          raise TypeError("Type Error")
      else:
        # Stack to hold operators
        ops = []
        # Stack to hold expressions
        operands = []
        operands_expression=[]
        # Process each token
        for i in range(len(tokens)):
            token = tokens[i]
            if i%2==0:
              operands.append(token)
            elif i%2==1:
              if token not in op:
                raise ValueError("Op not define.")
              else:
                ops.append(token)

        if len(set(ops))==0 and isinstance(operands[0],str):
           return operands[0]
        elif len(set(ops))==0 and isinstance(operands[0],list):
           return kernelize(operands[0])
        elif len(set(ops))!=1:
          raise ValueError("Only one op inside parenthesis")

        for index,oprand in enumerate(operands):
          if isinstance(oprand,str):
            operands_expression.append(oprand)
          elif isinstance(oprand,list):
            operands_expression.append(kernelize(oprand))
        return "("+f"{ops[0]} " + " ".join(operands_expression)+")"

    """Convert infix expression to prefix."""
    # Clean the expression and split into tokens
    tokens = expression.replace('(', ' ( ').replace(')', ' ) ').split(" ")
    tokens=[i for i in tokens if i!='' and i!=' ']
    tokens = ["?"+i if i not in ["(",")"]+op+prefix_ops else i for i in tokens]
    oprands=[i for i in tokens if i not in ["(",")"]+op+prefix_ops]

    if len(tokens)==1:
       if tokens[0] in ["(",")"]+op+prefix_ops:
          raise ValueError("Input Error" + expression)
       else:
          return tokens[0],oprands
    left_parenthesis_index=[i for i in range(len(tokens)) if tokens[i]=='(']
    right_parenthesis_index=[i for i in range(len(tokens)) if tokens[i]==')']
    if len(left_parenthesis_index)!=len(right_parenthesis_index):
        raise ValueError("Parenthesis do not balanced.")
    for i in range(len(left_parenthesis_index)):
        if (left_parenthesis_index[i]>=right_parenthesis_index[i]):
            raise ValueError("Parenthesis do not balanced.")

    # for i in reversed(range(len(left_parernthesis_index))):
    while len(left_parenthesis_index)>=1:
      right_index=right_parenthesis_index[0]
      left_indexs=[i for i in left_parenthesis_index if i<right_index]
      assert(left_indexs==list(np.sort(left_indexs))==left_parenthesis_index[:len(left_indexs)])
      left_index=left_indexs[-1]
      left_index_index=len(left_indexs)-1
      inner=tokens[left_index+1:right_index]
      tokens=tokens[:left_index]+[inner]+tokens[right_index+1:]
      left_parenthesis_index=[i-len(inner)-1 if i>left_index else i for i in left_parenthesis_index]
      left_parenthesis_index.pop(left_index_index)
      right_parenthesis_index.pop(0)
      right_parenthesis_index=[i-len(inner)-1 for i in right_parenthesis_index]
    return kernelize(tokens),list(OrderedDict.fromkeys(oprands))

def parse_genlib(file_path):
    cells = []
    current_cell = Cell("", 1.0, 1.0, "")

    gate_re = re.compile(r'^GATE (\S+)\s+(\d+\.\d+)?\s+(.*);')
    delay_re = re.compile(r'^DELAY \S+ \S+ (\d+\.\d+)')

    with open(file_path, 'r') as file:
        for line in file:
            gate_match = gate_re.match(line)
            if gate_match:
                if current_cell.name:
                    cells.append(current_cell)
                current_cell = Cell(
                    gate_match.group(1),
                    float(gate_match.group(2)) if gate_match.group(2) else 1.0,
                    1.0,
                    gate_match.group(3)
                )
                continue

            delay_match = delay_re.match(line)
            if delay_match:
                current_cell.delay = float(delay_match.group(1))

    if current_cell.name:
        cells.append(current_cell)

    return cells



# Example usage
def main():
    rule_content = """use egraph_mapping::{SimpleLanguage};
use egg::*;

pub fn external_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
    let rules=vec![
    ];

    rules
}""".splitlines()
    
    language_content = """#![recursion_limit = "256"]

pub use egg::*;


define_language! {
    pub enum SimpleLanguage {
        Symbol(Symbol),
        "out" = OUT(Vec<Id>),
        "dff" = DFF(Vec<Id>),
        "nor" = NOR(Vec<Id>),
        "and" = AND(Vec<Id>), 
        "nand" = NAND(Vec<Id>),
        "not" = NOT(Vec<Id>), 
        "or" = OR(Vec<Id>),
        "xor" = XOR(Vec<Id>),
        "buff" = BUFF(Vec<Id>),
    }
}

""".splitlines()
    
    language_enum_content = """pub fn convert_to_simple_language_enum(in_id: Vec<Id>, stripped_name: &str) -> Option<SimpleLanguage> {
    let language_enum = match stripped_name {
        "out" => {Some(SimpleLanguage::OUT(in_id))},
        "dff" => {Some(SimpleLanguage::DFF(in_id))},
        "nor" => {Some(SimpleLanguage::NOR(in_id))},
        "and" => {Some(SimpleLanguage::AND(in_id))},
        "nand" => {Some(SimpleLanguage::NAND(in_id))},
        "not" => {Some(SimpleLanguage::NOT(in_id))},
        "or" => {Some(SimpleLanguage::OR(in_id))},
        "xor" => {Some(SimpleLanguage::XOR(in_id))},
        "buff" => {Some(SimpleLanguage::BUFF(in_id))},
        _ => None,
    };
    language_enum
}""".splitlines()
    
    node_type_content = """pub fn get_node_type(enode: &SimpleLanguage) -> &str {
    match enode {
        SimpleLanguage::OUT(_) => "out",
        SimpleLanguage::DFF(_) => "dff",
        SimpleLanguage::NOR(_) => "nor",
        SimpleLanguage::AND(_) => "and",
        SimpleLanguage::NAND(_) => "nand",
        SimpleLanguage::NOT(_) => "not",
        SimpleLanguage::OR(_) => "or",
        SimpleLanguage::XOR(_) => "xor",
        SimpleLanguage::BUFF(_) => "buff",
        _ => "",
    }
}""".splitlines()
    
    cell_input_string="""pub fn get_input_list(cell:String) -> Vec<&'static str> {
    match cell.as_str() {
        _ =>{
            panic!("Invalid input for {}", cell);
        }
    }
}""".splitlines()
    
#     cell_name_string="""pub fn cell_name_converter(input:String) -> String {
#     match input.as_str() {
#         _ => {
#             panic!("Unknown cell name {}", input);
#         }
#     }
# }""".splitlines()
    
    rule_path = 'src/dynamic_rules.rs'
    language_path = 'src/language.rs'
    modified_content = []
    end_line=5


    op=['+','*']
    prefix_ops = ["!"]
    parser = argparse.ArgumentParser(description='Convert infix expressions to prefix notation.')
    parser.add_argument('-f', type=str, help='The infix expression to be converted.')
    args = parser.parse_args()
    file_path = args.f

    # formatted_expression = further_process(format_prefix_operations("!A", prefix_ops),prefix_ops)
    # formatted_expression=remove_redundant_parentheses(formatted_expression)
    # prefix_expression,oprands = infix_to_prefix(formatted_expression,prefix_ops,op)
    # prefix_expression=prefix_expression.replace("!","not").replace("+","or").replace("*","and")
    # print(prefix_expression)
    # print(oprands)
    # file_path = "7nm.genlib"

    cells = parse_genlib(file_path)
    rules=[]
    language=[]
    language_enum=[]
    cell_name=[]
    node_type=[]
    oprands_list=[]
    # cell_name_match=[]
    for cell in cells:
       try:
          formatted_expression = further_process(format_prefix_operations(cell.function.split("=")[1], prefix_ops),prefix_ops)
          formatted_expression=remove_redundant_parentheses(formatted_expression)
          prefix_expression,oprands = infix_to_prefix(formatted_expression,prefix_ops,op)
          oprands_list.append(f"        \"{cell.name}\" => vec![\"{("\", \"").join([i[1:] for i in oprands]+[cell.function.split("=")[0]])}\"],")
          prefix_expression=prefix_expression.replace("!","not").replace("+","or").replace("*","and")
          buff_pattern = re.compile(r'^\?[A-Za-z_][A-Za-z0-9_]*$')
          if "BUF" in cell.name and bool(buff_pattern.match(prefix_expression)):
            prefix_expression = f"(buff {prefix_expression})"
          rules.append(f"        rewrite!(\"external_rule_{len(rules)}\";\"{prefix_expression}\" => \"({cell.name} {" ".join(oprands)})\"),")
          rules.append(f"        rewrite!(\"external_rule_{len(rules)}\";\"({cell.name} {" ".join(oprands)})\" => \"{prefix_expression}\"),")
          if cell.name not in cell_name:
            language.append(f"        \"{cell.name}\" = {convert_upper_camel(cell.name)}(Vec<Id>),")
            # "dff" => {Some(SimpleLanguage::DFF(in_id))},
            language_enum.append(f"        \"{cell.name}\" => {{Some(SimpleLanguage::{convert_upper_camel(cell.name)}(in_id))}},")
            cell_name.append(cell.name)
            node_type.append(f"        SimpleLanguage::{convert_upper_camel(cell.name)}(_) => \"{cell.name}\",")
            # cell_name_match.append(f"        \"{convert_upper_camel(cell.name)}\" => \"{cell.name}\".to_string(),")
       except:
          import pdb;pdb.set_trace()
    modified_content=rule_content[:end_line]+rules+rule_content[end_line:]
    modified_content=[i + "\n" for i in modified_content]
    language_content=language_content[:16]+language+language_content[16:]
    language_content=[i + "\n" for i in language_content]
    language_enum_content=language_enum_content[:10]+language_enum+language_enum_content[10:]
    language_enum_content=[i + "\n" for i in language_enum_content]
    node_type_content = node_type_content[:10] + node_type + node_type_content[10:]
    node_type_content=[i + "\n" for i in node_type_content]
    cell_input_string = cell_input_string[:2] + oprands_list + cell_input_string[2:]
    cell_input_string =[i + "\n" for i in cell_input_string]
    # cell_name_string=cell_name_string[:2] + cell_name_match + cell_name_string[2:]
    # cell_name_string=[i + "\n" for i in cell_name_string]

    with open(rule_path, 'w') as file:
        file.writelines(modified_content)

    with open(language_path, 'w') as file:
        file.writelines(language_content+language_enum_content+node_type_content+cell_input_string)


if __name__ == '__main__':
    main()
