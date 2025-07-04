// This file is autogenerated with https://github.com/acheroncrypto/native-to-anchor

use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod axelar_solana_its {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        chain_name: String,
        its_hub_address: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_pause_status(ctx: Context<SetPauseStatus>, paused: bool) -> Result<()> {
        Ok(())
    }

    pub fn set_trusted_chain(ctx: Context<SetTrustedChain>, chain_name: String) -> Result<()> {
        Ok(())
    }

    pub fn remove_trusted_chain(
        ctx: Context<RemoveTrustedChain>,
        chain_name: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn approve_deploy_remote_interchain_token(
        ctx: Context<ApproveDeployRemoteInterchainToken>,
        deployer: Pubkey,
        salt: [u8; 32],
        destination_chain: String,
        destination_minter: Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn revoke_deploy_remote_interchain_token(
        ctx: Context<RevokeDeployRemoteInterchainToken>,
        deployer: Pubkey,
        salt: [u8; 32],
        destination_chain: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn register_canonical_interchain_token(
        ctx: Context<RegisterCanonicalInterchainToken>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn deploy_remote_canonical_interchain_token(
        ctx: Context<DeployRemoteCanonicalInterchainToken>,
        destination_chain: String,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn interchain_transfer(
        ctx: Context<InterchainTransfer>,
        token_id: [u8; 32],
        destination_chain: String,
        destination_address: Vec<u8>,
        amount: u64,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn deploy_interchain_token(
        ctx: Context<DeployInterchainToken>,
        salt: [u8; 32],
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn deploy_remote_interchain_token(
        ctx: Context<DeployRemoteInterchainToken>,
        salt: [u8; 32],
        destination_chain: String,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn deploy_remote_interchain_token_with_minter(
        ctx: Context<DeployRemoteInterchainTokenWithMinter>,
        salt: [u8; 32],
        destination_chain: String,
        destination_minter: Vec<u8>,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn register_token_metadata(
        ctx: Context<RegisterTokenMetadata>,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn register_custom_token(
        ctx: Context<RegisterCustomToken>,
        salt: [u8; 32],
        token_manager_type: Type,
        operator: Option<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn link_token(
        ctx: Context<LinkToken>,
        salt: [u8; 32],
        destination_chain: String,
        destination_token_address: Vec<u8>,
        token_manager_type: Type,
        link_params: Vec<u8>,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn call_contract_with_interchain_token(
        ctx: Context<CallContractWithInterchainToken>,
        token_id: [u8; 32],
        destination_chain: String,
        destination_address: Vec<u8>,
        amount: u64,
        data: Vec<u8>,
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn call_contract_with_interchain_token_offchain_data(
        ctx: Context<CallContractWithInterchainTokenOffchainData>,
        token_id: [u8; 32],
        destination_chain: String,
        destination_address: Vec<u8>,
        amount: u64,
        payload_hash: [u8; 32],
        gas_value: u64,
        signing_pda_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_flow_limit(ctx: Context<SetFlowLimit>, flow_limit: u64) -> Result<()> {
        Ok(())
    }

    pub fn transfer_operatorship(ctx: Context<TransferOperatorship>) -> Result<()> {
        Ok(())
    }

    pub fn propose_operatorship(ctx: Context<ProposeOperatorship>) -> Result<()> {
        Ok(())
    }

    pub fn accept_operatorship(ctx: Context<AcceptOperatorship>) -> Result<()> {
        Ok(())
    }

    pub fn add_token_manager_flow_limiter(ctx: Context<AddTokenManagerFlowLimiter>) -> Result<()> {
        Ok(())
    }

    pub fn remove_token_manager_flow_limiter(
        ctx: Context<RemoveTokenManagerFlowLimiter>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_token_manager_flow_limit(
        ctx: Context<SetTokenManagerFlowLimit>,
        flow_limit: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn transfer_token_manager_operatorship(
        ctx: Context<TransferTokenManagerOperatorship>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn propose_token_manager_operatorship(
        ctx: Context<ProposeTokenManagerOperatorship>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn accept_token_manager_operatorship(
        ctx: Context<AcceptTokenManagerOperatorship>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn handover_mint_authority(
        ctx: Context<HandoverMintAuthority>,
        token_id: [u8; 32],
    ) -> Result<()> {
        Ok(())
    }

    pub fn mint_interchain_token(ctx: Context<MintInterchainToken>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn transfer_interchain_token_mintership(
        ctx: Context<TransferInterchainTokenMintership>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn propose_interchain_token_mintership(
        ctx: Context<ProposeInterchainTokenMintership>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn accept_interchain_token_mintership(
        ctx: Context<AcceptInterchainTokenMintership>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    program_data_address: AccountInfo<'info>,
    #[account(mut)]
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    operator: AccountInfo<'info>,
    #[account(mut)]
    user_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetPauseStatus<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    program_data_address: AccountInfo<'info>,
    #[account(mut)]
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTrustedChain<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    program_data_address: AccountInfo<'info>,
    #[account(mut)]
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveTrustedChain<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    program_data_address: AccountInfo<'info>,
    #[account(mut)]
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveDeployRemoteInterchainToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    token_manager_pda: AccountInfo<'info>,
    roles_pda: AccountInfo<'info>,
    #[account(mut)]
    deploy_approval_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeDeployRemoteInterchainToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    deploy_approval_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterCanonicalInterchainToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    token_metadata_account: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(mut)]
    token_manager_ata: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    spl_associated_token_account: AccountInfo<'info>,
    #[account(mut)]
    its_user_roles_pda: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DeployRemoteCanonicalInterchainToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    mint: AccountInfo<'info>,
    metadata_account: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InterchainTransfer<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    source_account: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_ata: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    #[account(mut)]
    flow_slot_pda: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DeployInterchainToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(mut)]
    token_manager_ata: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    spl_associated_token_account: AccountInfo<'info>,
    #[account(mut)]
    its_user_roles_pda: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
    sysvar_instructions: AccountInfo<'info>,
    mpl_token_metadata: AccountInfo<'info>,
    #[account(mut)]
    metadata_account: AccountInfo<'info>,
    #[account(mut)]
    payer_ata: AccountInfo<'info>,
    optional_minter: AccountInfo<'info>,
    #[account(mut)]
    optional_minter_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DeployRemoteInterchainToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    mint: AccountInfo<'info>,
    metadata_account: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DeployRemoteInterchainTokenWithMinter<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    mint: AccountInfo<'info>,
    metadata_account: AccountInfo<'info>,
    minter: AccountInfo<'info>,
    #[account(mut)]
    deploy_approval: AccountInfo<'info>,
    minter_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RegisterTokenMetadata<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    mint: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RegisterCustomToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    token_metadata_account: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(mut)]
    token_manager_ata: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    spl_associated_token_account: AccountInfo<'info>,
    #[account(mut)]
    its_user_roles_pda: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
    #[account(mut)]
    optional_operator: AccountInfo<'info>,
    #[account(mut)]
    optional_operator_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LinkToken<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    token_manager_pda: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CallContractWithInterchainToken<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    source_account: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_ata: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    #[account(mut)]
    flow_slot_pda: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CallContractWithInterchainTokenOffchainData<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    source_account: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_ata: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    #[account(mut)]
    flow_slot_pda: AccountInfo<'info>,
    gateway_root_pda: AccountInfo<'info>,
    axelar_solana_gateway: AccountInfo<'info>,
    #[account(mut)]
    gas_config_pda: AccountInfo<'info>,
    gas_service: AccountInfo<'info>,
    system_program: Program<'info, System>,
    its_root_pda: AccountInfo<'info>,
    call_contract_signing_pda: AccountInfo<'info>,
    ID: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetFlowLimit<'info> {
    payer: Signer<'info>,
    its_root_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_pda: AccountInfo<'info>,
    its_user_roles_pda: AccountInfo<'info>,
    token_manager_user_roles_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferOperatorship<'info> {
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    payer_roles_pda: AccountInfo<'info>,
    its_root_pda: AccountInfo<'info>,
    to: AccountInfo<'info>,
    #[account(mut)]
    destination_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProposeOperatorship<'info> {
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    its_root_pda: AccountInfo<'info>,
    to: AccountInfo<'info>,
    destination_roles_pda: AccountInfo<'info>,
    #[account(mut)]
    proposal_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AcceptOperatorship<'info> {
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    payer_roles_pda: AccountInfo<'info>,
    its_root_pda: AccountInfo<'info>,
    from: AccountInfo<'info>,
    #[account(mut)]
    origin_roles_pda: AccountInfo<'info>,
    #[account(mut)]
    proposal_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddTokenManagerFlowLimiter<'info> {
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    flow_limiter: AccountInfo<'info>,
    #[account(mut)]
    flow_limiter_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveTokenManagerFlowLimiter<'info> {
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    flow_limiter: AccountInfo<'info>,
    #[account(mut)]
    flow_limiter_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetTokenManagerFlowLimit<'info> {
    payer: Signer<'info>,
    its_root_pda: AccountInfo<'info>,
    #[account(mut)]
    token_manager_pda: AccountInfo<'info>,
    token_manager_user_roles_pda: AccountInfo<'info>,
    its_user_roles_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokenManagerOperatorship<'info> {
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    to: AccountInfo<'info>,
    #[account(mut)]
    destination_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProposeTokenManagerOperatorship<'info> {
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    to: AccountInfo<'info>,
    #[account(mut)]
    destination_roles_pda: AccountInfo<'info>,
    #[account(mut)]
    proposal_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AcceptTokenManagerOperatorship<'info> {
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    from: AccountInfo<'info>,
    #[account(mut)]
    origin_roles_pda: AccountInfo<'info>,
    #[account(mut)]
    proposal_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct HandoverMintAuthority<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    its_root_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    #[account(mut)]
    minter_roles_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintInterchainToken<'info> {
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(mut)]
    to: AccountInfo<'info>,
    its_root_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    minter: Signer<'info>,
    minter_roles_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferInterchainTokenMintership<'info> {
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    to: AccountInfo<'info>,
    #[account(mut)]
    destination_roles_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProposeInterchainTokenMintership<'info> {
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    to: AccountInfo<'info>,
    #[account(mut)]
    destination_roles_pda: AccountInfo<'info>,
    #[account(mut)]
    proposal_pda: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AcceptInterchainTokenMintership<'info> {
    its_root_pda: AccountInfo<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut)]
    payer_roles_pda: AccountInfo<'info>,
    token_manager_pda: AccountInfo<'info>,
    from: AccountInfo<'info>,
    #[account(mut)]
    origin_roles_pda: AccountInfo<'info>,
    #[account(mut)]
    proposal_pda: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Type {
    /// For tokens that are deployed directly from ITS itself they use a native
    /// interchain token manager. Tokens that are deployed via the frontend
    /// portal also use this type of manager.
    NativeInterchainToken,

    /// The mint/burnFrom token manager type, allows tokens to be burnt on the
    /// source chain when they are transferred out of that chain and minted they
    /// are transferred back into the source chain. As the name suggests when
    /// the token is burnt on the source chain the manager is looking to trigger
    /// the `burnFrom` function on the token rather than the `burn` function.
    /// The main implication is that ITS must be approved to call `burnFrom` by
    /// the token. The manager must be granted the role to be able to `mint` the
    /// token on the destination chain.
    MintBurnFrom,

    /// Token integrations using the lock/unlock token manager will have their
    /// token locked with their token’s manager. Only a single lock/unlock
    /// manager can exist for a token as having multiple lock/unlock managers
    /// would make it substantially more difficult to manage liquidity across
    /// many different blockchains. These token managers are best used in the
    /// case where a token has a “home chain” where a token can be locked. On
    /// the remote chains users can then use a wrapped version of that token
    /// which derives it’s value from a locked token back on the home chain.
    /// Canonical tokens for example deployed via ITS are examples where a
    /// lock/unlock token manager type is useful. When bridging tokens out of
    /// the destination chain (locking them at the manager) ITS will call the
    /// `transferTokenFrom` function, which in turn will call the
    /// `safeTransferFrom` function. For this transaction to be successful, ITS
    /// must be `approved` to call the `safeTransferFrom` function, otherwise
    /// the call will revert.
    LockUnlock,

    /// This manager type is similar to the lock/unlock token manager, where the
    /// manager locks
    /// the token on it’s “home chain” when it is bridged out and unlocks it
    /// when it is bridged back. The key feature with this token manager is
    /// that you have the option to set a fee that will be deducted when
    /// executing an `interchainTransfer`.
    ///
    /// This token type is currently not supported.
    LockUnlockFee,

    /// The mint/burn token manager type is the most common token manager type
    /// used for integrating tokens to ITS. This token manager type is used when
    /// there is no home chain for your token and allows you to `burn` tokens
    /// from the source chain and `mint` tokens on the destination chain. The
    /// manager will need to be granted the role to be able to execute the
    /// `mint` and `burn` function on the token.
    MintBurn,
}
