use anchor_lang::prelude::*;
use mpl_core::types::{DataState, PluginAuthorityPair};

#[derive(Accounts)]
pub struct CreateV1<'info> {
    /// The address of the new asset.
    #[account(mut)]
    pub asset: Signer<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The authority signing for creation.
    pub authority: Option<Signer<'info>>,

    /// The account paying for the storage fees.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner of the new asset. Defaults to the authority if not present.
    /// CHECK: Checked in mpl-core.
    pub owner: Option<AccountInfo<'info>>,

    /// The authority on the new asset.
    /// CHECK: Checked in mpl-core.
    pub update_authority: Option<AccountInfo<'info>>,

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

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateV1Args {
    pub name: String,
    pub uri: String,
    // TODO: Add plugin_authority_pair
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

impl<'info> CreateV1<'info> {
    pub fn handler(ctx: Context<CreateV1>, args: CreateV1Args) -> Result<()> {
        mpl_core::instructions::CreateV1Cpi {
            asset: &ctx.accounts.asset.to_account_info(),
            collection: ctx.accounts.collection.as_ref(),
            authority: ctx.accounts.authority.as_deref(),
            payer: &ctx.accounts.payer.to_account_info(),
            owner: ctx.accounts.owner.as_ref(),
            update_authority: ctx.accounts.update_authority.as_ref(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            log_wrapper: ctx.accounts.log_wrapper.as_ref(),
            __program: &ctx.accounts.mpl_core,
            __args: mpl_core::instructions::CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            },
        }
        .invoke()?;

        Ok(())
    }
}
