use std::convert::TryFrom;

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
pub fn execute(state: &mut State, hue_change: u8, lightness_change: u8, current_region_size: usize) -> () {
    match (hue_change, lightness_change) {
        (0, 0) => (),
        (0, 1) => push(state, current_region_size),
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
    };
}

/// Pushes the value of the colour block just exited on to the stack.
/// Note that values of colour blocks are not automatically pushed on to the stack - this push operation must be explicitly carried out.
fn push(state: &mut State, current_region_size: usize) -> () {
    state.stack.push(current_region_size as isize);
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
    let stack_size = state.stack.len();
    let maybe_final_stack_size = stack_size.checked_sub(2);

    // Only roll if:
    //  - we actually have at least two elements to pop off the stack
    //  - the roll depth is positive
    //  - the roll depth is not greater than the stack size after popping off the top two elements
    if let Some(final_stack_size) = maybe_final_stack_size {
        // TODO: this is sloppy and full of redundant checks, it'll be easy to clean up after if let chains are added
        state
            .stack
            .get(final_stack_size)
            .and_then(|depth| usize::try_from(*depth).ok())
            .and_then(|depth| if depth <= final_stack_size { Some(depth) } else { None })
            .map(|depth|
                {
                    let turns = state.stack.pop().unwrap();
                    let _depth = state.stack.pop().unwrap();

                    if turns >= 0 {
                        for _ in 0..turns {
                            let top = state.stack.pop().unwrap();
                            state.stack.insert(final_stack_size - depth, top);
                        }
                    } else {
                        for _ in turns..0 {
                            let bottom = state.stack.remove(final_stack_size - depth);
                            state.stack.push(bottom);
                        }
                    }
                }
            );
    }
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
    state
        .stack
        .pop()
        .map(|top| print!("{}", top));
}

/// Pops the top value off the stack and prints it to STDOUT as either a number or character, depending on the particular incarnation of this command.
fn out_char(state: &mut State) -> () {
    state
        .stack
        .pop()
        .and_then(|top| u32::try_from(top).ok())
        .and_then(|top| std::char::from_u32(top))
        .map(|char| print!("{}", char));
}

#[cfg(test)]
mod test_command {
    use super::*;

    #[test]
    fn test_push() {
        let mut state = State::new();
        push(&mut state, 1);
        assert_eq!(state.stack, vec![1]);
    }

    #[test]
    fn test_pop() {
        let mut state = State::new();
        state.stack.push(1);
        pop(&mut state);
        assert_eq!(state.stack, vec![]);
    }

