use crate::state::State;

///	Selects a command based on the change in hue/lightness between two regions and executes it.
///
/// | hue/lightness change | 0    | 1        | 2      | 3       | 4         | 5          |
/// |----------------------|------|----------|--------|---------|-----------|------------|
/// | 0                    |      | add      | divide | greater | duplicate | in_char    |
/// | 1                    | push | subtract | modulo | pointer | roll      | out_number |
/// | 2                    | pop  | multiply | not    | switch  | in_number | out_char   |
///
/// Any operations which cannot be performed (such as popping values when not enough are on the stack) are simply ignored, and processing continues with the next command.
pub fn execute(state: &mut State, hue_change: u8, lightness_change: u8, last_region_size: usize) -> () {
    match (hue_change, lightness_change) {
        (0, 0) => (),
        (0, 1) => push(state, last_region_size),
        (0, 2) => pop(state),
        (1, 0) => add(state),
        (1, 1) => subtract(state),
        (1, 2) => multiply(state),
        (2, 0) => divide(state),
        (2, 1) => modulo(state),
        (2, 2) => not(state),
        (3, 0) => greater(state),
        (3, 1) => pointer(state),
        (3, 2) => switch(state),
        (4, 0) => duplicate(state),
        (4, 1) => roll(state),
        (4, 2) => in_number(state),
        (5, 0) => in_char(state),
        (5, 1) => out_number(state),
        (5, 2) => out_char(state),
        (hue, lightness) => panic!("Unexpected hue ({}) / lightness ({}) change", hue, lightness),
    }
    push(state, last_region_size);
}

/// Pushes the value of the colour block just exited on to the stack.
/// Note that values of colour blocks are not automatically pushed on to the stack - this push operation must be explicitly carried out.
fn push(state: &mut State, last_region_size: usize) -> () {
    state.stack.push(last_region_size as isize);
}

/// Pops the top value off the stack and discards it.
fn pop(state: &mut State) -> () {
    state.stack.pop();
}

/// Pops the top two values off the stack, adds them, and pushes the result back on the stack.
fn add(state: &mut State) -> () {
    if state.stack.len() >= 2 {
        let one = state.stack.pop().unwrap();
        let two = state.stack.pop().unwrap();
        state.stack.push(one + two);
    }
}

/// Pops the top two values off the stack, calculates the second top value minus the top value, and pushes the result back on the stack.
fn subtract(state: &mut State) -> () {
    if state.stack.len() >= 2 {
        let one = state.stack.pop().unwrap();
        let two = state.stack.pop().unwrap();
        state.stack.push(two - one);
    }
}

/// Pops the top two values off the stack, multiplies them, and pushes the result back on the stack.
fn multiply(state: &mut State) -> () {
    if state.stack.len() >= 2 {
        let one = state.stack.pop().unwrap();
        let two = state.stack.pop().unwrap();
        state.stack.push(two * one);
    }
}

/// Pops the top two values off the stack, calculates the integer division of the second top value by the top value, and pushes the result back on the stack.
/// If a divide by zero occurs, it is handled as an implementation-dependent error, though simply ignoring the command is recommended.
fn divide(state: &mut State) -> () {
    if state.stack.len() >= 2 && state.stack.last().unwrap() != &0 {
        let one = state.stack.pop().unwrap();
        let two = state.stack.pop().unwrap();
        state.stack.push(two / one);
    }
}

/// Pops the top two values off the stack, calculates the second top value modulo the top value, and pushes the result back on the stack.
/// The result has the same sign as the divisor (the top value).
/// If the top value is zero, this is a divide by zero error, which is handled as an implementation-dependent error, though simply ignoring the command is recommended.
fn modulo(state: &mut State) -> () {
    if state.stack.len() >= 2 && state.stack.last().unwrap() != &0 {
        let one = state.stack.pop().unwrap();
        let two = state.stack.pop().unwrap();
        state.stack.push(two % one);
    }
}

/// Replaces the top value of the stack with 0 if it is non-zero, and 1 if it is zero.
fn not(state: &mut State) -> () {
    if let Some(top) = state.stack.pop() {
        state.stack.push(if top == 0 { 1 } else { 0 });
    }
}

/// Pops the top two values off the stack, and pushes 1 on to the stack if the second top value is greater than the top value, and pushes 0 if it is not greater.
fn greater(state: &mut State) -> () {
    if state.stack.len() >= 2 {
        let one = state.stack.pop().unwrap();
        let two = state.stack.pop().unwrap();
        state.stack.push(if two > one { 1 } else { 0 });
    }
}

/// Pops the top value off the stack and rotates the DP clockwise that many steps (anticlockwise if negative).
fn pointer(state: &mut State) -> () {
    if let Some(top) = state.stack.pop() {
        let clockwise_steps = {
            let absolute_steps = top % 4;
            if absolute_steps >= 0 {
                absolute_steps
            } else {
                4 + absolute_steps
            }
        };
        for _ in 0..clockwise_steps {
            state.pointer_direction = state.pointer_direction.next();
        }
    }
}

/// Pops the top value off the stack and toggles the CC that many times (the absolute value of that many times if negative).
fn switch(state: &mut State) -> () {
    if let Some(top) = state.stack.pop() {
        let steps = (top % 2).abs();
        for _ in 0..steps {
            state.chooser_direction = state.chooser_direction.next();
        }
    }
}

/// Pushes a copy of the top value on the stack on to the stack.
fn duplicate(state: &mut State) -> () {
    if let Some(top) = state.stack.pop() {
        state.stack.push(top);
        state.stack.push(top);
    }
}

/// Pops the top two values off the stack and "rolls" the remaining stack entries to a depth equal to the second value popped, by a number of rolls equal to the first value popped.
/// A single roll to depth n is defined as burying the top value on the stack n deep and bringing all values above it up by 1 place.
/// A negative number of rolls rolls in the opposite direction.
/// A negative depth is an error and the command is ignored.
/// If a roll is greater than an implementation-dependent maximum stack depth, it is handled as an implementation-dependent error, though simply ignoring the command is recommended.
fn roll(state: &mut State) -> () {
    todo!()
}

/// Reads a value from STDIN as either a number or character, depending on the particular incarnation of this command and pushes it on to the stack.
/// If no input is waiting on STDIN, this is an error and the command is ignored.
/// If an integer read does not receive an integer value, this is an error and the command is ignored.
fn in_number(state: &mut State) -> () {
    todo!()
}

/// Reads a value from STDIN as either a number or character, depending on the particular incarnation of this command and pushes it on to the stack.
/// If no input is waiting on STDIN, this is an error and the command is ignored.
/// If an integer read does not receive an integer value, this is an error and the command is ignored.
fn in_char(state: &mut State) -> () {
    todo!()
}

/// Pops the top value off the stack and prints it to STDOUT as either a number or character, depending on the particular incarnation of this command.
fn out_number(state: &mut State) -> () {
    todo!()
}

/// Pops the top value off the stack and prints it to STDOUT as either a number or character, depending on the particular incarnation of this command.
fn out_char(state: &mut State) -> () {
    todo!()
}
