pub mod colors {
    #![allow(unused_variables)]
    #![allow(dead_code)]
    use macroquad::color::Color;
    use macroquad::color_u8;

    // =============================================================================
    // Tailwind CSS Full Color Palette
    // =============================================================================

    // SLATE
    pub const SLATE_50: Color = color_u8!(248, 250, 252, 255);
    pub const SLATE_100: Color = color_u8!(241, 245, 249, 255);
    pub const SLATE_200: Color = color_u8!(226, 232, 240, 255);
    pub const SLATE_300: Color = color_u8!(203, 213, 225, 255);
    pub const SLATE_400: Color = color_u8!(148, 163, 184, 255);
    pub const SLATE_500: Color = color_u8!(100, 116, 139, 255);
    pub const SLATE_600: Color = color_u8!(71, 85, 105, 255);
    pub const SLATE_700: Color = color_u8!(51, 65, 85, 255);
    pub const SLATE_800: Color = color_u8!(30, 41, 59, 255);
    pub const SLATE_900: Color = color_u8!(15, 23, 42, 255);
    pub const SLATE_950: Color = color_u8!(2, 6, 23, 255);

    // GRAY
    pub const GRAY_50: Color = color_u8!(249, 250, 251, 255);
    pub const GRAY_100: Color = color_u8!(243, 244, 246, 255);
    pub const GRAY_200: Color = color_u8!(229, 231, 235, 255);
    pub const GRAY_300: Color = color_u8!(209, 213, 219, 255);
    pub const GRAY_400: Color = color_u8!(156, 163, 175, 255);
    pub const GRAY_500: Color = color_u8!(107, 114, 128, 255);
    pub const GRAY_600: Color = color_u8!(75, 85, 99, 255);
    pub const GRAY_700: Color = color_u8!(55, 65, 81, 255);
    pub const GRAY_800: Color = color_u8!(31, 41, 55, 255);
    pub const GRAY_900: Color = color_u8!(17, 24, 39, 255);
    pub const GRAY_950: Color = color_u8!(3, 7, 18, 255);

    // ZINC
    pub const ZINC_50: Color = color_u8!(250, 250, 250, 255);
    pub const ZINC_100: Color = color_u8!(244, 244, 245, 255);
    pub const ZINC_200: Color = color_u8!(228, 228, 231, 255);
    pub const ZINC_300: Color = color_u8!(212, 212, 216, 255);
    pub const ZINC_400: Color = color_u8!(161, 161, 170, 255);
    pub const ZINC_500: Color = color_u8!(113, 113, 122, 255);
    pub const ZINC_600: Color = color_u8!(82, 82, 91, 255);
    pub const ZINC_700: Color = color_u8!(63, 63, 70, 255);
    pub const ZINC_800: Color = color_u8!(39, 39, 42, 255);
    pub const ZINC_900: Color = color_u8!(24, 24, 27, 255);
    pub const ZINC_950: Color = color_u8!(9, 9, 11, 255);

    // NEUTRAL
    pub const NEUTRAL_50: Color = color_u8!(250, 250, 250, 255);
    pub const NEUTRAL_100: Color = color_u8!(245, 245, 245, 255);
    pub const NEUTRAL_200: Color = color_u8!(229, 229, 229, 255);
    pub const NEUTRAL_300: Color = color_u8!(212, 212, 212, 255);
    pub const NEUTRAL_400: Color = color_u8!(163, 163, 163, 255);
    pub const NEUTRAL_500: Color = color_u8!(115, 115, 115, 255);
    pub const NEUTRAL_600: Color = color_u8!(82, 82, 82, 255);
    pub const NEUTRAL_700: Color = color_u8!(64, 64, 64, 255);
    pub const NEUTRAL_800: Color = color_u8!(38, 38, 38, 255);
    pub const NEUTRAL_900: Color = color_u8!(23, 23, 23, 255);
    pub const NEUTRAL_950: Color = color_u8!(10, 10, 10, 255);

