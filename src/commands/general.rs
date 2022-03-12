use crate::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"]
  #[autocomplete = "poise::builtins::autocomplete_command"]
  command: Option<String>,
) -> Result<(), Error> {
  poise::builtins::help(
    ctx,
    command.as_deref(),
    poise::builtins::HelpConfiguration {
      extra_text_at_bottom: "\
This is an example bot made to showcase features of my custom Discord bot framework",
      show_context_menu_commands: true,
      ..Default::default()
    },
  )
  .await?;
  Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
  if ctx.guild().is_none() {
    return Ok(());
  }
  poise::builtins::register_application_commands(ctx, global).await?;

  Ok(())
}
