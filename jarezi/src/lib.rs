pub mod utils;
use anchor_lang::solana_program::entrypoint::ProgramResult;

use {
    anchor_lang::{
        solana_program::        program::{invoke, invoke_signed},
solana_program::system_instruction,
        prelude::*,
        AnchorDeserialize, AnchorSerialize
    }
};
anchor_lang::declare_id!("4p3SG3CTzDdKBhm4mUBxMqX52fZZiPUxqvHBpZxbsww7");
pub const PREFIX: &str = "jarezi";
#[program]
pub mod jarezi {

    use super::*;



    pub fn update(
        ctx: Context<Update>,
        bulls: Vec<u8>,
        bears:Vec<u8>,
            kingbulls: Vec<u8>,
        kingbears: Vec<u8>,
        epochs: Vec<u8>,
    ) -> ProgramResult {
        
        let predictions =  &mut ctx.accounts.predictions;
        if predictions.epoch[predictions.epoch.len()-1] == epochs[epochs.len()-1] {
            if predictions.kingbear[kingbears.len()-1]  < bears[bears.len()-1] {
                predictions.kingbear[kingbears.len()-1] = bears[bears.len()-1];
            }
            if predictions.kingbull[kingbulls.len()-1]  < bulls[bulls.len()-1] {
                predictions.kingbull[kingbulls.len()-1] = bulls[bulls.len()-1];
            }
            predictions.bear[bears.len()-1] = bears[bears.len()-1]; 
            predictions.bull[bulls.len()-1] = bulls[bulls.len()-1];
        }
        else {

            predictions.epoch = epochs.into_boxed_slice();
            predictions.bull = bulls.into_boxed_slice();
            predictions.bear = bears.into_boxed_slice();

            predictions.kingbull = kingbulls.into_boxed_slice();
            predictions.kingbear = kingbears.into_boxed_slice();
            
            let predictions_account = &mut predictions.to_account_info();

            let system_program = &ctx.accounts.system_program;
            let payer_account = &ctx.accounts.auth.to_account_info();
            let new_size = predictions_account.data.borrow().len() + 56;
                
    
            let rent = Rent::get()?;
            let new_minimum_balance = rent.minimum_balance(new_size);

            let lamports_diff = new_minimum_balance.saturating_sub(predictions_account.lamports());
            invoke(
                &system_instruction::transfer(payer_account.key, predictions_account.key, lamports_diff),
                &[
                    payer_account.clone(),
                    predictions_account.clone(),
                    system_program.to_account_info().clone(),
                ],
            )?;
            predictions_account.realloc(new_size, false)?;


        }
        Ok(())
    }

}

#[account]
#[derive(Default, Debug)]
pub struct Predictions {
    bull: Box<[u8]>,
    bear:Box<[u8]>,
        kingbull: Box<[u8]>,
    kingbear: Box<[u8]>,
    epoch: Box<[u8]>,
    auth: Pubkey
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeFanoutArgs2 {
    bulls: Box<[u8]>,
    bears:Box<[u8]>,
        kingbulls: Box<[u8]>,
    kingbears: Box<[u8]>,
    epochs: Box<[u8]>,
}

#[derive(Accounts)]
#[instruction(bump: u8,   bulls: Box<[u8]>,
    bears:Box<[u8]>,
        kingbulls: Box<[u8]>,
    kingbears: Box<[u8]>,
    epochs: Box<[u8]>,)]
pub struct Update<'info> {
    #[account(init_if_needed, seeds=[b"pancake", auth.key().as_ref(), &[epochs[epochs.len()-1]]], bump, payer=auth, space=56 as usize)]
    
    pub predictions: Account<'info, Predictions>,
    #[account(mut)]
    pub auth: Signer <'info>,

    pub system_program: Program<'info, System>,
}
