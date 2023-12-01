use anchor_lang::prelude::*;

declare_id!("5reFTz45EsYQFjXfbmrQ9kU94ceoqkFDim36hR1rkfZK");

#[program]
pub mod todo_list_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
#[account(init, payer = author, space = Task::LEN)]
pub task: Account<'info, Task>,
#[account(mut)]
pub author: Signer<'info>,
pub system_program: Program<'info, System>,
}

#[account]
pub struct Task {
    pub author: Pubkey,
    pub is_done: bool,
    pub text: String,
    pub created_at: i64,
    pub updated_at: i64,
}

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 chars
const TIMESTAMP_LENGTH: usize = 8;


impl Task {
const LEN: usize = DISCRIMINATOR + // discriminator
PUBLIC_KEY_LENGTH + // author
BOOL_LENGTH + // is_done
TEXT_LENGTH + // text
TIMESTAMP_LENGTH + // created_at
TIMESTAMP_LENGTH; // updated_at
}

pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
let task = &mut ctx.accounts.task;
let author = &ctx.accounts.author; // The `author` account
let clock = Clock::get().unwrap(); // Getting the current timestamp
if text.chars().count() > 400 {
return Err(ErrorCode::TextTooLong.into());
}
task.author = *author.key;
task.is_done = false;
task.created_at = clock.unix_timestamp;
task.updated_at = clock.unix_timestamp;
task.text = text;
Ok(())


}

pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
let task = &mut ctx.accounts.task;
let author = &ctx.accounts.author; // The `author` account
let clock = Clock::get().unwrap(); // Getting the current timestamp
task.author = *author.key;
task.is_done = true;
task.updated_at = clock.unix_timestamp;
Ok(())
}
#[derive(Accounts)]
pub struct DeletingTask<'info> {
#[account(mut, has_one = author)]
pub task: Account<'info, Task>,
pub author: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
#[msg("The text is too long")]
TextTooLong,
}