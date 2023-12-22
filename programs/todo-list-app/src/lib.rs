use anchor_lang::prelude::*;
declare_id!("9LZdb5Cxg2TowwxKYU1h4mzKQDfEC52AaUAjoqt5emDM");

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; //400 char
const TIMESTAMP_LENGTH: usize = 8;

#[error_code]
pub enum ErrorCode {
    #[msg("Text too long")]
    TextTooLong,
}

impl From<ErrorCode> for std::io::Error {
    fn from(e: ErrorCode) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}


#[program]
pub mod todo_list_app {
    use std::io::Result;

    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

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

    pub fn updating_task(ctx:  Context<UpdatingTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        task.author = *author.key;
        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap(); 
        task.author = *author.key;
        task.is_done = true;
        task.updated_at = clock.unix_timestamp;
        Ok(())
       
    }
}

#[account]
pub struct Task {
    pub author: Pubkey,  // Account which owns the task,
    pub is_done: bool,   // whether the task is done or not,
    pub text: String,    // Text of the task,
    pub created_at: i64, // Timestamp for task creation
    pub updated_at: i64, // Timestamp for task updates,
}

impl Task {
    const LEN: usize = DISCRIMINATOR + // DISCRIMINATOR
    PUBLIC_KEY_LENGTH + // author
    BOOL_LENGTH + // is_done,
    TEXT_LENGTH + // text,
    TIMESTAMP_LENGTH + //created at,
    TIMESTAMP_LENGTH; // updated_at
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}
