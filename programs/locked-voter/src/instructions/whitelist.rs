use crate::*;

/// Accounts for [locked_voter::approve_program_lock_privilege].
#[derive(Accounts)]
pub struct ApproveProgramLockPrivilege<'info> {
    /// The [Locker].
    pub locker: Account<'info, Locker>,
    /// [LockerWhitelistEntry].
    #[account(
        init,
        seeds = [
            b"LockerWhitelistEntry".as_ref(),
            locker.key().to_bytes().as_ref(),
            executable_id.key().to_bytes().as_ref(),
            whitelisted_owner.key().to_bytes().as_ref()
        ],
        bump,
        payer = payer,
        space = 97 + 8,
    )]
    pub whitelist_entry: Account<'info, LockerWhitelistEntry>,

    /// Governor for the [Locker].
    pub governor: Account<'info, Governor>,

    /// Smart wallet on the [Governor].
    pub smart_wallet: Signer<'info>,

    /// CHECK: ProgramId of the program to whitelist.
    pub executable_id: UncheckedAccount<'info>,

    /// CHECK: Owner whitelisted. If set to [System], then the program is whitelisted for all accounts.
    pub whitelisted_owner: UncheckedAccount<'info>,

    /// Payer of the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program.
    pub system_program: Program<'info, System>,
}

impl<'info> ApproveProgramLockPrivilege<'info> {
    /// Creates a new [LockerWhitelistEntry].
    pub fn approve_program_lock_privilege(&mut self, bump: u8) -> Result<()> {
        let whitelist_entry = &mut self.whitelist_entry;
        whitelist_entry.bump = bump;
        whitelist_entry.locker = self.locker.key();
        whitelist_entry.program_id = self.executable_id.key();
        whitelist_entry.owner = self.whitelisted_owner.key();

        emit!(ApproveLockPrivilegeEvent {
            locker: whitelist_entry.locker,
            program_id: whitelist_entry.program_id,
            owner: whitelist_entry.owner,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }
}

impl<'info> Validate<'info> for ApproveProgramLockPrivilege<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_eq!(self.governor.smart_wallet, self.smart_wallet);
        invariant!(
            self.executable_id.executable,
            "program_id must be an executable"
        );

        Ok(())
    }
}

/// Accounts for [locked_voter::revoke_program_lock_privilege].
#[derive(Accounts)]
pub struct RevokeProgramLockPrivilege<'info> {
    /// The [Locker].
    pub locker: Account<'info, Locker>,
    /// [LockerWhitelistEntry].
    #[account(mut, close = payer)]
    pub whitelist_entry: Account<'info, LockerWhitelistEntry>,

    /// Governor for the [Locker].
    pub governor: Account<'info, Governor>,

    /// Smart wallet on the [Governor].
    pub smart_wallet: Signer<'info>,

    /// CHECK: ProgramId of the program to whitelist.
    pub executable_id: UncheckedAccount<'info>,

    /// Payer of the initialization.
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl<'info> RevokeProgramLockPrivilege<'info> {
    /// Emit event that [LockerWhitelistEntry] was closed.
    pub fn revoke_program_lock_privilege(&mut self) -> Result<()> {
        emit!(RevokeLockPrivilegeEvent {
            locker: self.whitelist_entry.locker,
            program_id: self.whitelist_entry.program_id,
            timestamp: Clock::get()?.unix_timestamp
        });

        Ok(())
    }
}

impl<'info> Validate<'info> for RevokeProgramLockPrivilege<'info> {
    fn validate(&self) -> Result<()> {
        assert_keys_eq!(self.governor.smart_wallet, self.smart_wallet);
        assert_keys_eq!(self.whitelist_entry.program_id, self.executable_id);

        Ok(())
    }
}

#[event]
/// Event called in [locked_voter::approve_program_lock_privilege].
pub struct ApproveLockPrivilegeEvent {
    /// The [Locker].
    #[index]
    pub locker: Pubkey,
    /// ProgramId approved to make CPI calls to [locked_voter::lock].
    pub program_id: Pubkey,
    /// Owner of the [Escrow].
    pub owner: Pubkey,
    /// Timestamp of the event.
    pub timestamp: i64,
}

#[event]
/// Event called in [locked_voter::revoke_program_lock_privilege].
pub struct RevokeLockPrivilegeEvent {
    /// The [Locker].
    #[index]
    pub locker: Pubkey,
    /// ProgramId approved to make CPI calls to [locked_voter::lock].
    pub program_id: Pubkey,
    /// Timestamp of the event.
    pub timestamp: i64,
}