    // STONE
    pub const STONE_50: Color = color_u8!(250, 250, 249, 255);
    pub const STONE_100: Color = color_u8!(245, 245, 244, 255);
    pub const STONE_200: Color = color_u8!(231, 229, 228, 255);
    pub const STONE_300: Color = color_u8!(214, 211, 209, 255);
    pub const STONE_400: Color = color_u8!(168, 162, 158, 255);
    pub const STONE_500: Color = color_u8!(120, 113, 108, 255);
    pub const STONE_600: Color = color_u8!(87, 83, 78, 255);
    pub const STONE_700: Color = color_u8!(68, 64, 60, 255);
    pub const STONE_800: Color = color_u8!(41, 37, 36, 255);
    pub const STONE_900: Color = color_u8!(28, 25, 23, 255);
    pub const STONE_950: Color = color_u8!(12, 10, 9, 255);

    // RED
    pub const RED_50: Color = color_u8!(254, 242, 242, 255);
    pub const RED_100: Color = color_u8!(254, 226, 226, 255);
    pub const RED_200: Color = color_u8!(254, 202, 202, 255);
    pub const RED_300: Color = color_u8!(252, 165, 165, 255);
    pub const RED_400: Color = color_u8!(248, 113, 113, 255);
    pub const RED_500: Color = color_u8!(239, 68, 68, 255);
    pub const RED_600: Color = color_u8!(220, 38, 38, 255);
    pub const RED_700: Color = color_u8!(185, 28, 28, 255);
    pub const RED_800: Color = color_u8!(153, 27, 27, 255);
    pub const RED_900: Color = color_u8!(127, 29, 29, 255);
    pub const RED_950: Color = color_u8!(69, 10, 10, 255);

    // ORANGE
    pub const ORANGE_50: Color = color_u8!(255, 247, 237, 255);
    pub const ORANGE_100: Color = color_u8!(255, 237, 213, 255);
    pub const ORANGE_200: Color = color_u8!(254, 215, 170, 255);
    pub const ORANGE_300: Color = color_u8!(253, 186, 116, 255);
    pub const ORANGE_400: Color = color_u8!(251, 146, 60, 255);
    pub const ORANGE_500: Color = color_u8!(249, 115, 22, 255);
    pub const ORANGE_600: Color = color_u8!(234, 88, 12, 255);
    pub const ORANGE_700: Color = color_u8!(194, 65, 12, 255);
    pub const ORANGE_800: Color = color_u8!(154, 52, 18, 255);
    pub const ORANGE_900: Color = color_u8!(124, 45, 18, 255);
    pub const ORANGE_950: Color = color_u8!(67, 20, 7, 255);

    // AMBER
    pub const AMBER_50: Color = color_u8!(255, 251, 235, 255);
    pub const AMBER_100: Color = color_u8!(254, 243, 199, 255);
    pub const AMBER_200: Color = color_u8!(253, 230, 138, 255);
    pub const AMBER_300: Color = color_u8!(252, 211, 77, 255);
    pub const AMBER_400: Color = color_u8!(251, 191, 36, 255);
    pub const AMBER_500: Color = color_u8!(245, 158, 11, 255);
    pub const AMBER_600: Color = color_u8!(217, 119, 6, 255);
    pub const AMBER_700: Color = color_u8!(180, 83, 9, 255);
    pub const AMBER_800: Color = color_u8!(146, 64, 14, 255);
    pub const AMBER_900: Color = color_u8!(120, 53, 15, 255);
    pub const AMBER_950: Color = color_u8!(69, 26, 3, 255);

    // YELLOW
    pub const YELLOW_50: Color = color_u8!(254, 252, 232, 255);
    pub const YELLOW_100: Color = color_u8!(254, 249, 195, 255);
    pub const YELLOW_200: Color = color_u8!(254, 240, 138, 255);
    pub const YELLOW_300: Color = color_u8!(253, 224, 71, 255);
    pub const YELLOW_400: Color = color_u8!(250, 204, 21, 255);
    pub const YELLOW_500: Color = color_u8!(234, 179, 8, 255);
    pub const YELLOW_600: Color = color_u8!(202, 138, 4, 255);
    pub const YELLOW_700: Color = color_u8!(161, 98, 7, 255);
    pub const YELLOW_800: Color = color_u8!(133, 77, 14, 255);
    pub const YELLOW_900: Color = color_u8!(113, 63, 18, 255);
    pub const YELLOW_950: Color = color_u8!(66, 32, 6, 255);

