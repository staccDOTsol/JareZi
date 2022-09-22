pub mod utils;

use {
    anchor_lang::{
        prelude::*,
        AnchorDeserialize, AnchorSerialize,
    }
};
anchor_lang::declare_id!("EMXJxUgMWD9uKiS2mtYCRBmKbGN9sHsDwMUnhNvWJR1X");
pub const PREFIX: &str = "jarezi";
#[program]
pub mod jarezi {

    use super::*;



    pub fn join_jarezi<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, JoinJareZi<'info>>,
    ) -> Result<u8> {
        let jarezi_instance = &mut ctx.accounts.jarezi_instance;
        let jares = &mut ctx.accounts.jares;
        let payer = &ctx.accounts.payer;

as None
        jarezi_instance.token_types_added = jarezi_instance
            .token_types_added
            .checked_add(1)
            .ok_or(ErrorCode::NumericalOverflowError)?;

        let now_ts = Clock::get().unwrap().unix_timestamp;

        if jarezi_instance.lastthousand == 0 {
            jarezi_instance.lastthousand = (now_ts + 1000) as i64;
        }
    if jares.lastplay  < now_ts - 10 || jares.lastplay > now_ts - 2  {// 5 > 20 -10 = 10 no
        jares.disqualified = true; 
        msg!("disq4");
    
    }
        if jares.lastplay == 0  {
            jares.disqualified = false;
            jares.nice = 0;
            msg!("disq2");
        }
            jares.nice = jares.nice.checked_add(1).ok_or(ErrorCode::NumericalOverflowError)?;
            msg!("canplay");

        if (now_ts) as i64 > jarezi_instance.lastthousand - 10 && !jares.disqualified && !jarezi_instance.bonus && jares.nice > 9 {
            jarezi_instance.bonus = true;
            return Ok(1 as u8);
        }
        if (now_ts) as i64 > jarezi_instance.lastthousand && !jares.disqualified && jares.nice > 1 {
            msg!("winnawinnachickems");

            jares.token_types_removed = jarezi_instance.token_types_removed;
            jarezi_instance.lastthousand = (now_ts + 1000) as i64;
            jares.nice = 0;
            if now_ts > jarezi_instance.lastplay  {
                msg!("becomewinna");
                jarezi_instance.lastplay = now_ts; 
                jarezi_instance.winning = payer.key();
                jares.lastplay = now_ts; 
                
                
            }
            if (jares.token_types_removed < jarezi_instance.token_types_removed) && jares.disqualified {
                jares.disqualified = false;
                msg!("disq1");
                jares.nice = 0;
                jares.token_types_removed = jarezi_instance.token_types_removed;
            }
            if (now_ts) as i64 > jarezi_instance.lastthousand - 10 && !jares.disqualified && !jarezi_instance.bonus && jares.nice > 9 {
                jarezi_instance.bonus = true;
    
                return   Ok(2 as u8);
            }
        if (now_ts) as i64 > jarezi_instance.lastthousand  && !jares.disqualified && jares.nice > 1 { 
            msg!("winnawinnachickems");

            jares.token_types_removed = jarezi_instance.token_types_removed;
            jarezi_instance.lastthousand = (now_ts + 1000) as i64;
            jares.nice = 0;
    
    
            return   Ok(3 as u8);
        }
        
    }
        if now_ts > jarezi_instance.lastplay  {
            msg!("becomewinna");
            jarezi_instance.lastplay = now_ts; 
            jarezi_instance.winning = payer.key();
            jares.lastplay = now_ts; 
            
            
        }

        if (jares.token_types_removed < jarezi_instance.token_types_removed) && jares.disqualified {
            jares.disqualified = false;
            msg!("disq1");
            jares.nice = 0;
            jares.token_types_removed = jarezi_instance.token_types_removed;
        }
        Ok(4 as u8)
    }
}

#[derive(Accounts)]
pub struct JoinJareZi<'info> {
    #[account(init_if_needed, seeds=[PREFIX.as_bytes(), payer.key().as_ref(), jarezi_instance.key().as_ref()], bump, payer=payer, space=56 as usize)]
    jares: Account<'info, Jares>,

    #[account(mut, constraint = jarezi_instance.to_account_info().owner == &Pubkey::new_from_array([
        11, 127, 234, 228,  24,   2, 223, 228,
        19, 247,  33,  58, 249,   3,  67, 114,
        21, 117, 159, 178, 240, 219,  24, 108,
        175, 122, 132, 132, 188, 236,  41,  73
    ]))]
    jarezi_instance: Box<Account<'info, JareZi>>,

    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[account]
pub struct JareZi {
    token_types_added: u8,
    token_types_removed: u8,
    jarezi_instance: Pubkey,
    lastthousand: i64,
    jares2: Pubkey,
    winning: Pubkey,
    lastplay: i64,
    bonus: bool
    
}
#[account]

 struct Jares {
    lastplay: i64,
    disqualified: bool,
    token_types_removed: u8,
    nice: u8
}
#[error_code]
pub enum ErrorCode {
    #[msg("hm")]
    GenericError,
    #[msg("numbers")]
    NumericalOverflowError,
    
}