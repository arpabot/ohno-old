use crate::{Context, Error};

/// コマンドのヘルプを表示します
#[poise::command(track_edits, slash_command)]
pub async fn help(
  ctx: Context<'_>,
  #[description = "説明を見たいコマンド"]
  #[autocomplete = "poise::builtins::autocomplete_command"]
  command: Option<String>,
) -> Result<(), Error> {
  poise::builtins::help(
    ctx,
    command.as_deref(),
    poise::builtins::HelpConfiguration {
      extra_text_at_bottom: r#"このBotのソースコードはGitHub上で公開されています。
  https://github.com/yuimarudev/ohno"#,
      show_context_menu_commands: true,
      ..Default::default()
    },
  )
  .await?;
  Ok(())
}

#[poise::command(hide_in_help, prefix_command)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
  if ctx.guild().is_none() {
    return Ok(());
  }
  poise::builtins::register_application_commands(ctx, global).await?;

  Ok(())
}