    // LIME
    pub const LIME_50: Color = color_u8!(247, 254, 231, 255);
    pub const LIME_100: Color = color_u8!(236, 252, 203, 255);
    pub const LIME_200: Color = color_u8!(217, 249, 157, 255);
    pub const LIME_300: Color = color_u8!(190, 242, 100, 255);
    pub const LIME_400: Color = color_u8!(163, 230, 53, 255);
    pub const LIME_500: Color = color_u8!(132, 204, 22, 255);
    pub const LIME_600: Color = color_u8!(101, 163, 13, 255);
    pub const LIME_700: Color = color_u8!(77, 124, 15, 255);
    pub const LIME_800: Color = color_u8!(63, 98, 18, 255);
    pub const LIME_900: Color = color_u8!(54, 83, 20, 255);
    pub const LIME_950: Color = color_u8!(26, 46, 5, 255);

    // GREEN
    pub const GREEN_50: Color = color_u8!(240, 253, 244, 255);
    pub const GREEN_100: Color = color_u8!(220, 252, 231, 255);
    pub const GREEN_200: Color = color_u8!(187, 247, 208, 255);
    pub const GREEN_300: Color = color_u8!(134, 239, 172, 255);
    pub const GREEN_400: Color = color_u8!(74, 222, 128, 255);
    pub const GREEN_500: Color = color_u8!(34, 197, 94, 255);
    pub const GREEN_600: Color = color_u8!(22, 163, 74, 255);
    pub const GREEN_700: Color = color_u8!(21, 128, 61, 255);
    pub const GREEN_800: Color = color_u8!(22, 101, 52, 255);
    pub const GREEN_900: Color = color_u8!(20, 83, 45, 255);
    pub const GREEN_950: Color = color_u8!(5, 46, 22, 255);

    // EMERALD
    pub const EMERALD_50: Color = color_u8!(236, 253, 245, 255);
    pub const EMERALD_100: Color = color_u8!(209, 250, 229, 255);
    pub const EMERALD_200: Color = color_u8!(167, 243, 208, 255);
    pub const EMERALD_300: Color = color_u8!(110, 231, 183, 255);
    pub const EMERALD_400: Color = color_u8!(52, 211, 153, 255);
    pub const EMERALD_500: Color = color_u8!(16, 185, 129, 255);
    pub const EMERALD_600: Color = color_u8!(5, 150, 105, 255);
    pub const EMERALD_700: Color = color_u8!(4, 120, 87, 255);
    pub const EMERALD_800: Color = color_u8!(6, 95, 70, 255);
    pub const EMERALD_900: Color = color_u8!(6, 78, 59, 255);
    pub const EMERALD_950: Color = color_u8!(2, 44, 34, 255);

    // TEAL
    pub const TEAL_50: Color = color_u8!(240, 253, 250, 255);
    pub const TEAL_100: Color = color_u8!(204, 251, 241, 255);
    pub const TEAL_200: Color = color_u8!(153, 246, 228, 255);
    pub const TEAL_300: Color = color_u8!(94, 234, 212, 255);
    pub const TEAL_400: Color = color_u8!(45, 212, 191, 255);
    pub const TEAL_500: Color = color_u8!(20, 184, 166, 255);
    pub const TEAL_600: Color = color_u8!(13, 148, 136, 255);
    pub const TEAL_700: Color = color_u8!(15, 118, 110, 255);
    pub const TEAL_800: Color = color_u8!(17, 94, 89, 255);
    pub const TEAL_900: Color = color_u8!(19, 78, 74, 255);
    pub const TEAL_950: Color = color_u8!(4, 47, 46, 255);

