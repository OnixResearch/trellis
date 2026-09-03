mod negative;
mod positive;

use super::OptimizationLevel;
use super::Settings;

const UNOPTIMIZED: Settings = Settings {
    optimization: OptimizationLevel::O0,
};
const OPTIMIZED: Settings = Settings {
    optimization: OptimizationLevel::O2,
};
