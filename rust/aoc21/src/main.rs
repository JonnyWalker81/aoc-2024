use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Display};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let codes = input.trim().split('\n').collect::<Vec<&str>>();

    // println!("{:?}", codes);

    let num_keypad = [
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];

    let dir_keypad = [vec![' ', '^', 'A'], vec!['<', 'v', '>']];

    let mut count = 0;
    // for code in codes.iter().take(1) {
    for code in codes.iter() {
        // let mut start = 'A';
        let mut start = (3, 2);
        let mut full_path = vec![];
        for c in code.chars() {
            let (path, pos) = bfs(&num_keypad, start, c);
            start = pos;
            // println!("path: {:?}", path);
            full_path.extend(path);
        }
        let num_path = full_path.iter().collect::<String>();
        println!("full path: {:?}", num_path);

        start = (0, 2);
        full_path.clear();
        for c in num_path.chars() {
            let (path, pos) = bfs(&dir_keypad, start, c);
            start = pos;
            // println!("path: {:?}", path);
            full_path.extend(path);
        }

        let dir_path_1 = full_path.iter().collect::<String>();
        println!("dir_path_1: {:?} -- {}", dir_path_1, dir_path_1.len());

        start = (0, 2);
        full_path.clear();
        for c in dir_path_1.chars() {
            let (path, pos) = bfs(&dir_keypad, start, c);
            start = pos;
            // println!("path: {:?}", path);
            full_path.extend(path);
        }

        let dir_path_2 = full_path.iter().collect::<String>();
        println!("dir_path_2: {:?} -- {}", dir_path_2, dir_path_2.len());
        let digits = &code[..code.len() - 1];
        println!("digits: {:?}", digits);
        let val: usize = code[..code.len() - 1].parse().unwrap();
        println!("total: {} * {}", dir_path_2.len(), val);
        let total = dir_path_2.len() * val;
        println!("total: {}", total);
        count += total;
    }

    println!("count: {}", count);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn dir(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

// INPUT:
//   - keypadGraph: A mapping of valid "button -> list of neighbors"
//                  where each neighbor is a direction (Up/Down/Left/Right),
//                  along with a "press" action at the current node.
//   - startButton: The button where the arm initially is (e.g., "A").
//   - targetButton: The button we ultimately want to press.

// OUTPUT:
//   - A shortest sequence of moves/presses (string or list of commands)
//     that starts at 'startButton' and ends with 'targetButton' pressed.

// PROCEDURE BFS(keypadGraph, startButton, targetButton):
//     # Each state in the BFS will represent (currentButton, pressedYet?)
//     #    or sometimes just currentButton if you BFS in segments:
//     #    "move to target" then "press" as a separate step.
//     # Below we do it in a single BFS with a boolean 'pressed' dimension.

//     CREATE a queue Q
//     CREATE a dictionary visited to store visited states and reconstruct path
//         visited[(currentButton, pressed)] = (previousState, actionTaken)
//         # 'previousState' is the parent state
//         # 'actionTaken' is the move or press used to get here

//     INITIAL_STATE = (startButton, false)  # haven't pressed yet
//     visited[INITIAL_STATE] = (None, None)  # no parent, no action
//     ENQUEUE Q with INITIAL_STATE

//     while Q is not empty:
//         currentState = DEQUEUE(Q)
//         currentButton, havePressed = currentState

//         if (currentButton == targetButton) AND (havePressed == true):
//             # We have pressed the target button. Reconstruct path and return it.
//             return RECONSTRUCT_PATH(visited, currentState)

//         # 1. Try moving Up/Down/Left/Right (if valid)
//         for (nextButton, moveCommand) in keypadGraph[currentButton].neighbors:
//             # nextButton is where we'd land, moveCommand is a character like '^', 'v', '<', '>', etc.
//             nextState = (nextButton, havePressed)
//             if nextState not in visited:
//                 visited[nextState] = (currentState, moveCommand)
//                 ENQUEUE(Q, nextState)

//         # 2. Try pressing the current button, if we haven't already or if pressing is relevant
//         # In many puzzles, you only "need" to press if currentButton == targetButton,
//         # but if you allow pressing any button at any time, adjust logic as needed.
//         if (currentButton == targetButton) AND (havePressed == false):
//             # Press action
//             nextState = (currentButton, true)  # now we've pressed the button
//             if nextState not in visited:
//                 visited[nextState] = (currentState, 'A')  # 'A' = press action
//                 ENQUEUE(Q, nextState)

//     # If we exhaust the queue without returning, there's no valid path
//     return "NO VALID SEQUENCE FOUND"

// PROCEDURE RECONSTRUCT_PATH(visited, finalState):
//     # Reconstructs the path (list of commands) by backtracking from finalState
//     commands = []
//     current = finalState

//     while visited[current] is not (None, None):
//         (parent, action) = visited[current]
//         # action is the command used to go from 'parent' to 'current'
//         if action != None:
//             commands.append(action)
//         current = parent

//     # The commands are reversed (we built them backwards), so reverse them
//     commands.reverse()

//     # Join them into a string or return as a list
//     return commands

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    pressed: bool,
}