    // CYAN
    pub const CYAN_50: Color = color_u8!(236, 254, 255, 255);
    pub const CYAN_100: Color = color_u8!(207, 250, 254, 255);
    pub const CYAN_200: Color = color_u8!(165, 243, 252, 255);
    pub const CYAN_300: Color = color_u8!(103, 232, 249, 255);
    pub const CYAN_400: Color = color_u8!(34, 211, 238, 255);
    pub const CYAN_500: Color = color_u8!(6, 182, 212, 255);
    pub const CYAN_600: Color = color_u8!(8, 145, 178, 255);
    pub const CYAN_700: Color = color_u8!(14, 116, 144, 255);
    pub const CYAN_800: Color = color_u8!(21, 94, 117, 255);
    pub const CYAN_900: Color = color_u8!(22, 78, 99, 255);
    pub const CYAN_950: Color = color_u8!(8, 51, 68, 255);

    // SKY
    pub const SKY_50: Color = color_u8!(240, 249, 255, 255);
    pub const SKY_100: Color = color_u8!(224, 242, 254, 255);
    pub const SKY_200: Color = color_u8!(186, 230, 253, 255);
    pub const SKY_300: Color = color_u8!(125, 211, 252, 255);
    pub const SKY_400: Color = color_u8!(56, 189, 248, 255);
    pub const SKY_500: Color = color_u8!(14, 165, 233, 255);
    pub const SKY_600: Color = color_u8!(2, 132, 199, 255);
    pub const SKY_700: Color = color_u8!(3, 105, 161, 255);
    pub const SKY_800: Color = color_u8!(7, 89, 133, 255);
    pub const SKY_900: Color = color_u8!(12, 74, 110, 255);
    pub const SKY_950: Color = color_u8!(8, 47, 73, 255);

    // BLUE
    pub const BLUE_50: Color = color_u8!(239, 246, 255, 255);
    pub const BLUE_100: Color = color_u8!(219, 234, 254, 255);
    pub const BLUE_200: Color = color_u8!(191, 219, 254, 255);
    pub const BLUE_300: Color = color_u8!(147, 197, 253, 255);
    pub const BLUE_400: Color = color_u8!(96, 165, 250, 255);
    pub const BLUE_500: Color = color_u8!(59, 130, 246, 255);
    pub const BLUE_600: Color = color_u8!(37, 99, 235, 255);
    pub const BLUE_700: Color = color_u8!(29, 78, 216, 255);
    pub const BLUE_800: Color = color_u8!(30, 64, 175, 255);
    pub const BLUE_900: Color = color_u8!(30, 58, 138, 255);
    pub const BLUE_950: Color = color_u8!(23, 37, 84, 255);

    // INDIGO
    pub const INDIGO_50: Color = color_u8!(238, 242, 255, 255);
    pub const INDIGO_100: Color = color_u8!(224, 231, 255, 255);
    pub const INDIGO_200: Color = color_u8!(199, 210, 254, 255);
    pub const INDIGO_300: Color = color_u8!(165, 180, 252, 255);
    pub const INDIGO_400: Color = color_u8!(129, 140, 248, 255);
    pub const INDIGO_500: Color = color_u8!(99, 102, 241, 255);
    pub const INDIGO_600: Color = color_u8!(79, 70, 229, 255);
    pub const INDIGO_700: Color = color_u8!(67, 56, 202, 255);
    pub const INDIGO_800: Color = color_u8!(55, 48, 163, 255);
    pub const INDIGO_900: Color = color_u8!(49, 46, 129, 255);
    pub const INDIGO_950: Color = color_u8!(30, 27, 75, 255);

    // VIOLET
    pub const VIOLET_50: Color = color_u8!(245, 243, 255, 255);
    pub const VIOLET_100: Color = color_u8!(237, 233, 254, 255);
    pub const VIOLET_200: Color = color_u8!(221, 214, 254, 255);
    pub const VIOLET_300: Color = color_u8!(196, 181, 253, 255);
    pub const VIOLET_400: Color = color_u8!(167, 139, 250, 255);
    pub const VIOLET_500: Color = color_u8!(139, 92, 246, 255);
    pub const VIOLET_600: Color = color_u8!(124, 58, 237, 255);
    pub const VIOLET_700: Color = color_u8!(109, 40, 217, 255);
    pub const VIOLET_800: Color = color_u8!(91, 33, 182, 255);
    pub const VIOLET_900: Color = color_u8!(76, 29, 149, 255);
    pub const VIOLET_950: Color = color_u8!(46, 16, 101, 255);

