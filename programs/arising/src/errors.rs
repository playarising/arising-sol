use anchor_lang::prelude::*;

/// Program error codes.
#[error_code]
pub enum ArisingError {
    #[msg("Authority is not the program authority.")]
    InvalidAuthority,
    #[msg("Payer is not owner of the token.")]
    InvalidOwner,
    #[msg("The ID for the forge recipe is invalid")]
    InvalidForgeRecipeID,
    #[msg("The ID for the craft recipe is invalid")]
    InvalidCraftRecipeID,
    #[msg("The ID for the upgrade recipe is invalid")]
    InvalidUpgradeRecipeID,
    #[msg("The ID for the quest is invalid")]
    InvalidQuestID,
    #[msg("Payer is not owner of the requested character")]
    InvalidCharacterOwner,
    #[msg("The amount of points you are trying to assign is invalid")]
    InvalidAssignPoints,
}