    #[test]
    fn test_add() {
        let mut state = State::new();
        state.stack.push(1);
        state.stack.push(2);
        add(&mut state);
        assert_eq!(state.stack, vec![3]);

        let mut short_state = State::new();
        short_state.stack.push(1);
        add(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    fn test_subtract() {
        let mut state = State::new();
        state.stack.push(1);
        state.stack.push(2);
        subtract(&mut state);
        assert_eq!(state.stack, vec![-1]);

        let mut short_state = State::new();
        short_state.stack.push(1);
        subtract(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    fn test_multiply() {
        let mut state = State::new();
        state.stack.push(2);
        state.stack.push(3);
        multiply(&mut state);
        assert_eq!(state.stack, vec![6]);

        let mut short_state = State::new();
        short_state.stack.push(1);
        multiply(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    fn test_divide() {
        let mut state = State::new();
        state.stack.push(7);
        state.stack.push(3);
        divide(&mut state);
        assert_eq!(state.stack, vec![2]);

        let mut ignore_zero_state = State::new();
        ignore_zero_state.stack.push(7);
        ignore_zero_state.stack.push(0);
        divide(&mut ignore_zero_state);
        assert_eq!(ignore_zero_state.stack, vec![7, 0]);

        let mut short_state = State::new();
        short_state.stack.push(1);
        divide(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    fn test_modulo() {
        let mut state = State::new();
        state.stack.push(7);
        state.stack.push(3);
        modulo(&mut state);
        assert_eq!(state.stack, vec![1]);

        let mut ignore_zero_state = State::new();
        ignore_zero_state.stack.push(7);
        ignore_zero_state.stack.push(0);
        modulo(&mut ignore_zero_state);
        assert_eq!(ignore_zero_state.stack, vec![7, 0]);

        let mut short_state = State::new();
        short_state.stack.push(1);
        modulo(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    fn test_not() {
        let mut true_state = State::new();
        true_state.stack.push(0);
        not(&mut true_state);
        assert_eq!(true_state.stack, vec![1]);

        let mut false_state = State::new();
        false_state.stack.push(33);
        not(&mut false_state);
        assert_eq!(false_state.stack, vec![0]);
    }

    #[test]
    fn test_greater() {
        let mut greater_state = State::new();
        greater_state.stack.push(2);
        greater_state.stack.push(1);
        greater(&mut greater_state);
        assert_eq!(greater_state.stack, vec![1]);

        let mut lesser_state = State::new();
        lesser_state.stack.push(1);
        lesser_state.stack.push(2);
        greater(&mut lesser_state);
        assert_eq!(lesser_state.stack, vec![0]);

        let mut short_state = State::new();
        short_state.stack.push(1);
        greater(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    fn test_pointer() {
        let mut state = State::new();
        let initial_direction = state.pointer_direction.clone();
        state.stack.push(2);
        pointer(&mut state);
        assert_eq!(state.pointer_direction, initial_direction.next().next());

        let mut wrapping_state = State::new();
        wrapping_state.stack.push(5);
        pointer(&mut wrapping_state);
        assert_eq!(wrapping_state.pointer_direction, initial_direction.next());

        let mut negative_state = State::new();
        negative_state.stack.push(-3);
        pointer(&mut negative_state);
        assert_eq!(negative_state.pointer_direction, initial_direction.next());
    }

    #[test]
    fn test_switch() {
        let mut state = State::new();
        let initial_direction = state.chooser_direction.clone();
        state.stack.push(1);
        switch(&mut state);
        assert_eq!(state.chooser_direction, initial_direction.next());

        let mut wrapping_state = State::new();
        wrapping_state.stack.push(4);
        switch(&mut wrapping_state);
        assert_eq!(wrapping_state.chooser_direction, initial_direction);

        let mut absolute_state = State::new();
        wrapping_state.stack.push(-3);
        switch(&mut wrapping_state);
        assert_eq!(wrapping_state.chooser_direction, initial_direction.next());
    }

    #[test]
    fn test_duplicate() {
        let mut state = State::new();
        state.stack.push(1);
        duplicate(&mut state);
        assert_eq!(state.stack, vec![1, 1]);
    }

    #[test]
    fn test_roll() {
        let simple_stack: Vec<isize> = vec![1, 2, 3, 4, 5, 6];

        let mut state = State::new();
        state.stack.append(&mut simple_stack.clone());
        state.stack.push(3); // depth
        state.stack.push(2); // turns
        roll(&mut state);
        assert_eq!(state.stack, vec![1, 2, 3, 5, 6, 4]);

        let mut negative_turns_state = State::new();
        negative_turns_state.stack.append(&mut simple_stack.clone());
        negative_turns_state.stack.push(3); // depth
        negative_turns_state.stack.push(-2); // turns
        let negative_turns_initial = negative_turns_state.stack.clone();
        roll(&mut negative_turns_state);
        assert_eq!(negative_turns_state.stack, vec![1, 2, 3, 6, 4, 5]);

        let mut negative_depth_state = State::new();
        negative_depth_state.stack.append(&mut simple_stack.clone());
        negative_depth_state.stack.push(-1); // depth
        negative_depth_state.stack.push(2); // turns
        let negative_depth_initial = negative_depth_state.stack.clone();
        roll(&mut negative_depth_state);
        assert_eq!(negative_depth_state.stack, negative_depth_initial);

        let mut short_state = State::new();
        short_state.stack.push(1);
        roll(&mut short_state);
        assert_eq!(short_state.stack, vec![1]);
    }

    #[test]
    #[ignore]
    fn test_in_number() { todo!(); }

    #[test]
    #[ignore]
    fn test_in_char() { todo!(); }

    #[test]
    #[ignore]
    fn test_out_number() { todo!(); }

    #[test]
    #[ignore]
    fn test_out_char() { todo!(); }
}