    // PURPLE
    pub const PURPLE_50: Color = color_u8!(250, 245, 255, 255);
    pub const PURPLE_100: Color = color_u8!(243, 232, 255, 255);
    pub const PURPLE_200: Color = color_u8!(233, 213, 255, 255);
    pub const PURPLE_300: Color = color_u8!(216, 180, 254, 255);
    pub const PURPLE_400: Color = color_u8!(192, 132, 252, 255);
    pub const PURPLE_500: Color = color_u8!(168, 85, 247, 255);
    pub const PURPLE_600: Color = color_u8!(147, 51, 234, 255);
    pub const PURPLE_700: Color = color_u8!(126, 34, 206, 255);
    pub const PURPLE_800: Color = color_u8!(107, 33, 168, 255);
    pub const PURPLE_900: Color = color_u8!(88, 28, 135, 255);
    pub const PURPLE_950: Color = color_u8!(59, 7, 100, 255);

    // FUCHSIA
    pub const FUCHSIA_50: Color = color_u8!(253, 244, 255, 255);
    pub const FUCHSIA_100: Color = color_u8!(250, 232, 255, 255);
    pub const FUCHSIA_200: Color = color_u8!(245, 208, 254, 255);
    pub const FUCHSIA_300: Color = color_u8!(240, 171, 252, 255);
    pub const FUCHSIA_400: Color = color_u8!(232, 121, 249, 255);
    pub const FUCHSIA_500: Color = color_u8!(217, 70, 239, 255);
    pub const FUCHSIA_600: Color = color_u8!(192, 38, 211, 255);
    pub const FUCHSIA_700: Color = color_u8!(162, 28, 175, 255);
    pub const FUCHSIA_800: Color = color_u8!(134, 25, 143, 255);
    pub const FUCHSIA_900: Color = color_u8!(112, 26, 117, 255);
    pub const FUCHSIA_950: Color = color_u8!(74, 4, 78, 255);

    // PINK
    pub const PINK_50: Color = color_u8!(253, 242, 248, 255);
    pub const PINK_100: Color = color_u8!(252, 231, 243, 255);
    pub const PINK_200: Color = color_u8!(251, 207, 232, 255);
    pub const PINK_300: Color = color_u8!(249, 168, 212, 255);
    pub const PINK_400: Color = color_u8!(244, 114, 182, 255);
    pub const PINK_500: Color = color_u8!(236, 72, 153, 255);
    pub const PINK_600: Color = color_u8!(219, 39, 119, 255);
    pub const PINK_700: Color = color_u8!(190, 24, 93, 255);
    pub const PINK_800: Color = color_u8!(157, 23, 77, 255);
    pub const PINK_900: Color = color_u8!(131, 24, 67, 255);
    pub const PINK_950: Color = color_u8!(80, 7, 36, 255);

    // ROSE
    pub const ROSE_50: Color = color_u8!(255, 241, 242, 255);
    pub const ROSE_100: Color = color_u8!(255, 228, 230, 255);
    pub const ROSE_200: Color = color_u8!(254, 205, 211, 255);
    pub const ROSE_300: Color = color_u8!(253, 164, 175, 255);
    pub const ROSE_400: Color = color_u8!(251, 113, 133, 255);
    pub const ROSE_500: Color = color_u8!(244, 63, 98, 255);
    pub const ROSE_600: Color = color_u8!(225, 29, 72, 255);
    pub const ROSE_700: Color = color_u8!(190, 18, 60, 255);
    pub const ROSE_800: Color = color_u8!(159, 18, 57, 255);
    pub const ROSE_900: Color = color_u8!(136, 19, 55, 255);
    pub const ROSE_950: Color = color_u8!(76, 5, 25, 255);
}
