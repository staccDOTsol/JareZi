use {
    anchor_lang::{
        solana_program::system_program,
    }
};

use borsh::{BorshSerialize, BorshDeserialize};
use std::num::NonZeroU64;
use serum_dex::matching::*;

use serum_dex::state::*;
use serum_dex::instruction::*;
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



   
    pub  fn set_market(ctx: Context<SetMarket>
    )  -> Result<()> {
        let stuff = ctx.accounts.stuff;
        stuff.proxy = ctx.accounts.proxy.key();
        stuff.market = ctx.accounts.market.to_account_info();
        stuff.open_orders = ctx.accounts.open_orders.to_account_info();
        stuff.request_queue = ctx.accounts.request_queue.to_account_info();
        stuff.event_queue = ctx.accounts.event_queue.to_account_info();
        stuff.market_bids = ctx.accounts.bids.to_account_info();
        stuff.market_asks = ctx.accounts.asks.to_account_info();
        stuff.order_payer_token_account = ctx.accounts.order_payer_token_account.key();
        stuff.open_orders_authority = ctx.accounts.prediction_market_account.to_account_info();
        stuff.coin_vault = ctx.accounts.coin_vault.to_account_info();
        stuff.pc_vault = ctx.accounts.pc_vault.to_account_info();
        stuff.token_program = ctx.accounts.token_program.to_account_info();
        stuff.rent = ctx.accounts.rent.to_account_info();
        stuff.dex_program = ctx.accounts.dex_program.to_account_info();
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
        let (price_lots, coin_lots, pc_lot_size) = {
            let market = MarketState::load(&stuff.market, &stuff.proxy, true);
            (limit_price.checked_mul(market.unwrap().coin_lot_size).unwrap()
                        .checked_div(market.unwrap().pc_lot_size).unwrap()
                        .checked_div(BASE as u64).unwrap(), 
            coin_amount.checked_div(market.unwrap().coin_lot_size).unwrap(),
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

        let seeds = &[];
        let signer =  &ctx.accounts.signer;
        let mut dex_accs = vec![
            AccountMeta::new(*stuff.market.key, false),
            AccountMeta::new(*stuff.open_orders.key, false),
            AccountMeta::new(*stuff.request_queue.key, false),
            AccountMeta::new(*stuff.event_queue.key, false),
            AccountMeta::new(*stuff.market_bids.key, false),
            AccountMeta::new(*stuff.market_asks.key, false),
            AccountMeta::new(stuff.order_payer_token_account.clone(), false),
            AccountMeta::new_readonly(stuff.prediction_market_account.to_account_info().key(), true), // signer?
            AccountMeta::new(*stuff.coin_vault.key, false),
            AccountMeta::new(*stuff.pc_vault.key, false),
            AccountMeta::new_readonly(*stuff.token_program.key, false),
            AccountMeta::new_readonly(*stuff.rent.key, false),
        ];
        let ctx2 = CpiContext::new_with_signer(stuff.dex_program.clone(), dex_accs, &[]);
        // if let Some(referral) = referral {
        //     ctx = ctx.with_remaining_accounts(vec![referral]);
        // }
        new_order (
            ctx2,
            side.into(),
            NonZeroU64::new(price_lots).unwrap(),
            NonZeroU64::new(coin_lots).unwrap(),
            NonZeroU64::new(max_native_pc_qty).unwrap(),
            SelfTradeBehavior::DecrementTake,
            order_type,
            client_order_id,
            limit,
        );
        Ok(())
    }

  

}



#[account]
pub struct Stuff<'info> {
    token_program: AccountInfo<'info>,
    proxy: Pubkey,   
    market: AccountInfo<'info>,
        open_orders: AccountInfo<'info>,
        request_queue: AccountInfo<'info>,
        event_queue: AccountInfo<'info>,
        market_bids: AccountInfo<'info>,
        market_asks: AccountInfo<'info>,
        order_payer_token_account: Pubkey,
        prediction_market_account: AccountInfo<'info>,
        open_orders_authority: AccountInfo<'info>,
        coin_vault: AccountInfo<'info>,
        pc_vault: AccountInfo<'info>,
        rent: AccountInfo<'info>,
        dex_program: AccountInfo<'info>
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

#[derive(Accounts)]
pub struct NewOrder2 <'info>{
    stuff: Account<'info, Stuff<'info>>,
    #[account(mut)]
    signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct InitStuff<'info> {

#[account(init, payer = payer, space = 300)]
stuff: Account<'info, Stuff<'info>>,
#[account(mut)]
payer: Signer<'info>,
#[account(address = system_program::ID)]
system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SetMarket<'info> {
  #[account(mut)]
  stuff: Account<'info, Stuff<'info>>,
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