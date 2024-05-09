use anchor_lang::prelude::*;
use mpl_core::types::{PluginAuthority, PluginType};

#[derive(Accounts)]
pub struct ApprovePluginAuthorityV1<'info> {
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
pub struct ApprovePluginAuthorityV1Args {
    pub plugin_type: PluginType,
    pub new_authority: PluginAuthority,
}

impl<'info> ApprovePluginAuthorityV1<'info> {
    pub fn handler(
        ctx: Context<ApprovePluginAuthorityV1>,
        args: ApprovePluginAuthorityV1Args,
    ) -> Result<()> {
        mpl_core::instructions::ApprovePluginAuthorityV1Cpi {
            asset: &ctx.accounts.asset.to_account_info(),
            collection: ctx.accounts.collection.as_ref(),
            authority: ctx.accounts.authority.as_deref(),
            payer: &ctx.accounts.payer.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            log_wrapper: ctx.accounts.log_wrapper.as_ref(),
            __program: &ctx.accounts.mpl_core,
            __args: mpl_core::instructions::ApprovePluginAuthorityV1InstructionArgs {
                plugin_type: args.plugin_type,
                new_authority: args.new_authority,
            },
        }
        .invoke()?;

        Ok(())
    }
}
