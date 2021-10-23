use serenity::prelude::*;
use serenity::{
    framework::standard::{help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "Si quieres más información sobre un comando específico solo escribe pasa el comando como argumento"]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
#[lacking_permissions = "Hide"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
