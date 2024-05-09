use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferV1<'info> {
    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The account paying for the storage fees.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,

    /// The new owner of the asset.
    /// CHECK: Just a destination, no checks needed.
    pub new_owner: AccountInfo<'info>,

    /// The system program.
    /// CHECK: Checked in mpl-core.
    pub system_program: Option<AccountInfo<'info>>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferV1Args {}

impl<'info> TransferV1<'info> {
    pub fn handler(ctx: Context<TransferV1>, _args: TransferV1Args) -> Result<()> {
        mpl_core::instructions::TransferV1Cpi {
            asset: &ctx.accounts.asset.to_account_info(),
            collection: ctx.accounts.collection.as_ref(),
            payer: &ctx.accounts.payer.to_account_info(),
            authority: ctx.accounts.authority.as_deref(),
            new_owner: &ctx.accounts.new_owner.to_account_info(),
            system_program: ctx.accounts.system_program.as_ref(),
            log_wrapper: ctx.accounts.log_wrapper.as_ref(),
            __program: &ctx.accounts.mpl_core,
            __args: mpl_core::instructions::TransferV1InstructionArgs {
                compression_proof: None,
            },
        }
        .invoke()?;

        Ok(())
    }
}
