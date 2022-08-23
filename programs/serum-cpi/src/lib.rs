
use {
    anchor_lang::{
        solana_program::system_program,
        prelude::*,
        AnchorDeserialize, AnchorSerialize,
    }
};


use std::num::NonZeroU64;
use serum_dex::matching::*;

use serum_dex::state::*;
use serum_dex::instruction::*;
anchor_lang::declare_id!("EMXJxUgMWD9uKiS2mtYCRBmKbGN9sHsDwMUnhNvWJR1X");

pub const PREFIX: &str = "jarezi";
pub mod serum_cpi {


    use super::*;


    const BASE: u128 = 1_000_000;



   
  pub  fn set_market(ctx: Context<SetMarket>
    )  -> Result<()> {
        let stuff = &mut ctx.accounts.stuff;
        stuff.proxy = ctx.accounts.proxy.key();
        stuff.market = ctx.accounts.market.key();
        stuff.open_orders = ctx.accounts.open_orders.key();
        stuff.request_queue = ctx.accounts.request_queue.key();
        stuff.event_queue = ctx.accounts.event_queue.key();
        stuff.market_bids = ctx.accounts.bids.key();
        stuff.market_asks = ctx.accounts.asks.key();
        stuff.order_payer_token_account = ctx.accounts.order_payer_token_account.key();
        stuff.open_orders_authority = ctx.accounts.prediction_market_account.key();
        stuff.coin_vault = ctx.accounts.coin_vault.key();
        stuff.pc_vault = ctx.accounts.pc_vault.key();
        stuff.token_program = ctx.accounts.token_program.key();
        stuff.rent = ctx.accounts.rent.key();
        stuff.dex_program = ctx.accounts.dex_program.key();
        Ok(())
    }

  pub  fn new_order2(
    ctx: Context<NewOrder2>,
        side: Side,
        limit_price: u64,
        coin_amount: u64,
        order_type: OrderType,
    )  -> Result<()> {
        let stuff = &ctx.accounts.stuff;
        let market_account = &ctx.accounts.market_account;
        let (price_lots, coin_lots, pc_lot_size) = {
            let market = MarketState::load(market_account, &stuff.proxy, true);
            (limit_price.checked_mul(market.as_ref().unwrap().coin_lot_size).unwrap()
                        .checked_div(market.as_ref().unwrap().pc_lot_size).unwrap()
                        .checked_div(BASE as u64).unwrap(), 
            coin_amount.checked_div(market.as_ref().unwrap().coin_lot_size).unwrap(),
            market.unwrap().pc_lot_size)
        };
        let max_native_pc_qty = price_lots.checked_mul(coin_lots).unwrap().checked_mul(pc_lot_size).unwrap();
        // Client order id is only used for cancels. Not used here so hardcode.
        let client_order_id = 0;
        // Limit is the dex's custom compute budge parameter, setting an upper
        // bound on the number of matching cycles the program can perform
        // before giving up and posting the remaining unmatched order.
        let limit = 65535;
        // wat
       // let short_name = stuff.prediction_market_account.short_name.as_ref().clone();
       // let seeds = &[
         //   short_name.trim_ascii_whitespace(),
        //    &[stuff.prediction_market_account.nonce],
        //];

//        let ctx2 = CpiContext::new_with_signer(stuff.dex_program.clone(), dex_accs, &[]);
        // if let Some(referral) = referral {
        //     ctx = ctx.with_remaining_accounts(vec![referral]);
        // }
        serum_dex::instruction::new_order (&stuff.market, 
            &stuff.open_orders, 
            &stuff.request_queue,
            &stuff.event_queue,&stuff.market_bids,&stuff.market_asks,
            &stuff.order_payer_token_account,&stuff.prediction_market_account,&stuff.coin_vault,
            &stuff.pc_vault,
            &stuff.token_program, &stuff.rent,Some(&stuff.prediction_market_account),
            &stuff.proxy, 
    side, 
    
    
    NonZeroU64::new(limit_price).unwrap(),
    NonZeroU64::new(max_native_pc_qty).unwrap(),
order_type,
0 as u64, SelfTradeBehavior::DecrementTake,limit,
NonZeroU64::new(max_native_pc_qty).unwrap());

        Ok(())
    }

  

}



#[account]
pub struct Stuff {
    token_program: Pubkey,
    proxy: Pubkey,   
    market: Pubkey,
        open_orders: Pubkey,
        request_queue: Pubkey,
        event_queue: Pubkey,
        market_bids: Pubkey,
        market_asks: Pubkey,
        order_payer_token_account: Pubkey,
        prediction_market_account: Pubkey,
        open_orders_authority: Pubkey,
        coin_vault: Pubkey,
        pc_vault: Pubkey,
        rent: Pubkey,
        dex_program: Pubkey
  }
  #[derive(Accounts)]
    pub struct NewOrder2 <'info>{
        market_account: UncheckedAccount<'info>,
        stuff: Account<'info, Stuff>,
        #[account(mut)]
        payer: Signer<'info>,
  }
  #[derive(Accounts)]
pub struct InitStuff <'info> {

    #[account(init, payer = payer, space = 300)]
    stuff: Account<'info, Stuff>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(address = system_program::ID)]
    system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
    pub struct SetMarket<'info> {
      #[account(mut)]
      stuff: Account<'info, Stuff>,
        proxy: UncheckedAccount<'info>,
      market: UncheckedAccount<'info>,
      open_orders: UncheckedAccount<'info>,
      request_queue: UncheckedAccount<'info>,
      event_queue: UncheckedAccount<'info>,
      bids: UncheckedAccount<'info>,
      asks: UncheckedAccount<'info>,
      prediction_market_account: UncheckedAccount<'info>,
      order_payer_token_account: UncheckedAccount<'info>,
      open_orders_authority:UncheckedAccount<'info>, // stays pubkey
      coin_vault: UncheckedAccount<'info>,
      pc_vault: UncheckedAccount<'info>,
      token_program: UncheckedAccount<'info>,
      rent: UncheckedAccount<'info>,
      dex_program: UncheckedAccount<'info>
  }