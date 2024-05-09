use anchor_lang::prelude::*;
use mpl_core::types::PluginType;

#[derive(Accounts)]
pub struct RemoveCollectionPluginV1<'info> {
    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: AccountInfo<'info>,

    /// The account paying for the storage fees.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,

    /// The system program.
    pub system_program: Program<'info, System>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemoveCollectionPluginV1Args {
    pub name: String,
    pub uri: String,
    pub plugin_type: PluginType,
}

impl<'info> RemoveCollectionPluginV1<'info> {
    pub fn handler(
        ctx: Context<RemoveCollectionPluginV1>,
        args: RemoveCollectionPluginV1Args,
    ) -> Result<()> {
        mpl_core::instructions::RemoveCollectionPluginV1Cpi {
            collection: &ctx.accounts.collection.to_account_info(),
            authority: ctx.accounts.authority.as_deref(),
            payer: &ctx.accounts.payer.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            log_wrapper: ctx.accounts.log_wrapper.as_ref(),
            __program: &ctx.accounts.mpl_core,
            __args: mpl_core::instructions::RemoveCollectionPluginV1InstructionArgs {
                plugin_type: args.plugin_type,
            },
        }
        .invoke()?;

        Ok(())
    }
}
