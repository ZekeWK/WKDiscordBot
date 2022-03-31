use crate::base::error::CommandError;

const LIERS_DICE_CREATE : &str = "create";
const LIERS_DICE_CALL : &str = "call";



fn liers_dice_parse(mut args : impl std::iter::Iterator<Item = String>) -> CommandType {
    let action = match args.next() {
        Some(val) => val,
        None => return Error(CommandError::MissingArgs),
    };

    return match action {



        _ => Error(CommandError::UnvalidArgs)
    };
}