use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    rc::Rc,
};

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::alpha1,
    multi::separated_list1,
    Finish, IResult,
};
use tailcall::tailcall;

#[derive(Debug)]
struct ParsedNetwork<'a> {
    instructions: &'a [u8],
    nodes: Vec<ParsedNode<'a>>,
}

#[derive(Debug)]
struct ParsedNode<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

// Don't derive Debug on this, it is recursive!
struct Node<'a> {
    name: &'a str,
    left: Option<Rc<RefCell<Node<'a>>>>,
    right: Option<Rc<RefCell<Node<'a>>>>,
}

fn steps_to_dest(node: Rc<RefCell<Node>>, dest: &str, instructions: &[u8]) -> Result<u64, String> {
    let mut seen: HashSet<(String, usize)> = HashSet::new();
    // tailcall is necessary to avoid stack overflow
    #[tailcall]
    fn steps_to_dest_inner(
        accum: u64,
        seen: &mut HashSet<(String, usize)>,
        node: Rc<RefCell<Node>>,
        dest: &str,
        instructions: &[u8],
        instructions_idx: usize,
    ) -> Result<u64, String> {
        // Loop detector. Very important if you start at the wrong node
        let node = node.borrow();
        if seen
            .get(&(node.name.to_string(), instructions_idx))
            .is_some()
        {
            return Err(format!(
                "Loop detected at {} with instruction {} at idx {} accum {}",
                node.name, instructions[instructions_idx] as char, instructions_idx, accum,
            ));
        }
        seen.insert((node.name.to_string(), instructions_idx as usize));

        if node.name == dest {
            Ok(accum)
        } else {
            match instructions[instructions_idx] {
                b'L' => steps_to_dest_inner(
                    accum + 1,
                    seen,
                    node.left.clone().unwrap(),
                    dest,
                    instructions,
                    (instructions_idx + 1) % instructions.len(),
                ),
                b'R' => steps_to_dest_inner(
                    accum + 1,
                    seen,
                    node.right.clone().unwrap(),
                    dest,
                    instructions,
                    (instructions_idx + 1) % instructions.len(),
                ),
                _ => Err(format!(
                    "Invalid instruction {} for node {}",
                    instructions[instructions_idx] as char, node.name
                )),
            }
        }
    }
    steps_to_dest_inner(0, &mut seen, node, dest, instructions, 0)
}

fn build_network<'a>(parsed_network: ParsedNetwork<'a>) -> Rc<RefCell<Node<'a>>> {
    let mut map: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    for node in parsed_network.nodes.iter() {
        map.insert(
            node.name,
            Rc::new(RefCell::new(Node {
                name: node.name,
                left: None,
                right: None,
            })),
        );
    }
    for parsed_node in parsed_network.nodes.iter() {
        let mut node = map.get(parsed_node.name).unwrap().borrow_mut();
        node.left = map.get(parsed_node.left).map(|n| Rc::clone(n));
        node.right = map.get(parsed_node.right).map(|n| Rc::clone(n));
    }
    Rc::clone(map.get("AAA").unwrap())
}

fn parse_instructions(input: &str) -> IResult<&str, &[u8]> {
    let (remains, instructions) = is_a("LR")(input)?;
    Ok((remains, instructions.as_bytes()))
}

fn parse_node(input: &str) -> IResult<&str, ParsedNode> {
    let (remain, name) = alpha1(input)?;
    let (remain, _) = tag(" = (")(remain)?;
    let (remain, left) = alpha1(remain)?;
    let (remain, _) = tag(", ")(remain)?;
    let (remain, right) = alpha1(remain)?;
    let (remain, _) = tag(")")(remain)?;
    Ok((
        remain,
        ParsedNode {
            name: name,
            left: left,
            right: right,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, ParsedNetwork> {
    let (remain, instructions) = parse_instructions(input)?;
    let (remain, _) = tag("\n\n")(remain)?;
    let (remain, nodes) = separated_list1(tag("\n"), parse_node)(remain)?;
    Ok((
        remain,
        ParsedNetwork {
            instructions,
            nodes,
        },
    ))
}

pub fn part1(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).unwrap();
    let (remain, parsed_network) = parse(&input)
        .finish()
        .map_err(|e| format!("Parse Error: {}", e))?;
    if remain != "" {
        return Err("Parse Error".to_string());
    }
    let instructions = parsed_network.instructions;
    let network = build_network(parsed_network);

    let steps = steps_to_dest(network, "ZZZ", instructions)?;
    Ok(steps)
}

pub fn part2(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).unwrap();
    println!("{:?}", parse(&input).finish());
    Ok(0u64)
}
