use anchor_lang::prelude::*;

/// Character context errors
#[error_code]
pub enum CharacterError {
    #[msg("Character: not enough level to perform this actions")]
    NotEnoughLevel,
    #[msg("Character: not enough pool points to consume")]
    NotEnoughPoolPointsToConsume,
    #[msg("Character: not enough base points to sacrifice")]
    NotEnoughBasePointsToSacrifice,
    #[msg("Character: not enough points available to assign")]
    NotEnoughAssignablePoints,
    #[msg("Character: not able to refresh now")]
    RefreshNotAvailable,
    #[msg("Character: character not able to forge a recipe now")]
    NotAbleToForgeRecipe,
    #[msg("Character: character not able to start a quest")]
    NotAbleToQuest,
    #[msg("Character: character not able to claim the forge recipe now")]
    NotAbleToClaimForgeRecipe,
    #[msg("Character: character not able to claim the quest now")]
    NotAbleToClaimQuest,
    #[msg("Character: not enough resources to perform the task")]
    NotEnoughResources,
}

/// Forge context errors
#[error_code]
pub enum ForgeError {
    #[msg("Forge: recipe is not available.")]
    NotAvailable,
    #[msg("Forge: invalid forge recipe ID.")]
    InvalidID,
}

/// Quest context errors
#[error_code]
pub enum QuestError {
    #[msg("Quest: quest is not available.")]
    NotAvailable,
    #[msg("Quest: invalid quest ID.")]
    InvalidID,
}

/// Program context errors.
#[error_code]
pub enum ArisingError {
    #[msg("Arising: authority is not the program authority.")]
    InvalidAuthority,
    #[msg("Arising: payer is not owner of the token.")]
    InvalidOwner,
}