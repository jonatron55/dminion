use crate::dice::{DiceExpr, Roll};

#[tauri::command]
pub fn roll(expr: String) -> Result<Roll, String> {
    let dice_expr = DiceExpr::parse(&expr).map_err(|e| e.to_string())?;
    let result = dice_expr
        .roll(&mut rand::thread_rng())
        .map_err(|e| e.to_string())?;
    Ok(result)
}