fn bfs(keypad: &[Vec<char>], start: Pos, target: char) -> (Vec<char>, Pos) {
    let mut visited: HashMap<State, (Option<State>, Option<char>)> = HashMap::new();
    let state = State {
        pos: start,
        pressed: false,
    };

    visited.insert(state, (None, None));

    let mut queue = VecDeque::from([(state, 0)]);

    while let Some((state, cost)) = min_cost(&mut queue) {
        let current = state.pos;
        let pressed = state.pressed;

        if keypad[current.0][current.1] == target && pressed {
            // println!("found target - {}: {:?}", target, current);
            return (reconstruct_path(&visited, state), current);
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        {
            let (dr, dc) = dir.dir();
            let nr = current.0 as i32 + dr;
            let nc = current.1 as i32 + dc;

            if nr < 0 || nr >= keypad.len() as i32 || nc < 0 || nc >= keypad[0].len() as i32
            // || keypad[nr as usize][nc as usize] == ' '
            {
                continue;
            }

            let np = (nr as usize, nc as usize);
            let next_state = State { pos: np, pressed };

            if !visited.contains_key(&next_state) {
                visited.insert(next_state, (Some(state), Some(dir.to_char())));
                let next_cost = cost + 1;
                queue.push_back((next_state, next_cost));
            }
        }

        if keypad[current.0][current.1] == target && !pressed {
            let next_state = State {
                pos: current,
                pressed: true,
            };

            if !visited.contains_key(&next_state) {
                visited.insert(next_state, (Some(state), Some('A')));
                let next_cost = cost + 1;
                queue.push_back((next_state, next_cost));
            }
        }
    }

    (vec![], (0, 0))
}

fn reconstruct_path(
    visited: &HashMap<State, (Option<State>, Option<char>)>,
    final_state: State,
) -> Vec<char> {
    let mut commands = vec![];
    let mut current = final_state;

    // println!("final state: {:?}", final_state);
    while let Some((parent, action)) = visited.get(&current) {
        if action.is_none() {
            // println!("parent: {:?}, action: {:?}", parent, action);
            break;
        }
        // println!("parent: {:?}, action: {:?}", parent, action);
        commands.push(action.unwrap());
        current = parent.unwrap();
    }

    commands.reverse();
    commands
}

// fn bfs(code: &str, keypad: &[Vec<char>], start: (usize, usize)) -> Vec<char> {
//     let mut code_path = vec![];
//     let mut pos = start;

//     for c in code.chars() {
//         let mut queue = VecDeque::from(vec![(pos, vec![], 0)]);
//         let mut visited = HashSet::new();

//         while let Some((p, mut path, cost)) = min_cost(&mut queue) {
//             if keypad[p.0][p.1] == c {
//                 println!("found code - {}: {}", c, cost);
//                 println!("path: {:?}", path);
//                 pos = p;
//                 let mut char_vec = path.iter().map(|d| d.to_char()).collect::<Vec<char>>();
//                 char_vec.push('A');
//                 code_path.extend(char_vec);
//                 // break;
//             }

//             if visited.contains(&p) {
//                 continue;
//             }

//             visited.insert(p);

//             for dir in [
//                 Direction::Up,
//                 Direction::Down,
//                 Direction::Left,
//                 Direction::Right,
//             ]
//             .iter()
//             {
//                 let (dr, dc) = dir.dir();
//                 let nr = p.0 as i32 + dr;
//                 let nc = p.1 as i32 + dc;

//                 if nr < 0 || nr >= keypad.len() as i32 || nc < 0 || nc >= keypad[0].len() as i32 {
//                     continue;
//                 }

//                 let np = (nr as usize, nc as usize);
//                 let next_cost = cost + 1;

//                 let mut next_path = path.clone();
//                 next_path.push(*dir);

//                 if keypad[np.0][np.1] != ' ' {
//                     queue.push_back((np, next_path, next_cost));
//                 }
//             }
//         }
//     }

//     code_path
// }

fn min_cost(queue: &mut VecDeque<(State, usize)>) -> Option<(State, usize)> {
    let mut min_cost = usize::MAX;
    let mut min_idx = 0;

    for (i, (_, cost)) in queue.iter().enumerate() {
        if *cost < min_cost {
            min_cost = *cost;
            min_idx = i;
        }
    }

    queue.remove(min_idx)
}
