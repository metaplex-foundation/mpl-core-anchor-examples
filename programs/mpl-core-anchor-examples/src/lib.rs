pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("EXMPLWxvGsyDuHMcnZHBnUN32RaoyVowkzhyXmAVxmEM");

#[program]
pub mod mpl_core_anchor_wrapper {
    use super::*;

    pub fn create_v1(ctx: Context<CreateV1>, args: CreateV1Args) -> Result<()> {
        create_v1::CreateV1::handler(ctx, args)
    }

    pub fn create_collection_v1(
        ctx: Context<CreateCollectionV1>,
        args: CreateCollectionV1Args,
    ) -> Result<()> {
        create_collection_v1::CreateCollectionV1::handler(ctx, args)
    }

    pub fn add_plugin_v1(ctx: Context<AddPluginV1>, args: AddPluginV1Args) -> Result<()> {
        add_plugin_v1::AddPluginV1::handler(ctx, args)
    }

    pub fn add_collection_plugin_v1(
        ctx: Context<AddCollectionPluginV1>,
        args: AddCollectionPluginV1Args,
    ) -> Result<()> {
        add_collection_plugin_v1::AddCollectionPluginV1::handler(ctx, args)
    }

    pub fn remove_plugin_v1(ctx: Context<RemovePluginV1>, args: RemovePluginV1Args) -> Result<()> {
        remove_plugin_v1::RemovePluginV1::handler(ctx, args)
    }

    pub fn remove_collection_plugin_v1(
        ctx: Context<RemoveCollectionPluginV1>,
        args: RemoveCollectionPluginV1Args,
    ) -> Result<()> {
        remove_collection_plugin_v1::RemoveCollectionPluginV1::handler(ctx, args)
    }

    pub fn update_plugin_v1(ctx: Context<UpdatePluginV1>, args: UpdatePluginV1Args) -> Result<()> {
        update_plugin_v1::UpdatePluginV1::handler(ctx, args)
    }

    pub fn update_collection_plugin_v1(
        ctx: Context<UpdateCollectionPluginV1>,
        args: UpdateCollectionPluginV1Args,
    ) -> Result<()> {
        update_collection_plugin_v1::UpdateCollectionPluginV1::handler(ctx, args)
    }

    pub fn transfer_v1(ctx: Context<TransferV1>, args: TransferV1Args) -> Result<()> {
        transfer_v1::TransferV1::handler(ctx, args)
    }
}
